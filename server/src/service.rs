use tonic::{Request, Response, Status};

use crate::domain::repository::cloud_stroage::CloudStorageRepository;
use crate::domain::repository::example::ExampleRepository;
use crate::gateway::aws_s3::AwsS3;
use crate::gateway::example::ExampleRepoImpl;
use crate::gen::sample_aws_v1::sample_aws_service_server::SampleAwsService;
use crate::gen::sample_aws_v1::{ExampleMethodRequest, ExampleMethodResponse};
use crate::gen::sample_aws_v1::{UploadImageRequest, UploadImageResponse};
use crate::usecase::aws::AwsUsecase;
use crate::usecase::example::ExampleUsecase;

#[derive(Debug)]
pub struct Aws<ER, CSR>
where
    ER: ExampleRepository + Send + Sync,
    CSR: CloudStorageRepository + Send + Sync,
{
    aws_usecase: AwsUsecase<CSR>,
    example_usecase: ExampleUsecase<ER>,
}

impl<ER, CSR> Aws<ER, CSR>
where
    ER: ExampleRepository + Send + Sync,
    CSR: CloudStorageRepository + Send + Sync,
{
    pub fn new(aws_usecase: AwsUsecase<CSR>, example_usecase: ExampleUsecase<ER>) -> Self {
        Self {
            aws_usecase,
            example_usecase,
        }
    }
}

#[tonic::async_trait]
impl SampleAwsService for Aws<ExampleRepoImpl, AwsS3> {
    async fn example_method(
        &self,
        req: Request<ExampleMethodRequest>,
    ) -> Result<Response<ExampleMethodResponse>, Status> {
        // dbg!(req);
        let md: &tonic::metadata::MetadataMap = req.metadata();
        let header = md.clone().into_headers();
        dbg!(&header["content-type"]);

        match self.example_usecase.example().await {
            Ok(_) => {
                let res: Response<ExampleMethodResponse> = Response::new(ExampleMethodResponse {});
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

    async fn upload_image(
        &self,
        req: Request<UploadImageRequest>,
    ) -> Result<Response<UploadImageResponse>, Status> {
        let param: UploadImageRequest = req.into_inner();

        match self
            .aws_usecase
            .upload_image(&param.name, &param.image_data)
            .await
        {
            Ok(_) => {
                let res: Response<UploadImageResponse> = Response::new(UploadImageResponse {
                    message: String::from("success"),
                });
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
