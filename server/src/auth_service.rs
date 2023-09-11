use std::str::FromStr;

use tonic::metadata::{Ascii, MetadataMap, MetadataValue};
use tonic::{Request, Response, Status};

use crate::apperror::error::AppError;
use crate::domain::repository::auth::AuthRepository;
use crate::domain::repository::user::UserRepository;
use crate::gateway::aws_cognito::AwsCognito;
use crate::gateway::user::UserRepoImpl;
use crate::gen::auth_sample_aws_service::auth_sample_aws_service_server::AuthSampleAwsService;
use crate::gen::auth_sample_aws_service::{SignUpRequest, SignUpResponse};
use crate::usecase::auth::AuthUsecase;

#[derive(Debug)]
pub struct AuthService<AR, UR>
where
    AR: AuthRepository + Send + Sync,
    UR: UserRepository + Send + Sync,
{
    auth_uc: AuthUsecase<AR, UR>,
}

impl<AR, UR> AuthService<AR, UR>
where
    AR: AuthRepository + Send + Sync,
    UR: UserRepository + Send + Sync,
{
    pub fn new(auth_uc: AuthUsecase<AR, UR>) -> Self {
        Self { auth_uc }
    }

    fn to_ascii_value(value: &str) -> Result<MetadataValue<Ascii>, AppError> {
        MetadataValue::from_str(value).map_err(|err| {
            AppError::new_internal_with_error(
                "AuthService::to_asscii_value failed to parse ascii",
                &err,
            )
        })
    }
}

#[tonic::async_trait]
impl AuthSampleAwsService for AuthService<AwsCognito, UserRepoImpl> {
    async fn sign_up(
        &self,
        req: Request<SignUpRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {
        let param: SignUpRequest = req.into_inner();

        match self
            .auth_uc
            .sign_up(
                &param.name,
                &param.email,
                param.birthday,
                &param.password,
                &param.password_confirm,
            )
            .await
        {
            Ok(token) => {
                let md_session: MetadataValue<Ascii> = match Self::to_ascii_value(&token.id_token) {
                    Ok(value) => value,
                    Err(err) => return Err(Status::from(err)),
                };
                let md_acccess: MetadataValue<Ascii> =
                    match Self::to_ascii_value(&token.access_token) {
                        Ok(value) => value,
                        Err(err) => return Err(Status::from(err)),
                    };
                let md_refresh: MetadataValue<Ascii> =
                    match Self::to_ascii_value(&token.refresh_token) {
                        Ok(value) => value,
                        Err(err) => return Err(Status::from(err)),
                    };
                let md_expires_in: MetadataValue<Ascii> = MetadataValue::from(token.expires_in);

                let mut res: Response<SignUpResponse> = Response::new(SignUpResponse {});
                let md: &mut MetadataMap = res.metadata_mut();
                md.insert("session", md_session);
                md.insert("access_token", md_acccess);
                md.insert("refresh_token", md_refresh);
                md.insert("expires_in", md_expires_in);

                Ok(res)
            }
            Err(err) => Err(Status::from(err)),
        }
    }
}
