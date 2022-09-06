use pj_core::usecase::get_product::GetProductUsecase;
use sqlx::PgPool;

use crate::infra::repository::product_repository::ProductRepository;
use crate::presentation::graphql::query_usecase::QueryUsecase;

pub(crate) fn inject(pool: PgPool) -> QueryUsecase {
    let get_product_usecase = GetProductUsecase::new(ProductRepository::new(pool.clone()));
    let qu = QueryUsecase::new(get_product_usecase);
    qu
}
