use axum::routing::{get, post};
use axum::Router;
use http::Method;
use sqlx::{Any, Pool};
use tower_http::cors::CorsLayer;

use crate::config::Config;
use crate::dependency_injection;
use crate::middleware::log::LogLayer;
use crate::web::api::auth::sign_up;
use crate::web::api::example::create_example;
use crate::web::api::example::get_by_id_example;
use crate::web::api::example::get_example;

use crate::web::api::image::upload_image;

const VERSION1: &str = "/v1";

pub async fn new_router(conf: &Config) -> Router {
    let pool: Pool<Any> = conf.init_db().await;

    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(["http://localhost:3000"
            .parse()
            .expect("route::new_router failed to parse origin url")])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let example_route: Router = Router::new()
        .route("/example", get(get_example).post(create_example))
        .route("/example/:id", get(get_by_id_example))
        .with_state(dependency_injection::di_example(pool.clone()));

    let image_route: Router = Router::new()
        .route("/image", post(upload_image))
        .with_state(dependency_injection::di_image(
            &conf.aws_config().await,
            conf.s3_bucket_name(),
        ));

    let auth_route: Router =
        Router::new()
            .route("/signup", post(sign_up))
            .with_state(dependency_injection::di_auth(
                &conf.aws_config().await,
                conf.user_pool_id(),
                conf.app_client_id(),
                conf.app_client_secret(),
                pool.clone(),
            ));

    Router::new()
        .nest(VERSION1, example_route)
        .nest(VERSION1, image_route)
        .nest(VERSION1, auth_route)
        .layer(cors)
        .layer(LogLayer)
}
