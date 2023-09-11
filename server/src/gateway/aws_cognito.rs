use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_cognitoidentityprovider::operation::admin_create_user::AdminCreateUserOutput;
use aws_sdk_cognitoidentityprovider::operation::admin_initiate_auth::AdminInitiateAuthOutput;
use aws_sdk_cognitoidentityprovider::types::{
    AttributeType, AuthFlowType, AuthenticationResultType, MessageActionType, UserType,
};
use aws_sdk_cognitoidentityprovider::Client;
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;

use crate::apperror::error::AppError;
use crate::domain::model::auth::EmailAndPass;
use crate::domain::model::auth::Token;
use crate::domain::repository::auth::AuthRepository;

#[derive(Debug)]
pub struct AwsCognito {
    client: Client,
    user_pool_id: String,
    app_client_id: String,
    app_client_secret: String,
}

impl AwsCognito {
    pub fn new(
        conf: &SdkConfig,
        pool_id: &str,
        app_client_id: &str,
        app_client_secret: &str,
    ) -> Self {
        Self {
            client: Client::new(conf),
            user_pool_id: pool_id.to_owned(),
            app_client_id: app_client_id.to_owned(),
            app_client_secret: app_client_secret.to_owned(),
        }
    }

    fn user_pool_id(&self) -> String {
        self.user_pool_id.clone()
    }

    fn app_client_id(&self) -> String {
        self.app_client_id.clone()
    }

    fn secret_hash(&self, user_id: &str) -> Result<String, AppError> {
        let joined_name_id: String = user_id.to_owned() + &self.app_client_id;
        let byte_name_id: &[u8] = joined_name_id.as_bytes();

        let byte_secret: &[u8] = self.app_client_secret.as_bytes();
        let mut mac = Hmac::<Sha256>::new_from_slice(byte_secret).map_err(|err| {
            AppError::new_internal_with_error("AwsCognito::secret_hash failed to init hmac", &err)
        })?;
        mac.update(byte_name_id);
        let result = mac.finalize();

        Ok(general_purpose::STANDARD.encode(result.into_bytes()))
    }

    fn build_attributes(params: Vec<(&str, &str)>) -> Vec<AttributeType> {
        let mut attributes: Vec<AttributeType> = Vec::new();

        for (key, value) in params {
            attributes.push(AttributeType::builder().name(key).value(value).build())
        }

        attributes
    }

    fn signin_parameter(params: Vec<(&str, &str)>) -> HashMap<String, String> {
        let mut signin_param: HashMap<String, String> = HashMap::new();

        for (key, value) in params {
            signin_param.insert(key.to_owned(), value.to_owned());
        }

        signin_param
    }

    fn set_token_to_model(auth_result: &AuthenticationResultType) -> Result<Token, AppError> {
        let id_token: &str = match auth_result.id_token() {
            Some(token) => token,
            None => {
                return Err(AppError::new_internal(
                    "AwsCognito::set_token_to_model id token is none",
                ))
            }
        };

        let access_token: &str = match auth_result.access_token() {
            Some(token) => token,
            None => {
                return Err(AppError::new_internal(
                    "AwsCognito::set_token_to_model access token is none",
                ))
            }
        };

        let refresh_token: &str = match auth_result.refresh_token() {
            Some(token) => token,
            None => {
                return Err(AppError::new_internal(
                    "AwsCognito::set_token_to_model refresh token is none",
                ))
            }
        };

        Ok(Token {
            id_token: id_token.to_owned(),
            access_token: access_token.to_owned(),
            refresh_token: refresh_token.to_owned(),
            expires_in: auth_result.expires_in(),
        })
    }
}

#[async_trait]
impl AuthRepository for AwsCognito {
    async fn sign_up(&self, email: &str) -> Result<String, AppError> {
        let attributes: Vec<AttributeType> = Self::build_attributes(vec![
            ("name", email),
            ("email", email),
            ("email_verified", "True"),
        ]);

        let result: AdminCreateUserOutput = self
            .client
            .admin_create_user()
            .user_pool_id(&self.user_pool_id)
            .username(email)
            .set_user_attributes(Some(attributes))
            .message_action(MessageActionType::Suppress)
            .send()
            .await
            .map_err(|err| {
                AppError::new_internal_with_error("AwsCognito::signup failed to create user", &err)
            })?;

        let result_user: &UserType = match result.user() {
            Some(user) => user,
            None => {
                Self::delete_user(self, email).await?;
                return Err(AppError::new_internal(
                    "AwsCognito::signup failed to sign up",
                ));
            }
        };

        let attributes: &[AttributeType] = match result_user.attributes() {
            Some(attr) => attr,
            None => {
                Self::delete_user(self, email).await?;
                return Err(AppError::new_internal(
                    "AwsCognito::signup failed to sign up",
                ));
            }
        };

        let mut user_id: String = String::new();
        for attribute in attributes {
            match attribute.name() {
                Some(name) => {
                    if name == "sub" && attribute.value().is_some() {
                        user_id = attribute.value().unwrap().to_owned();
                        break;
                    }
                }
                None => {
                    Self::delete_user(self, email).await?;
                    return Err(AppError::new_internal(
                        "AwsCognito::signup failed to sign up",
                    ));
                }
            }
        }

        Ok(user_id)
    }

    async fn set_password(&self, param: &EmailAndPass) -> Result<(), AppError> {
        self.client
            .admin_set_user_password()
            .user_pool_id(&self.user_pool_id)
            .username(&param.email)
            .password(&param.password)
            .permanent(true)
            .send()
            .await
            .map_err(|err| {
                AppError::new_internal_with_error(
                    "AwsCognito::set_password failed to set user password",
                    &err,
                )
            })?;

        Ok(())
    }

    async fn sign_in(&self, param: &EmailAndPass) -> Result<Token, AppError> {
        let hash: String = Self::secret_hash(self, &param.email)?;

        let signin_input: HashMap<String, String> = Self::signin_parameter(vec![
            ("USERNAME", &param.email),
            ("PASSWORD", &param.password),
            ("SECRET_HASH", &hash),
        ]);

        let signin_result: AdminInitiateAuthOutput = self
            .client
            .admin_initiate_auth()
            .auth_flow(AuthFlowType::AdminUserPasswordAuth)
            .user_pool_id(self.user_pool_id())
            .client_id(self.app_client_id())
            .set_auth_parameters(Some(signin_input))
            .send()
            .await
            .map_err(|err| {
                AppError::new_internal_with_error("AwsCognito::signin failed to sign in", &err)
            })?;

        match signin_result.authentication_result() {
            Some(result) => {
                let token: Token = Self::set_token_to_model(result)?;
                Ok(token)
            }
            None => {
                return Err(AppError::new_internal(
                    "AwsCognito::signin failed to signin",
                ))
            }
        }
    }

    async fn delete_user(&self, email: &str) -> Result<(), AppError> {
        self.client
            .admin_delete_user()
            .user_pool_id(self.user_pool_id())
            .username(email)
            .send()
            .await
            .map_err(|err| {
                AppError::new_internal_with_error(
                    "AwsCognito::delete_user failed to delete user",
                    &err,
                )
            })?;

        Ok(())
    }
}
