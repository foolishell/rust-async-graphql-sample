use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};

use crate::presentation::graphql::schema::product::ProductQuery;

pub mod product;

#[derive(MergedObject, Default)]
pub struct Query(ProductQuery);

pub fn schema_builder() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
}

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;
