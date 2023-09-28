use sqlx::{Any, Pool};

use crate::gateway::example::ExampleRepoImpl;
use crate::usecase::example::ExampleUsecase;

pub fn di_example(pool: Pool<Any>) -> ExampleUsecase<ExampleRepoImpl> {
    ExampleUsecase::new(
        ExampleRepoImpl::new(),
        pool,
    )
}
