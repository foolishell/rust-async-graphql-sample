use async_graphql::MergedObject;
use query_product::QueryProduct;

pub mod query_product;
pub mod type_product;

#[derive(MergedObject, Default)]
pub struct ProductQuery(QueryProduct);
