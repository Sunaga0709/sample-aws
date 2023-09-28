use tonic::{Request, Response, Status};

use crate::domain::repository::cloud_stroage::CloudStorageRepository;
use crate::domain::repository::example::ExampleRepository;
use crate::gateway::aws_s3::AwsS3;
use crate::gateway::example::ExampleRepoImpl;
use crate::gen::sample_aws_service::sample_aws_service_server::SampleAwsService;
use crate::gen::sample_aws_service::{ExampleMethodRequest, ExampleMethodResponse};
use crate::gen::sample_aws_service::{UploadImageRequest, UploadImageResponse};
use crate::usecase::example::ExampleUsecase;
use crate::usecase::image::ImageUsecase;

#[derive(Debug)]
pub struct SampleAws<ER, CSR>
where
    ER: ExampleRepository + Send + Sync,
    CSR: CloudStorageRepository + Send + Sync,
{
    image_uc: ImageUsecase<CSR>,
    example_uc: ExampleUsecase<ER>,
}

impl<ER, CSR> SampleAws<ER, CSR>
where
    ER: ExampleRepository + Send + Sync,
    CSR: CloudStorageRepository + Send + Sync,
{
    pub fn new(image_uc: ImageUsecase<CSR>, example_uc: ExampleUsecase<ER>) -> Self {
        Self {
            image_uc,
            example_uc,
        }
    }
}

#[tonic::async_trait]
impl SampleAwsService for SampleAws<ExampleRepoImpl, AwsS3> {
    async fn example_method(
        &self,
        req: Request<ExampleMethodRequest>,
    ) -> Result<Response<ExampleMethodResponse>, Status> {
        let (md, ext, param) = req.into_parts();
        dbg!(md);
        dbg!(ext);
        dbg!(param);

        match self.example_uc.create().await {
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
            .image_uc
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
