

use async_trait::async_trait;
use crate::model::product::Product;


#[async_trait]
pub trait ProductRepositoryTrait: Send + Sync + 'static {
    type Err;

    async fn find_product(&self, product_id: String) -> anyhow::Result<Option<Product>, Self::Err>;
}
