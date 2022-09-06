use async_graphql::{Object, ID};

#[derive(Clone)]
pub(crate) struct Product {
    product_id: ID,
    name: String,
    price: i32,
    inventory_quantity: i32,
}

#[Object(rename_fields = "camelCase")]
impl Product {
    pub(crate) async fn id(&self) -> ID {
        self.product_id.clone()
    }
    pub(crate) async fn name(&self) -> String {
        self.name.clone()
    }
    pub(crate) async fn price(&self) -> i32 {
        self.price
    }
    pub(crate) async fn inventory_quantity(&self) -> i32 {
        self.inventory_quantity
    }
}

impl From<pj_core::model::product::Product> for Product {
    fn from(product: pj_core::model::product::Product) -> Self {
        Self {
            product_id: ID::from(product.product_id()),
            name: product.name().clone(),
            price: *product.price(),
            inventory_quantity: *product.inventory_quantity(),
        }
    }
}
