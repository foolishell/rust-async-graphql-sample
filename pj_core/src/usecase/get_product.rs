use crate::repository_trait::ProductRepositoryTrait;
use crate::model::product::Product;


pub struct GetProductUsecase {
    pub product_repo: Box<dyn ProductRepositoryTrait>,
}

impl GetProductUsecase {
    pub async fn execute(&self, product_id: String) -> Product {
        self.product_repo.get_product(product_id).await
    }
}
