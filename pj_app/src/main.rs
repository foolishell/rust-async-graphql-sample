use axum::extract::Path;


use axum::routing::get;
use axum::{Extension, Json, Router};
use infra::repository::product_repository::ProductRepository;
use pj_core::model::product::Product;
use pj_core::usecase::get_product::GetProductUsecase;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;


use std::net::SocketAddr;
use std::sync::Arc;

use crate::error::AppError;

mod infra;
mod error;

struct State {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/pj_db")
        .await?;

    let app = Router::new()
        .route("/product/:id", get(get_product))
        .layer(Extension(Arc::new(State { pool: pool })));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_product(
    Path(product_id): Path<String>,
    Extension(state): Extension<Arc<State>>
) -> anyhow::Result<Json<Option<Product>>, AppError> {
    let repo = ProductRepository::new(state.pool.clone());
    let product = GetProductUsecase::new(repo)
        .execute(product_id)
        .await?;

    Ok(Json(product))
}
