use pondrs::datasets::Param;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ModelOptions {
    pub test_size: f32,
    pub random_state: i32,
    pub features: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    pub model_options: Param<ModelOptions>,
}
