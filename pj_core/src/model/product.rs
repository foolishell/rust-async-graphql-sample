use serde::Serialize;
use validator::{Validate};

use crate::error::DomainError;

#[derive(Serialize, Validate, Clone)]
pub struct Product {
    product_id: String,
    name: String,
    #[validate(range(min = 0))]
    price: i32,
    #[validate(range(min = 0))]
    inventory_quantity: i32,
}

impl Product {
    pub fn new(product_id: String, name: String, price: i32, inventory_quantity: i32) -> anyhow::Result<Self, DomainError> {
        let product = Self {
            product_id,
            name,
            price,
            inventory_quantity,
        };
        product.validate()?;
        Ok(product)
    }
}
