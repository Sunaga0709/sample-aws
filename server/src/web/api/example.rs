use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;

use crate::apperror::error::AppError;
use crate::gateway::example::ExampleRepoImpl;
use crate::usecase::example::ExampleUsecase;
use crate::web::schema::example::DetailResponse;
use crate::web::schema::example::GetResponse;
use crate::web::schema::example::{CreateRequest, CreateResponse};

type UcState = State<ExampleUsecase<ExampleRepoImpl>>;

/// Get examples
///
/// example document
#[utoipa::path(
    get,
    path = "/v1/example",
    responses(
        (status = 200, description = "テストデータの一覧取得に成功", body = GetResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse),
    )
)]
pub async fn get_example(State(uc): UcState) -> Result<impl IntoResponse, AppError> {
    match uc.get().await {
        Ok(result) => {
            let examples: Vec<DetailResponse> = result
                .iter()
                .map(|value| DetailResponse {
                    example_id: value.example_id.clone(),
                    created_at: value.created_at,
                })
                .collect();

            Ok(axum::Json(GetResponse { examples }))
        }
        Err(err) => Err(err),
    }
}

pub async fn get_by_id_example(
    State(uc): UcState,
    Path(example_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    match uc.get_by_id(&example_id).await {
        Ok(result) => Ok(axum::Json(DetailResponse {
            example_id: result.example_id.clone(),
            created_at: result.created_at,
        })),
        Err(err) => Err(err),
    }
}

pub async fn create_example(
    State(uc): UcState,
    Json(_): Json<CreateRequest>,
) -> Result<impl IntoResponse, AppError> {
    match uc.create().await {
        Ok(_) => Ok(axum::Json(CreateResponse {})),
        Err(err) => Err(err),
    }
}
