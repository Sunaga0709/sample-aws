use tonic::{Request, Response, Status};

use crate::domain::repository::cloud_stroage::CloudStorageRepository;
use crate::domain::repository::example::ExampleRepository;
use crate::gateway::aws_s3::AwsS3;
use crate::gateway::example::ExampleRepoImpl;
use crate::gen::sample_aws_v1::sample_aws_service_server::SampleAwsService;
use crate::gen::sample_aws_v1::{TestMethodRequest, TestMethodResponse};
use crate::gen::sample_aws_v1::{UploadImageRequest, UploadImageResponse};
use crate::usecase::sample_aws::SampleAwsUsecase;

#[derive(Debug)]
pub struct Aws<ER, CS>
where
    ER: ExampleRepository + Send + Sync,
    CS: CloudStorageRepository + Send + Sync,
{
    uc: SampleAwsUsecase<ER, CS>,
}

impl<ER, CS> Aws<ER, CS>
where
    ER: ExampleRepository + Send + Sync,
    CS: CloudStorageRepository + Send + Sync,
{
    pub fn new(uc: SampleAwsUsecase<ER, CS>) -> Self {
        Self { uc }
    }
}

#[tonic::async_trait]
impl SampleAwsService for Aws<ExampleRepoImpl, AwsS3> {
    async fn test_method(
        &self,
        req: Request<TestMethodRequest>,
    ) -> Result<Response<TestMethodResponse>, Status> {
        // dbg!(req);
        let md: &tonic::metadata::MetadataMap = req.metadata();
        let header = md.clone().into_headers();
        dbg!(&header["content-type"]);

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

    async fn upload_image(
        &self,
        req: Request<UploadImageRequest>,
    ) -> Result<Response<UploadImageResponse>, Status> {
        // let raw_str: &str = "test test";
        // let encode_str = general_purpose::STANDARD.encode(raw_str);
        // dbg!(&encode_str);
        // let decode_str = engine::GeneralPurpose::new(
        //     &alphabet::URL_SAFE,
        //     general_purpose::NO_PAD)
        //     .decode(&encode_str).unwrap();
        // dbg!(String::from_utf8(decode_str));

        let param: UploadImageRequest = req.into_inner();

        match self.uc.upload_image(&param.name, &param.image_data).await {
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
