use async_trait::async_trait;

use crate::model::product::Product;

#[async_trait]
pub trait ProductRepositoryTrait: Send + Sync + 'static {
    async fn get_product(&self, product_id: String) -> Product;
}
