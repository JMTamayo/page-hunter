//! ***Page Hunter*** library is a Rust-based pagination tool that provides a way to manage and navigate through pages of data.
//! It offers a set of resources that encapsulates all the necessary pagination information such as the current page, total pages, previous page, next page and the items on the current page.
//!
//! The library also includes validation methods to ensure the integrity of the pagination data.
//! It's designed to be flexible and easy to integrate into any Rust project that requires pagination functionality standard data validation
//!
//! ## CRATE FEATURES
//! - `serde`: Add [Serialize](https://docs.rs/serde/1.0.200/serde/trait.Serialize.html) and [Deserialize](https://docs.rs/serde/1.0.200/serde/trait.Deserialize.html) support for [`Page`] and [`Book`] based on [serde](https://crates.io/crates/serde/1.0.200). This feature is useful for implementing pagination models as a request or response body in REST APIs, among other implementations.
//!  - `utoipa`: Add [ToSchema](https://docs.rs/utoipa/4.2.0/utoipa/trait.ToSchema.html) support for [`Page`] and  [`Book`] based on [utoipa](https://crates.io/crates/utoipa/4.2.0). This feature is useful for generating OpenAPI schemas for pagination models. This feature depends on the `serde` feature and therefore you only need to implement `utoipa` to get both.
//! - `sqlx`: Add support for pagination with [SQLx](https://docs.rs/sqlx/0.8.1/sqlx/) for Postgres, MySQL and SQLite databases.
//!
//! ## BASIC OPERATION
//!
//! The **page-hunter** library provides two main models to manage pagination:
//! - [`Page`]: Represents a page of records with the current page, total pages, previous page, next page, and the items on the current page.
//! - [`Book`]: Represents a book of pages with a collection of [`Page`] instances.
//!
//! The library also provides a set of functions to paginate records into a [`Page`] model and bind records into a [`Book`] model. The following examples show how to use the **page-hunter** library:
//!
//! #### Paginate records:
//! If you need to paginate records and get a specific [`Page`]:
//! ```rust,no_run
//!   use page_hunter::{Page, paginate_records, RecordsPagination};
//!
//!   let records: Vec<u32> = vec![1, 2, 3, 4, 5];
//!   let page: usize = 0;
//!   let size: usize = 2;
//!
//!   // Using the paginate_records function:
//!   let page_model: Page<u32> = match paginate_records(&records, page, size) {
//!     Ok(p) => p,
//!     Err(e) => panic!("Error paginating records: {:?}", e),
//!   };
//!
//!   // Using RecordsPagination trait:
//!   let page_model: Page<u32> = match records.paginate(page, size) {
//!     Ok(p) => p,
//!     Err(e) => panic!("Error paginating records: {:?}", e),
//!   };
//! ```
//!
//! To create a new `Page` instance from known parameters:
//! ```rust,no_run
//!   use page_hunter::{Page, PaginationResult};
//!
//!   let items: Vec<u32> = vec![1, 2];
//!   let page: usize = 0;
//!   let size: usize = 2;
//!   let total_elements: usize = 5;
//!
//!   let page_model_result: PaginationResult<Page<u32>> = Page::new(
//!     &items,
//!     page,
//!     size,
//!     total_elements,
//!   );
//! ```
//!
//! On feature `serde` enabled, you can serialize and deserialize a [`Page`] as follows:
//! ```rust,no_run
//!   use page_hunter::Page;
//!
//!   let items: Vec<u32> = vec![1, 2];
//!   let page: usize = 0;
//!   let size: usize = 2;
//!   let total_elements: usize = 5;
//!
//!   let page_model: Page<u32> = Page::new(
//!     &items,
//!     page,
//!     size,
//!     total_elements,
//!   ).unwrap_or_else(|error| {
//!     panic!("Error creating page model: {:?}", error);
//!   });
//!
//!   let serialized_page: String = serde_json::to_string(&page_model)
//!     .unwrap_or_else(|error| {
//!       panic!("Error serializing page model: {:?}", error);
//!   });
//!
//!   let deserialized_page: Page<u32> = serde_json::from_str(&serialized_page)
//!     .unwrap_or_else(|error| {
//!       panic!("Error deserializing page model: {:?}", error);
//!   });
//! ```
//!
//! When you create a new [`Page`] instance from the constructor or deserialization, the following rules are validated for the fields on the page:
//! - ***pages*** must be equal to ***total*** divided by ***size*** rounded up. When ***size*** is 0, ***pages*** must be 1.
//! - ***page*** must be less than or equal to ***pages*** - 1.
//! - if ***page*** is less than ***pages*** - 1, ***items*** length must be equal to ***size***.
//! - if ***page*** is equal to ***pages*** - 1, ***total*** must be equal to (***pages*** - 1) * ***size*** + ***items*** length.
//! - ***previous_page*** must be equal to ***page*** - 1 if ***page*** is greater than 0, otherwise it must be [`None`].
//! - ***next_page*** must be equal to ***page*** + 1 if ***page*** is less than ***pages*** - 1, otherwise it must be [`None`].
//!
//! If any of these rules are violated, a [`PaginationError`] will be returned.
//!
//! #### Bind records:
//! If you need to bind records into a [`Book`] model:
//! ```rust,no_run
//!   use page_hunter::{bind_records, Book, RecordsPagination};
//!
//!   let records: Vec<u32> = vec![1, 2, 3, 4, 5];
//!   let size: usize = 2;
//!
//!   // Using the bind_records function:
//!   let book: Book<u32> = match bind_records(&records, size) {
//!     Ok(b) => b,
//!     Err(e) => panic!("Error binding records: {:?}", e),
//!   };
//!
//!   // Using RecordsPagination trait:
//!   let book: Book<u32> = match records.bind(size) {
//!     Ok(b) => b,
//!     Err(e) => panic!("Error binding records: {:?}", e),
//!   };
//! ```
//!
//! To create a new [`Book`] instance from known parameters:
//! ```rust,no_run
//!   use page_hunter::{Book, Page};
//!
//!   let sheets: Vec<Page<u32>> = vec![
//!     Page::new(&vec![1, 2], 0, 2, 5).unwrap(),
//!     Page::new(&vec![3, 4], 1, 2, 5).unwrap(),
//!   ];
//!
//!   let book: Book<u32> = Book::new(&sheets);
//! ```
//!
//! On feature `serde` enabled, you can serialize and deserialize a [`Book`] as follows:
//! ```rust,no_run
//!   use page_hunter::{Book, Page};
//!
//!   let sheets: Vec<Page<u32>> = vec![
//!     Page::new(&vec![1, 2], 0, 2, 5).unwrap(),
//!     Page::new(&vec![3, 4], 1, 2, 5).unwrap(),
//!   ];
//!
//!   let book: Book<u32> = Book::new(&sheets);
//!
//!   let serialized_book: String = serde_json::to_string(&book)
//!     .unwrap_or_else(|error| {
//!       panic!("Error serializing book model: {:?}", error);
//!   });
//!
//!   let deserialized_book: Book<u32> = serde_json::from_str(&serialized_book)
//!     .unwrap_or_else(|error| {
//!       panic!("Error deserializing book model: {:?}", error);
//!   });
//! ```
//!
//! #### Generate OpenAPI schemas:
//! On feature `utoipa` enabled, you can generate OpenAPI schemas for [`Page`] and [`Book`] models as follows:
//!
//! ```rust,no_run
//!   use page_hunter::{Book, Page};
//!   use utoipa::{OpenApi, ToSchema};
//!   use serde::{Deserialize, Serialize};
//!
//!   #[derive(Clone, ToSchema)]
//!   pub struct Person {
//!     id: u16,
//!     name: String,
//!     last_name: String,
//!     still_alive: bool,
//!   }
//!
//!   pub type PeoplePage = Page<Person>;
//!   pub type PeopleBook = Book<Person>;
//!
//!   #[derive(OpenApi)]
//!   #[openapi(
//!     components(schemas(PeoplePage, PeopleBook))
//!   )]
//!   pub struct ApiDoc;
//! ```
//!
//! Take a look at the [examples](https://github.com/JMTamayo/page-hunter/tree/main/examples)  folder where you can find practical uses in REST API implementations with some web frameworks.
//!
//! #### Paginate records from a relational database with SQLx:
//! To paginate records from a Postgres database:
//! ```rust,no_run
//!   use page_hunter::{Page, SQLxPagination};
//!   use sqlx::postgres::{PgPool, Postgres};
//!   use sqlx::{FromRow, QueryBuilder};
//!   use uuid::Uuid;
//!
//!   #[tokio::main]
//!   async fn main() {
//!     #[derive(Clone, Debug, FromRow)]
//!     pub struct Country {
//!       id: Uuid,
//!       name: String,
//!     }
//!
//!     let pool: PgPool = PgPool::connect(
//!       "postgres://username:password@localhost/db"
//!     ).await.unwrap_or_else(|error| {
//!       panic!("Error connecting to database: {:?}", error);
//!     });
//!
//!     let query: QueryBuilder<Postgres> = QueryBuilder::new(
//!       "SELECT * FROM db.geo.countries"
//!     );
//!
//!     let page: Page<Country> =
//!       query.paginate(&pool, 0, 10).await.unwrap_or_else(|error| {
//!         panic!("Error paginating records: {:?}", error);
//!     });
//!   }
//! ```
//!
//! Similar to using pagination for Postgres, [`SQLxPagination`] can be used for MySQL and SQLite.
mod book;
mod errors;
mod page;
mod pagination;
mod results;

pub use book::Book;
pub use errors::{ErrorKind, PaginationError};
pub use page::Page;
pub use pagination::records::{bind_records, paginate_records, RecordsPagination};
pub use results::PaginationResult;

#[cfg(feature = "sqlx")]
pub use pagination::sqlx::queries::SQLxPagination;
