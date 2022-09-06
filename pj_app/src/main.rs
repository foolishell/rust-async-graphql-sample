use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Path;

use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{extract::Extension, Json, Router};
use infra::repository::product_repository::ProductRepository;
use pj_core::model::product::Product;
use pj_core::usecase::get_product::GetProductUsecase;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use std::net::SocketAddr;
use std::sync::Arc;

use crate::error::AppError;
use crate::presentation::graphql::schema::{schema_builder, AppSchema};

mod di;
mod error;
mod infra;
mod presentation;

struct State {
    pool: PgPool,
}

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/pj_db")
        .await?;

    let query_usecase = di::inject(pool.clone());

    let schema = schema_builder().data(query_usecase).finish();

    let app = Router::new()
        .route("/product/:id", get(get_product))
        .route("/graphql", get(graphql_playground))
        .route("/graphql", post(graphql_handler))
        .layer(Extension(Arc::new(State { pool })))
        .layer(Extension(schema));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_product(
    Path(product_id): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> anyhow::Result<Json<Option<Product>>, AppError> {
    let repo = ProductRepository::new(state.pool.clone());
    let product = GetProductUsecase::new(repo).execute(product_id).await?;

    Ok(Json(product))
}
