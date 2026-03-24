use pondrs::{Node, Pipeline, PondError, RunnableStep};

use crate::catalog::Catalog;

use super::nodes::{create_model_input_table, preprocess_companies, preprocess_shuttles};

pub fn data_processing_pipeline<'a>(
    cat: &'a Catalog,
    _params: &'a (),
) -> impl RunnableStep<PondError> + 'a {
    Pipeline {
        name: "data_processing",
        input: (&cat.companies, &cat.shuttles, &cat.reviews),
        output: (&cat.model_input_table, &cat.preprocessed_shuttles),
        steps: (
            Node {
                func: preprocess_companies,
                input: (&cat.companies,),
                output: (&cat.preprocessed_companies,),
                name: "preprocess_companies_node",
            },
            Node {
                func: preprocess_shuttles,
                input: (&cat.shuttles,),
                output: (&cat.preprocessed_shuttles,),
                name: "preprocess_shuttles_node",
            },
            Node {
                func: create_model_input_table,
                input: (
                    &cat.preprocessed_shuttles,
                    &cat.preprocessed_companies,
                    &cat.reviews,
                ),
                output: (&cat.model_input_table,),
                name: "create_model_input_table_node",
            },
        ),
    }
}
