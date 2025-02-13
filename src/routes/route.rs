

use axum::{extract::Request, middleware::{self, Next}, response::{IntoResponse, Response}, Router};
use http::StatusCode;


use crate::{enums::response_enum::{ResponseErrorMessage, VerifiedToken}, utils::token::verified_token};

use super::{account_route::account_routes, contractors_route::contractors_route, project_interface_route::project_interface_route, project_takers_route::project_takers_route};

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
        .nest("/contractors", contractors_route())
}