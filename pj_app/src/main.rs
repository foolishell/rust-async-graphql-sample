use std::net::SocketAddr;

use axum::{Router, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use pj_core::usecase::get_product::GetProductUsecase;
use pj_infra::repository::product_repository::ProductRepository;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/product", get(get_product));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_product() -> impl IntoResponse {
    let product = GetProductUsecase {
        product_repo: Box::new(ProductRepository),
    }.execute(String::from("product_id")).await;

    (StatusCode::OK, Json(product))
}
