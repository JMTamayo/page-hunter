use log::debug;
use page_hunter::{Page, PaginationResult, SQLxPagination};
use sqlx::{Error as SqlxError, FromRow, PgPool, Postgres, QueryBuilder};
use std::future::Future;
use uuid::Uuid;

use crate::db::handler::Repository;

use crate::models::products::{
    CreateProductRequest, PaginatedProducts, PaginatedProductsBase, Product, ProductBase,
    ProductId, SearchProductsRequest, UpdateProductQuantityRequest,
};

pub trait ProductsRepositoryMethods {
    fn get_product_by_id(&self, id: Uuid) -> impl Future<Output = Result<Product, SqlxError>>;

    fn list_products(
        &self,
        search_by: &SearchProductsRequest,
    ) -> impl Future<Output = PaginationResult<PaginatedProducts>>;

    fn delete_product(&self, id: Uuid) -> impl Future<Output = Result<ProductId, SqlxError>>;

    fn create_product(
        &self,
        product: &CreateProductRequest,
    ) -> impl Future<Output = Result<ProductId, SqlxError>>;

    fn update_product_quantity(
        &self,
        id: Uuid,
        update_by: &UpdateProductQuantityRequest,
    ) -> impl Future<Output = Result<ProductId, SqlxError>>;
}

pub struct ProductsRepository<'p> {
    pool: &'p PgPool,
}

impl<'p> Repository<'p> for ProductsRepository<'p> {
    fn get_pool(&self) -> &'p PgPool {
        self.pool
    }

    fn new(pool: &'p PgPool) -> Self {
        Self { pool }
    }
}

impl<'p> ProductsRepositoryMethods for ProductsRepository<'p> {
    async fn get_product_by_id(&self, id: Uuid) -> Result<Product, SqlxError> {
        Ok(Product::from(ProductBase::from_row(
            &QueryBuilder::<Postgres>::new(format!(
                r#"SELECT
						p.*,
						c.id AS category_id,
						c.name AS category_name,
						c.created_at AS category_created_at
					FROM
						inventory.products p
					INNER JOIN inventory.categories c ON
						p.category_id = c.id
					WHERE
						p.id = '{}';
				"#,
                id
            ))
            .build()
            .fetch_one(self.get_pool())
            .await?,
        )?))
    }

    async fn list_products(
        &self,
        search_by: &SearchProductsRequest,
    ) -> PaginationResult<PaginatedProducts> {
        let mut query: QueryBuilder<Postgres> = QueryBuilder::<Postgres>::new(
            r#"SELECT
				p.*,
				c.id AS category_id,
				c.name AS category_name,
				c.created_at AS category_created_at
			FROM
				inventory.products p
			INNER JOIN inventory.categories c ON
				p.category_id = c.id
			WHERE
				1=1
			"#,
        );

        match search_by.get_name_like() {
            Some(name_like) => {
                debug!("filtering by name_like: {}", name_like);
                query.push(format!("AND LOWER(p.name) ILIKE '%{name_like}%'"));
            }
            None => debug!("No name_like provided"),
        }

        match search_by.get_description_like() {
            Some(description_like) => {
                debug!("filtering by description_like: {}", description_like);
                query.push(format!(
                    "AND LOWER(p.description) ILIKE '%{description_like}%'"
                ));
            }
            None => debug!("No description_like provided"),
        }

        match search_by.get_category_id() {
            Some(category_id) => {
                debug!("filtering by category_id: {}", category_id);
                query.push(format!("AND category_id = '{}'", category_id));
            }
            None => debug!("No category_id provided"),
        }

        match search_by.get_category_name_like() {
            Some(category_name_like) => {
                debug!("filtering by category_name_like: {}", category_name_like);
                query.push(format!("AND LOWER(c.name) ILIKE '%{category_name_like}%'"));
            }
            None => debug!("No category_name_like provided"),
        }

        let products_base: PaginatedProductsBase = query
            .paginate(self.get_pool(), search_by.get_page(), search_by.get_size())
            .await?;

        let products: PaginatedProducts = Page::new(
            &products_base
                .clone()
                .into_iter()
                .map(|product_base| Product::from(product_base))
                .collect::<Vec<Product>>(),
            products_base.get_page(),
            products_base.get_size(),
            products_base.get_total(),
        )?;

        Ok(products)
    }

    async fn delete_product(&self, id: Uuid) -> Result<ProductId, SqlxError> {
        Ok(ProductId::from_row(
            &QueryBuilder::<Postgres>::new(format!(
                r#"
				DELETE FROM
					inventory.products p
				WHERE 
					p.id = '{}'
				RETURNING
					p.id;
			"#,
                id
            ))
            .build()
            .fetch_one(self.get_pool())
            .await?,
        )?)
    }

    async fn create_product(&self, product: &CreateProductRequest) -> Result<ProductId, SqlxError> {
        Ok(ProductId::from_row(
            &QueryBuilder::<Postgres>::new(format!(
                r#"
				INSERT INTO inventory.products (
					name,
					description,
					quantity,
					unitary_price,
					category_id
				) VALUES (
					'{name}',
					'{description}',
					{quantity},
					{unitary_price},
					'{category_id}'
				) RETURNING 
					id;
			"#,
                name = product.get_name(),
                description = product.get_description(),
                quantity = product.get_quantity(),
                unitary_price = product.get_unitary_price(),
                category_id = product.get_category_id()
            ))
            .build()
            .fetch_one(self.get_pool())
            .await?,
        )?)
    }

    async fn update_product_quantity(
        &self,
        id: Uuid,
        update_by: &UpdateProductQuantityRequest,
    ) -> Result<ProductId, SqlxError> {
        Ok(ProductId::from_row(
            &QueryBuilder::<Postgres>::new(format!(
                r#"
				UPDATE
					inventory.products p
				SET
					quantity = {quantity}
				WHERE
					p.id = '{id}'
				RETURNING
					p.id;
			"#,
                quantity = update_by.get_quantity(),
            ))
            .build()
            .fetch_one(self.get_pool())
            .await?,
        )?)
    }
}
