use serde::Serialize;

#[derive(Serialize)]
pub struct Product {
    pub product_id: String,
    pub name: String,
    pub price: u32,
    pub inventory_quantity: u32,
}
