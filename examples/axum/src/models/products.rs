use page_hunter::Page;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::{serde::rfc3339, OffsetDateTime};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::models::categories::Category;

#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchProductsRequest {
    name_like: Option<String>,
    description_like: Option<String>,
    category_id: Option<Uuid>,
    category_name_like: Option<String>,
    page: Option<usize>,
    size: Option<usize>,
}

impl SearchProductsRequest {
    pub fn get_name_like(&self) -> &Option<String> {
        &self.name_like
    }

    pub fn get_description_like(&self) -> &Option<String> {
        &self.description_like
    }

    pub fn get_category_id(&self) -> Option<Uuid> {
        self.category_id
    }

    pub fn get_category_name_like(&self) -> &Option<String> {
        &self.category_name_like
    }

    pub fn get_page(&self) -> usize {
        self.page.unwrap_or(0)
    }

    pub fn get_size(&self) -> usize {
        self.size.unwrap_or(50)
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProductRequest {
    name: String,
    description: String,
    quantity: i32,
    unitary_price: f64,
    category_id: Uuid,
}

impl CreateProductRequest {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }

    pub fn get_unitary_price(&self) -> f64 {
        self.unitary_price
    }

    pub fn get_category_id(&self) -> Uuid {
        self.category_id
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProductQuantityRequest {
    quantity: i32,
}

impl UpdateProductQuantityRequest {
    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct ProductBase {
    id: Uuid,
    name: String,
    description: String,
    quantity: i32,
    unitary_price: f64,
    created_at: OffsetDateTime,
    updated_at: Option<OffsetDateTime>,
    category_id: Uuid,
    category_name: String,
    category_created_at: OffsetDateTime,
}

impl ProductBase {
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }

    pub fn get_unitary_price(&self) -> f64 {
        self.unitary_price
    }

    pub fn get_created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    pub fn get_updated_at(&self) -> Option<OffsetDateTime> {
        self.updated_at
    }

    pub fn get_category_id(&self) -> Uuid {
        self.category_id
    }

    pub fn get_category_name(&self) -> &str {
        &self.category_name
    }

    pub fn get_category_created_at(&self) -> OffsetDateTime {
        self.category_created_at
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Product {
    id: Uuid,
    name: String,
    description: String,
    quantity: i32,
    unitary_price: f64,
    #[serde(with = "rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "rfc3339::option")]
    updated_at: Option<OffsetDateTime>,
    category: Category,
}

impl Product {
    pub fn new(
        id: Uuid,
        name: String,
        description: String,
        quantity: i32,
        unitary_price: f64,
        created_at: OffsetDateTime,
        updated_at: Option<OffsetDateTime>,
        category: Category,
    ) -> Self {
        Self {
            id,
            name,
            description,
            quantity,
            unitary_price,
            created_at,
            updated_at,
            category,
        }
    }
}

impl From<ProductBase> for Product {
    fn from(product_base: ProductBase) -> Self {
        Self::new(
            product_base.get_id(),
            product_base.get_name().to_owned(),
            product_base.get_description().to_owned(),
            product_base.get_quantity(),
            product_base.get_unitary_price(),
            product_base.get_created_at(),
            product_base.get_updated_at(),
            Category::new(
                product_base.get_category_id(),
                product_base.get_category_name().to_owned(),
                product_base.get_category_created_at(),
            ),
        )
    }
}

pub type PaginatedProductsBase = Page<ProductBase>;
pub type PaginatedProducts = Page<Product>;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct ProductId {
    id: Uuid,
}

impl ProductId {
    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
