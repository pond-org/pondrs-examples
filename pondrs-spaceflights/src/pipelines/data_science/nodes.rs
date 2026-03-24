use linfa::prelude::*;
use linfa_linear::{FittedLinearRegression, LinearRegression};
use log::info;
use ndarray::{Array1, Array2};
use polars::prelude::*;

use crate::params::ModelOptions;

fn df_to_array2(df: &DataFrame) -> Result<Array2<f64>, PolarsError> {
    let nrows = df.height();
    let ncols = df.width();
    let mut data = Vec::with_capacity(nrows * ncols);
    for col in df.columns() {
        let s = col.as_materialized_series().cast(&DataType::Float64)?;
        let ca = s.f64()?;
        data.extend(ca.into_no_null_iter());
    }
    // Data is column-major, transpose to row-major
    Array2::from_shape_vec((ncols, nrows), data)
        .map(|a| a.t().to_owned())
        .map_err(|e| PolarsError::ComputeError(format!("{e}").into()))
}

fn series_to_array1(s: &Series) -> Result<Array1<f64>, PolarsError> {
    let ca = s.f64()?;
    Ok(Array1::from_vec(ca.into_no_null_iter().collect()))
}

pub fn split_data(
    data: DataFrame,
    parameters: ModelOptions,
) -> Result<(DataFrame, DataFrame, DataFrame, DataFrame), PolarsError> {
    // Shuffle rows using random_state as seed (matches sklearn's train_test_split behaviour)
    let data = data.sample_n_literal(
        data.height(),
        false,
        true,
        Some(parameters.random_state as u64),
    )?;

    let x = data.select(&parameters.features)?;
    let y = data.select(["price"])?;

    let n = data.height();
    let test_n = (n as f32 * parameters.test_size) as usize;
    let train_n = n - test_n;

    let x_train = x.slice(0, train_n);
    let x_test = x.slice(train_n as i64, test_n);
    let y_train = y.slice(0, train_n);
    let y_test = y.slice(train_n as i64, test_n);

    Ok((x_train, x_test, y_train, y_test))
}

pub fn train_model(
    x_train: DataFrame,
    y_train: DataFrame,
) -> Result<(FittedLinearRegression<f64>,), PolarsError> {
    let x = df_to_array2(&x_train)?;
    let y = series_to_array1(y_train.column("price")?.as_materialized_series())?;

    let dataset = Dataset::new(x, y);
    let regressor = LinearRegression::default()
        .fit(&dataset)
        .map_err(|e| PolarsError::ComputeError(format!("linfa error: {e}").into()))?;

    Ok((regressor,))
}

pub fn evaluate_model(
    regressor: FittedLinearRegression<f64>,
    x_test: DataFrame,
    y_test: DataFrame,
) -> Result<(DataFrame,), PolarsError> {
    let x = df_to_array2(&x_test)?;
    let y_true = series_to_array1(y_test.column("price")?.as_materialized_series())?;

    let y_pred = regressor.predict(&x);

    // R² score
    let y_mean = y_true.mean().unwrap_or(0.0);
    let ss_res: f64 = y_true
        .iter()
        .zip(y_pred.iter())
        .map(|(t, p)| (t - p).powi(2))
        .sum();
    let ss_tot: f64 = y_true.iter().map(|t| (t - y_mean).powi(2)).sum();
    let score = 1.0 - ss_res / ss_tot;

    // MAE
    let mae: f64 = y_true
        .iter()
        .zip(y_pred.iter())
        .map(|(t, p)| (t - p).abs())
        .sum::<f64>()
        / y_true.len() as f64;

    // Max error
    let me: f64 = y_true
        .iter()
        .zip(y_pred.iter())
        .map(|(t, p)| (t - p).abs())
        .fold(f64::NEG_INFINITY, f64::max);

    info!("Model has a coefficient R^2 of {:.3} on test data.", score);

    let metrics = df! {
        "r2_score" => &[score],
        "mae" => &[mae],
        "max_error" => &[me],
    }?;

    Ok((metrics,))
}
