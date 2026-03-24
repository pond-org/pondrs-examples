use serde::{Deserialize, Serialize};

use linfa_linear::FittedLinearRegression;
use pondrs::datasets::{
    JsonDataset, MemoryDataset, PlotlyDataset, PolarsCsvDataset, PolarsExcelDataset,
    PolarsParquetDataset,
};

#[derive(Serialize, Deserialize)]
pub struct Catalog {
    pub companies: PolarsCsvDataset,
    pub reviews: PolarsCsvDataset,
    pub shuttles: PolarsExcelDataset,
    pub preprocessed_companies: PolarsParquetDataset,
    pub preprocessed_shuttles: PolarsParquetDataset,
    pub model_input_table: PolarsParquetDataset,

    pub regressor: MemoryDataset<FittedLinearRegression<f64>>,
    pub shuttle_passenger_capacity_plot_exp: PlotlyDataset,
    pub shuttle_passenger_capacity_plot_go: JsonDataset,
    pub dummy_confusion_matrix: PlotlyDataset,
    // type: matplotlib.MatplotlibDataset
    pub X_train: PolarsParquetDataset,
    pub y_train: PolarsParquetDataset,
    pub X_test: PolarsParquetDataset,
    pub y_test: PolarsParquetDataset,
    pub metrics: PolarsParquetDataset,
}
