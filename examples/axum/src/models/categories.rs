use page_hunter::Page;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::{serde::rfc3339, OffsetDateTime};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchCategoriesRequest {
    name_like: Option<String>,
    page: Option<usize>,
    size: Option<usize>,
}

impl SearchCategoriesRequest {
    pub fn get_name_like(&self) -> &Option<String> {
        &self.name_like
    }

    pub fn get_page(&self) -> usize {
        self.page.unwrap_or(0)
    }

    pub fn get_size(&self) -> usize {
        self.size.unwrap_or(50)
    }
}

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
pub struct Category {
    id: Uuid,
    name: String,
    #[serde(with = "rfc3339")]
    created_at: OffsetDateTime,
}

impl Category {
    pub fn new(id: Uuid, name: String, created_at: OffsetDateTime) -> Self {
        Self {
            id,
            name,
            created_at,
        }
    }
}

pub type PaginatedCategories = Page<Category>;
