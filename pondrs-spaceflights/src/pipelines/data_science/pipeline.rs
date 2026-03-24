use pondrs::{Node, Pipeline, PondError, RunnableStep};

use crate::{Catalog, Params};

use super::nodes::{evaluate_model, split_data, train_model};

pub fn data_science_pipeline<'a>(
    cat: &'a Catalog,
    params: &'a Params,
) -> impl RunnableStep<PondError> + 'a {
    Pipeline {
        name: "data_science",
        input: (&cat.model_input_table, &params.model_options),
        output: (),
        steps: (
            Node {
                func: split_data,
                input: (&cat.model_input_table, &params.model_options),
                output: (&cat.X_train, &cat.X_test, &cat.y_train, &cat.y_test),
                name: "split_data_node",
            },
            Node {
                func: train_model,
                input: (&cat.X_train, &cat.y_train),
                output: (&cat.regressor,),
                name: "train_model_node",
            },
            Node {
                func: evaluate_model,
                input: (&cat.regressor, &cat.X_test, &cat.y_test),
                output: (&cat.metrics,),
                name: "evaluate_model_node",
            },
        ),
    }
}
