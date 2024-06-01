use log::debug;
use page_hunter::{PaginationResult, SQLxPagination};
use sqlx::{Error as SqlxError, FromRow, PgPool, Postgres, QueryBuilder};
use std::future::Future;
use uuid::Uuid;

use crate::db::handler::Repository;

use crate::models::categories::{Category, PaginatedCategories, SearchCategoriesRequest};

pub trait CategoriesRepositoryMethods {
    fn get_category_by_id(&self, id: Uuid) -> impl Future<Output = Result<Category, SqlxError>>;

    fn list_categories(
        &self,
        search_by: &SearchCategoriesRequest,
    ) -> impl Future<Output = PaginationResult<PaginatedCategories>>;
}

pub struct CategoriesRepository<'p> {
    pool: &'p PgPool,
}

impl<'p> Repository<'p> for CategoriesRepository<'p> {
    fn get_pool(&self) -> &'p PgPool {
        self.pool
    }

    fn new(pool: &'p PgPool) -> Self {
        Self { pool }
    }
}

impl<'p> CategoriesRepositoryMethods for CategoriesRepository<'p> {
    async fn get_category_by_id(&self, id: Uuid) -> Result<Category, SqlxError> {
        Ok(Category::from_row(
            &QueryBuilder::<Postgres>::new(format!(
                r#"SELECT
						*
					FROM
						inventory.categories
					WHERE
						1=1
						AND id = '{}';
				"#,
                id
            ))
            .build()
            .fetch_one(self.get_pool())
            .await?,
        )?)
    }

    async fn list_categories(
        &self,
        search_by: &SearchCategoriesRequest,
    ) -> PaginationResult<PaginatedCategories> {
        let mut query: QueryBuilder<Postgres> = QueryBuilder::<Postgres>::new(
            r#"SELECT
				*
			FROM
				inventory.categories
			WHERE
				1=1
			"#,
        );

        match search_by.get_name_like() {
            Some(name_like) => {
                debug!("filtering by name_like: {}", name_like);
                query.push(format!("AND LOWER(name) ILIKE '%{name_like}%'"));
            }
            None => debug!("No name_like provided"),
        }

        let categories: PaginatedCategories = query
            .paginate(self.get_pool(), search_by.get_page(), search_by.get_size())
            .await?;

        Ok(categories)
    }
}
