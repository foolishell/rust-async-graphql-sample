use crate::model::product::Product;
use async_trait::async_trait;

#[async_trait]
pub trait ProductRepositoryTrait: Send + Sync + 'static {
    type Err;

    async fn find_product(&self, product_id: String) -> anyhow::Result<Option<Product>, Self::Err>;
}
