use derive_more::Constructor;

use crate::model::product::Product;
use crate::repository_trait::ProductRepositoryTrait;

#[derive(Constructor)]
pub struct GetProductUsecase<PR: ProductRepositoryTrait> {
    product_repo: PR,
}

impl<PR: ProductRepositoryTrait> GetProductUsecase<PR> {
    pub async fn execute(&self, product_id: String) -> anyhow::Result<Option<Product>, PR::Err> {
        Ok(self.product_repo.find_product(product_id).await?)
    }
}
