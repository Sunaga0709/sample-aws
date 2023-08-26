use tonic::{Request, Response, Status};

use crate::domain::repository::example::ExampleRepository;
use crate::gateway::example::ExampleRepoImpl;
use crate::gen::aws_v1::aws_service_server::AwsService;
use crate::gen::aws_v1::{TestMethodRequest, TestMethodResponse};
use crate::usecase::aws::AwsUsecase;

#[derive(Debug)]
pub struct Aws<ER>
where
    ER: ExampleRepository + Send + Sync,
{
    uc: AwsUsecase<ER>,
}

impl<ER> Aws<ER>
where
    ER: ExampleRepository + Send + Sync,
{
    pub fn new(uc: AwsUsecase<ER>) -> Self {
        Self { uc }
    }
}

#[tonic::async_trait]
impl AwsService for Aws<ExampleRepoImpl> {
    async fn test_method(
        &self,
        req: Request<TestMethodRequest>,
    ) -> Result<Response<TestMethodResponse>, Status> {
        dbg!(req);

        match self.uc.test().await {
            Ok(_) => {
                let res: Response<TestMethodResponse> = Response::new(TestMethodResponse {});
                dbg!(&res);
                Ok(res)
            }
            Err(err) => {
                let err_res: Status = Status::from(err);
                dbg!(&err_res);
                Err(err_res)
            }
        }
    }
}
