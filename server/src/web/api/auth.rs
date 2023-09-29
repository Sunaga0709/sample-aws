use axum::extract::State;
use axum::http::header::{HeaderValue, SET_COOKIE};
use axum::response::{self, IntoResponse, Response as AxumResponse};
use axum::Json;
use std::collections::HashMap;

use crate::apperror::error::AppError;
use crate::domain::model::auth::Token;
use crate::gateway::aws_cognito::AwsCognito;
use crate::gateway::user::UserRepoImpl;
use crate::usecase::auth::AuthUsecase;
use crate::web::header::{header_values, ACCESS_TOKEN_KEY, REFRESH_TOKEN_KEY, SESSION_KEY};
use crate::web::schema::auth::{SignUpRequest, SignUpResponse};

type AuthUsecaseState = State<AuthUsecase<AwsCognito, UserRepoImpl>>;

pub async fn sign_up(
    State(uc): AuthUsecaseState,
    Json(param): Json<SignUpRequest>,
) -> Result<impl IntoResponse, AppError> {
    let token: Token = uc
        .sign_up(
            &param.name,
            &param.email,
            param.birthday,
            &param.password,
            &param.password_confirm,
        )
        .await?;

    let mut cookie_value: HashMap<&str, &str> = HashMap::new();
    cookie_value.insert(SESSION_KEY, &token.id_token);
    cookie_value.insert(ACCESS_TOKEN_KEY, &token.access_token);
    cookie_value.insert(REFRESH_TOKEN_KEY, &token.refresh_token);

    let header_values: Vec<HeaderValue> = header_values(cookie_value)?;

    let mut res: AxumResponse = response::Json(SignUpResponse {}).into_response();
    for v in header_values {
        res.headers_mut().append(SET_COOKIE, v);
    }

    Ok(res)

    // todo!();
}
