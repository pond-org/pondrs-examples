use polars::prelude::*;

fn _is_true(x: &Series) -> Result<Series, PolarsError> {
    Ok(x.equal("t")?.into_series())
}

fn _parse_percentage(x: &Series) -> Result<Series, PolarsError> {
    let x = x.str()?.replace("%", "")?;
    let x = x.into_series().cast(&DataType::Float64)? / 100;
    Ok(x)
}

fn _parse_money(x: &Series) -> Result<Series, PolarsError> {
    let x = x.str()?.replace("\\$", "")?;
    let x = x.replace(",", "")?;
    let x = x.into_series().cast(&DataType::Float64)?;
    Ok(x)
}

pub fn preprocess_companies(companies: DataFrame) -> Result<(DataFrame,), PolarsError> {
    let mut companies = companies;
    companies.replace("iata_approved", _is_true(companies.column("iata_approved")?.as_materialized_series())?.into())?;
    companies.replace("company_rating", _parse_percentage(companies.column("company_rating")?.as_materialized_series())?.into())?;
    Ok((companies,))
}

pub fn preprocess_shuttles(shuttles: DataFrame) -> Result<(DataFrame,), PolarsError> {
    let mut shuttles = shuttles;
    shuttles.replace("d_check_complete", _is_true(shuttles.column("d_check_complete")?.as_materialized_series())?.into())?;
    shuttles.replace("moon_clearance_complete", _is_true(shuttles.column("moon_clearance_complete")?.as_materialized_series())?.into())?;
    shuttles.replace("price", _parse_money(shuttles.column("price")?.as_materialized_series())?.into())?;
    Ok((shuttles,))
}

pub fn create_model_input_table(
    shuttles: DataFrame,
    companies: DataFrame,
    reviews: DataFrame,
) -> Result<(DataFrame,), PolarsError> {
    let mut rated_shuttles = shuttles.join(
        &reviews, ["id"], ["shuttle_id"], JoinArgs::new(JoinType::Inner), None,
    )?;
    rated_shuttles = rated_shuttles.drop("id")?;
    let model_input_table = rated_shuttles.join(
        &companies, ["company_id"], ["id"], JoinArgs::new(JoinType::Inner), None,
    )?;
    let model_input_table = model_input_table.drop_nulls::<String>(None)?;
    Ok((model_input_table,))
}
