use plotly::{Bar, HeatMap, Layout, Plot};
use polars::prelude::*;
use serde_json::Value;

// This function uses plotly.express
pub fn compare_passenger_capacity_exp(
    preprocessed_shuttles: DataFrame,
) -> Result<(Plot,), PolarsError> {
    let data_frame = preprocessed_shuttles
        .lazy()
        .group_by([col("shuttle_type")])
        .agg([col("passenger_capacity").mean()])
        .collect()?;

    let mut plot = Plot::new();
    plot.add_trace(Bar::new(
        data_frame.column("shuttle_type")?.as_materialized_series().str()?.into_no_null_iter().map(String::from).collect::<Vec<_>>(),
        data_frame.column("passenger_capacity")?.as_materialized_series().f64()?.into_no_null_iter().collect::<Vec<_>>(),
    ));

    Ok((plot,))
}

// This function uses plotly.graph_objects
pub fn compare_passenger_capacity_go(
    preprocessed_shuttles: DataFrame,
) -> Result<(Value,), PolarsError> {
    let data_frame = preprocessed_shuttles
        .lazy()
        .group_by([col("shuttle_type")])
        .agg([col("passenger_capacity").mean()])
        .collect()?;

    let mut fig = Plot::new();
    fig.add_trace(Bar::new(
        data_frame.column("shuttle_type")?.as_materialized_series().str()?.into_no_null_iter().map(String::from).collect::<Vec<_>>(),
        data_frame.column("passenger_capacity")?.as_materialized_series().f64()?.into_no_null_iter().collect::<Vec<_>>(),
    ));

    let fig: Value = serde_json::from_str(&fig.to_json())
        .map_err(|e| PolarsError::ComputeError(format!("{e}").into()))?;

    Ok((fig,))
}

pub fn create_confusion_matrix(_companies: DataFrame) -> Result<(Plot,), PolarsError> {
    let actuals = vec![0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1];
    let predicted = vec![1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1];

    let data = df! {
        "y_Actual" => &actuals,
        "y_Predicted" => &predicted,
    }?;

    let confusion_matrix = data
        .lazy()
        .group_by([col("y_Actual"), col("y_Predicted")])
        .agg([col("y_Predicted").count().alias("count")])
        .sort(["y_Actual", "y_Predicted"], Default::default())
        .collect()?;

    // Pivot into matrix form: rows=Actual, cols=Predicted
    let labels = vec![0, 1];
    let mut z: Vec<Vec<i64>> = Vec::new();
    for actual in &labels {
        let mut row = Vec::new();
        for pred in &labels {
            let count = confusion_matrix
                .clone()
                .lazy()
                .filter(col("y_Actual").eq(lit(*actual)).and(col("y_Predicted").eq(lit(*pred))))
                .collect()?
                .column("count")
                .ok()
                .and_then(|c| c.as_materialized_series().u32().ok()?.get(0).map(|v| v as i64))
                .unwrap_or(0);
            row.push(count);
        }
        z.push(row);
    }

    let trace = HeatMap::new(labels.clone(), labels, z);

    let layout = Layout::new().title("Confusion Matrix");

    let mut fig = Plot::new();
    fig.add_trace(trace);
    fig.set_layout(layout);

    Ok((fig,))
}
