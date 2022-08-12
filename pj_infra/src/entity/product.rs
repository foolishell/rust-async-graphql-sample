use pj_core::model::product::Product;

pub struct ProductResource {
    pub product_id: String,
    pub name: String,
    pub price: u32,
}

pub struct ProductInventoryEvent {
    pub product_id: String,
    pub amount: i32,
}

