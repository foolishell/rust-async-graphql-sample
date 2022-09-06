use derive_getters::Getters;
use derive_more::Constructor;
use pj_core::usecase::get_product::GetProductUsecase;

use crate::infra::repository::product_repository::ProductRepository;

#[derive(Constructor, Getters)]
pub(crate) struct QueryUsecase {
    pub get_product_usecase: GetProductUsecase<ProductRepository>,
}
