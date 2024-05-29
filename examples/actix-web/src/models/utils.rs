use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct PaginationParams {
    page: Option<usize>,
    size: Option<usize>,
}

impl PaginationParams {
    pub fn get_page(&self) -> usize {
        match self.page {
            Some(page) => page,
            None => 0,
        }
    }

    pub fn get_size(&self) -> usize {
        match self.size {
            Some(size) => size,
            None => 15,
        }
    }
}

#[derive(Deserialize, IntoParams)]
pub struct BindingParams {
    pub size: Option<usize>,
}

impl BindingParams {
    pub fn get_size(&self) -> usize {
        match self.size {
            Some(size) => size,
            None => 15,
        }
    }
}
