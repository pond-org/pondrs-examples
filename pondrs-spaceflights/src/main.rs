use pondrs::hooks::LoggingHook;
use pondrs::viz::VizHook;

use pondrs::{PondError, Steps};

mod catalog;
mod params;
mod pipelines;

use catalog::Catalog;
use params::Params;

// #[derive(Serialize, Deserialize)]
// pub struct Params;
use pipelines::data_processing::pipeline::data_processing_pipeline;
use pipelines::data_science::pipeline::data_science_pipeline;
use pipelines::reporting::pipeline::reporting_pipeline;

fn pipeline<'a>(cat: &'a Catalog, params: &'a Params) -> impl Steps<PondError> + 'a {
    (
        data_processing_pipeline(cat, &()),
        data_science_pipeline(cat, params),
        reporting_pipeline(cat, &()),
    )
}

fn main() -> Result<(), PondError> {
    pondrs::app::App::from_args(std::env::args_os())?
        .with_hooks((
            LoggingHook::new(),
            VizHook::new("http://localhost:8080".to_string()),
        ))
        .with_args(std::env::args_os())?
        .dispatch(pipeline)
}
