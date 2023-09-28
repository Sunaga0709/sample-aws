use axum::Router;
use axum::routing::{get, post};
use sqlx::{Any, Pool};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_http::trace::{
    DefaultOnRequest,
    DefaultOnResponse,
    DefaultOnFailure,
};
use tracing::Level;
use http::Method;

use crate::dependency_injection;
use crate::middleware::log::LogLayer;
use crate::web::api::example::create_example;
use crate::web::api::example::get_example;
use crate::web::api::example::get_by_id_example;

const VERSION1: &str = "/v1";

pub fn new_router(pool: Pool<Any>) -> Router {
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(
            [
                "http://localhost:3000".parse().expect("route::new_router failed to parse origin url"),
            ]
        )
        .allow_methods(
            [Method::GET, Method::POST, Method::PUT, Method::DELETE]
        );

    let v1_route: Router = Router::new()
        .route("/example", get(get_example).post(create_example))
        .route("/example/:id", get(get_by_id_example))
        .with_state(dependency_injection::di_example(pool.clone()));

    Router::new()
        .nest(VERSION1, v1_route)
        .layer(cors)
        .layer(LogLayer)
}