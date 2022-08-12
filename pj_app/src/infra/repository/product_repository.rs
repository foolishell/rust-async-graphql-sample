use async_trait::async_trait;
use pj_core::model::product::Product;
use pj_core::repository_trait::ProductRepositoryTrait;
use sqlx::types::Uuid;
use sqlx::PgPool;

pub(crate) struct ProductRepository {
    pool: PgPool,
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepositoryTrait for ProductRepository {
    async fn find_product(&self, product_id: String) -> anyhow::Result<Product> {
        let record = sqlx::query!(
            "SELECT * FROM product_rsc WHERE product_id = $1",
            Uuid::parse_str(product_id.as_str())?
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(Product {
            product_id: record.product_id.to_string(),
            name: record.name,
            price: record.price as u32,
            inventory_quantity: 0,
        })
    }
}
