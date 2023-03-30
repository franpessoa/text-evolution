pub mod web;
pub mod calc;

use axum::{
    Router,
    handler::HandlerWithoutStateExt, 
};
use hyper::StatusCode;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let server = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap());    
    let router = Router::new()
        .nest_service(
            "/static", 
            ServeDir::new("src")
                .not_found_service(file_not_found.into_service()
            )
        );
    
    server
        .serve(router.into_make_service())
        .await
        .unwrap();
    
}

async fn file_not_found() -> (StatusCode, &'static str){
    return (StatusCode::NOT_FOUND, "File not found")
}