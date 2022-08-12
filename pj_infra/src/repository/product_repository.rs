use async_trait::async_trait;
use pj_core::model::product::Product;
use pj_core::repository_trait::ProductRepositoryTrait;

pub struct ProductRepository;

#[async_trait]
impl ProductRepositoryTrait for ProductRepository {
    async fn get_product(&self, product_id: String) -> Product {
        Product {
            product_id: String::from("product_id"),
            name: String::from("name"),
            price: 1,
            inventory_quantity: 3,
        }
    }
}
