




use async_trait::async_trait;

use pj_core::model::product::Product;
use pj_core::repository_trait::{ProductRepositoryTrait};
use sqlx::PgPool;
use sqlx::types::Uuid;

use crate::error::AppError;


pub(crate) struct ProductRepository {
    pool: PgPool
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }
}

#[async_trait]
impl ProductRepositoryTrait for ProductRepository {
    type Err = AppError;
    
    async fn find_product(&self, product_id: String) -> Result<Option<Product>, AppError> { 
        let record_product = sqlx::query!(
            "SELECT * FROM product_rsc WHERE product_id = $1",
            Uuid::parse_str(product_id.as_str())?
        ).fetch_optional(&self.pool).await?;
        
        if let Some(product) = record_product {
            let record_product_inventory = sqlx::query!(
                "SELECT * FROM product_inventory_evt WHERE product_id = $1",
                Uuid::parse_str(product_id.as_str())?
            ).fetch_all(&self.pool).await?;

            Ok(Some(Product::new(
                product.product_id.to_string(),
                product.name,
                product.price,
                record_product_inventory.iter().map(|x| x.quantity).sum(),
            )?))
        } else {
            Ok(None)
        }
    }
}
