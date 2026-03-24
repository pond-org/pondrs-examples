use pondrs::{Node, Pipeline, PondError, RunnableStep};

use crate::Catalog;

use super::nodes::{
    compare_passenger_capacity_exp, compare_passenger_capacity_go, create_confusion_matrix,
};

pub fn reporting_pipeline<'a>(
    cat: &'a Catalog,
    _params: &'a (),
) -> impl RunnableStep<PondError> + 'a {
    Pipeline {
        name: "reporting",
        input: (&cat.preprocessed_shuttles, &cat.companies),
        output: (),
        steps: (
            Node {
                func: compare_passenger_capacity_exp,
                input: (&cat.preprocessed_shuttles,),
                output: (&cat.shuttle_passenger_capacity_plot_exp,),
                name: "compare_passenger_exp_node",
            },
            Node {
                func: compare_passenger_capacity_go,
                input: (&cat.preprocessed_shuttles,),
                output: (&cat.shuttle_passenger_capacity_plot_go,),
                name: "compare_passenger_go_node",
            },
            Node {
                func: create_confusion_matrix,
                input: (&cat.companies,),
                output: (&cat.dummy_confusion_matrix,),
                name: "confusion_matrix_node",
            },
        ),
    }
}
