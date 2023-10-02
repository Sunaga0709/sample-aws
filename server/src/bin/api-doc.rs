use std::fs::write;
use std::path::Path;
use utoipa::openapi::OpenApi as OpenApiDoc;
use utoipa::OpenApi;

use server::web::api::example as example_api;
use server::web::schema::error_response::ErrorResponse;
use server::web::schema::example as example_schema;

#[derive(OpenApi)]
#[openapi(
    paths(example_api::get_example),
    components(schemas(ErrorResponse, example_schema::GetResponse,))
)]
struct ApiDoc {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc: OpenApiDoc = ApiDoc::openapi();
    doc.info.title = String::from("sample-aws-api");
    doc.info.version = String::from("1.0.0");

    let doc_yaml: String = doc.to_yaml()?;

    let output_path: &Path = Path::new("openapi.yml");
    write(output_path, doc_yaml)?;

    log::info!("created openapi.yml 🎉");

    Ok(())
}
