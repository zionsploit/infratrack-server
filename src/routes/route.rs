


use std::{fs::File, io::Read};

use axum::{extract::Request, middleware::{self, Next}, response::{IntoResponse, Response}, routing::{get, post}, Router};
use http::{header, StatusCode};
use infer::Infer;


use crate::{enums::response_enum::{ResponseErrorMessage, VerifiedToken}, services::storage_service::storage, utils::token::verified_token};

use super::{account_route::account_routes, contractors_route::contractors_route, project_interface_route::project_interface_route, project_route::project_route, project_takers_route::project_takers_route};

// Routes middleware for Token Verification
async fn verified_token_middleware (req: Request, next: Next) -> Result<Response, impl IntoResponse> {
    match req.headers().get("Authorization") {
        Some(result) => {

            let result = result.to_str().unwrap().to_owned();

            match verified_token(&result) {
                VerifiedToken::TokenIsValid => Ok(next.run(req).await),
                VerifiedToken::TokenIsNotValid => Err((StatusCode::FORBIDDEN, format!("{:?}", ResponseErrorMessage::TokenIsNotValid)))
            }
        }
        None => Err((StatusCode::FORBIDDEN, format!("{:?}", ResponseErrorMessage::AuthorizationRequired))),
    }
}

// API Routes
pub fn api_routes () -> Router {
    let route_middleware = middleware::from_fn(verified_token_middleware);

    Router::new()
        .nest("/account", account_routes() )
        .nest("/project-interface", project_interface_route()
            .route_layer(route_middleware.clone())
        )
        .nest("/project-takers", project_takers_route()
            .route_layer(route_middleware.clone())
        )
        .nest("/contractors", contractors_route()
            .route_layer(route_middleware.clone()))
        .nest("/project", project_route()
            .route_layer(route_middleware.clone()))
        .route("/upload-assets", post(storage ))
        .route("/image", get(serve_image))
}

async fn serve_image() -> impl IntoResponse {

    let file_path = "Storage/images/2c9c85f4ed5d44578fc92c3d9b7eb8f8.jpg";

    let mut file = File::open(file_path).unwrap();

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    let infer = Infer::new();

    let mime_type = match infer.get(&buffer) {
        Some(info) => info.mime_type(),
        None => return (StatusCode::UNSUPPORTED_MEDIA_TYPE, "Unsupported file type").into_response(),
    };
    ([(header::CONTENT_TYPE, mime_type.to_string())], buffer).into_response()
}