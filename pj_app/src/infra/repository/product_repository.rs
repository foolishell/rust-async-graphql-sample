use async_trait::async_trait;

use pj_core::model::product::Product;
use pj_core::repository_trait::ProductRepositoryTrait;
use sqlx::types::Uuid;
use sqlx::{PgPool, Transaction};

use crate::error::AppError;

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
    type Err = AppError;

    async fn find_product(&self, product_id: String) -> Result<Option<Product>, AppError> {
        let mut tx = self.pool.begin().await?;
        let product_id = Uuid::parse_str(product_id.as_str())?;
        let product = InternalProductRepository::find_product(&product_id, &mut tx).await?;
        tx.commit().await?;

        Ok(product)
    }
}

pub(in crate::infra) struct InternalProductRepository {}

impl InternalProductRepository {
    async fn find_product(
        product_id: &Uuid,
        tx: &mut Transaction<'_, sqlx::Postgres>,
    ) -> Result<Option<Product>, AppError> {
        let record_product = sqlx::query!(
            "SELECT * FROM product_rsc WHERE product_id = $1",
            product_id
        )
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(product) = record_product {
            let record_product_inventory = sqlx::query!(
                "SELECT * FROM product_inventory_evt WHERE product_id = $1",
                product_id
            )
            .fetch_all(&mut *tx)
            .await?;

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

#[cfg(test)]
mod find_product_tests {

    use sqlx::postgres::PgPoolOptions;

    use super::*;

    #[tokio::test]
    async fn test_not_found() -> anyhow::Result<()> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:password@localhost/pj_db")
            .await?;
        let mut tx = pool.begin().await?;

        let product_id = Uuid::new_v4();

        let product_none = InternalProductRepository::find_product(&product_id, &mut tx)
            .await
            .unwrap();
        assert!(product_none.is_none());

        tx.rollback().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_ok() -> anyhow::Result<()> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:password@localhost/pj_db")
            .await?;
        let mut tx = pool.begin().await?;

        let product_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO product_rsc (product_id, name, price) VALUES ($1, 'name', 111)",
            product_id
        )
        .execute(&mut *tx)
        .await?;

        let quantities = vec![20, -10, 30];
        for n in quantities.iter() {
            sqlx::query!(
                "INSERT INTO product_inventory_evt (product_id, quantity) VALUES ($1, $2)",
                product_id,
                n
            )
            .execute(&mut *tx)
            .await?;
        }

        let product = InternalProductRepository::find_product(&product_id, &mut tx)
            .await
            .unwrap();
        assert!(product.is_some());
        assert_eq!(
            product.unwrap(),
            Product::new(
                product_id.to_string(),
                "name".to_string(),
                111,
                quantities.iter().sum()
            )
            .unwrap()
        );

        tx.rollback().await?;
        Ok(())
    }
}
