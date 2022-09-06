use async_graphql::{Context, Object, ID};

use crate::error::AppError;

use crate::presentation::graphql::query_usecase::QueryUsecase;
use crate::presentation::graphql::schema::product::type_product::Product;

#[derive(Default)]
pub struct QueryProduct;

#[Object(rename_fields = "camelCase")]
impl QueryProduct {
    pub(crate) async fn product(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<Option<Product>, AppError> {
        let usecase = ctx.data::<QueryUsecase>().unwrap();
        let product = usecase.get_product_usecase.execute(id.to_string()).await?;

        Ok(product.map(Product::from))
    }
}
