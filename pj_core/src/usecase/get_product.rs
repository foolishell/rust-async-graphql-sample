
use crate::model::product::Product;
use crate::repository_trait::ProductRepositoryTrait;

pub struct GetProductUsecase<PR: ProductRepositoryTrait> {
    product_repo: PR,
}

impl<PR: ProductRepositoryTrait> GetProductUsecase<PR> {
    pub fn new(product_repo: PR) -> Self {
        Self { product_repo }
    }

    pub async fn execute(&self, product_id: String) -> anyhow::Result<Option<Product>, PR::Err> {
        Ok(self.product_repo.find_product(product_id).await?)
    }
}
