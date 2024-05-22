//! ***Page Hunter*** library is a Rust-based pagination tool that provides a way to manage and navigate through pages of data.
//! It offers a set of resources that encapsulates all the necessary pagination information such as the current page, total pages, previous page, next page and the items on the current page.
//!
//! The library also includes validation methods to ensure the integrity of the pagination data.
//! It's designed to be flexible and easy to integrate into any Rust project that requires pagination functionality standard data validation
//!
//! To use **page-hunter** from GitHub repository with specific version, set the dependency in Cargo.toml file as follows:
//!
//! ```ini
//! [dependencies]
//! page-hunter = {git = "https://github.com/JMTamayo/page-hunter.git", version = "0.1.2", features = ["serde"] }
//! ```
//!
//! You can depend on it via cargo by adding the following dependency to your `Cargo.toml` file:
//!
//! ```ini
//! [dependencies]
//! page-hunter = { version = "0.1.2", features = ["serde"] }
//! ```
//!
//! ## CRATE FEATURES
//! - `serde`: Add [Serialize](https://docs.rs/serde/1.0.197/serde/trait.Serialize.html) and [Deserialize](https://docs.rs/serde/1.0.197/serde/trait.Deserialize.html) support for [`Page`] and [`Book`] based on crate [serde](https://crates.io/crates/serde/1.0.197). This feature is useful for implementing pagination models as a request or response body in REST APIs, among other implementations.
//! - `pg-sqlx`: Add support for pagination with [SQLx](https://crates.io/crates/sqlx) crate for PostgreSQL database. This feature is useful for paginating records from a PostgreSQL database.
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
//!
//! If you need to paginate records and get a specific [`Page`]:
//! ```rust,no_run
//!     use page_hunter::*;
//!
//!     let records: Vec<u32> = vec![1, 2, 3, 4, 5];
//!     let page: usize = 0;
//!     let size: usize = 2;
//!
//!     let pagination_result: PaginationResult<Page<u32>> =
//!         paginate_records(&records, page, size);
//! ```
//!
//! To create a new `Page` instance from known parameters:
//! ```rust,no_run
//!     use page_hunter::*;
//!
//!     let items: Vec<u32> = vec![1, 2];
//!     let page: usize = 0;
//!     let size: usize = 2;
//!     let total_elements: usize = 5;
//!
//!     let page_model_result: PaginationResult<Page<u32>> = Page::new(
//!         &items,
//!         page,
//!         size,
//!         total_elements,
//!     );
//! ```
//!
//! On feature `serde` enabled, you can serialize and deserialize a [`Page`] as follows:
//! ```rust,no_run
//!     use page_hunter::*;
//!
//!     let items: Vec<u32> = vec![1, 2];
//!     let page: usize = 0;
//!     let size: usize = 2;
//!     let total_elements: usize = 5;
//!
//!     let page_model: Page<u32> = Page::new(
//!         &items,
//!         page,
//!         size,
//!         total_elements,
//!     ).unwrap_or_else(|error| {
//!         panic!("Error creating page model: {:?}", error);
//!     });
//!
//!     let serialized_page: String = serde_json::to_string(&page_model).unwrap_or_else(|error| {
//!         panic!("Error serializing page model: {:?}", error);
//!     });
//!
//!     let deserialized_page: Page<u32> = serde_json::from_str(&serialized_page).unwrap_or_else(|error| {
//!         panic!("Error deserializing page model: {:?}", error);
//!     });
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
//!
//! If you need to bind records into a [`Book`] model:
//! ```rust,no_run
//!     use page_hunter::*;
//!
//!     let records: Vec<u32> = vec![1, 2, 3, 4, 5];
//!     let size: usize = 2;
//!
//!     let book_result: PaginationResult<Book<u32>> =
//!         bind_records(&records, size);
//! ```
//!
//! To create a new [`Book`] instance from known parameters:
//! ```rust,no_run
//!     use page_hunter::*;
//!
//!     let sheets: Vec<Page<u32>> = vec![
//!         Page::new(&vec![1, 2], 0, 2, 5).unwrap(),
//!         Page::new(&vec![3, 4], 1, 2, 5).unwrap(),
//!     ];
//!
//!     let book: Book<u32> = Book::new(&sheets);
//! ```
//!
//! On feature `serde` enabled, you can serialize and deserialize a [`Book`] as follows:
//! ```rust,no_run
//!     use page_hunter::*;
//!
//!     let sheets: Vec<Page<u32>> = vec![
//!         Page::new(&vec![1, 2], 0, 2, 5).unwrap(),
//!         Page::new(&vec![3, 4], 1, 2, 5).unwrap(),
//!     ];
//!
//!     let book: Book<u32> = Book::new(&sheets);
//!
//!     let serialized_book: String = serde_json::to_string(&book).unwrap_or_else(|error| {
//!         panic!("Error serializing book model: {:?}", error);
//!     });
//!
//!     let deserialized_book: Book<u32> = serde_json::from_str(&serialized_book).unwrap_or_else(|error| {
//!         panic!("Error deserializing book model: {:?}", error);
//!     });
//! ```
//!
//! #### Paginate records from a PostgreSQL database with SQLx:
//!
//! If you need to paginate records from a PostgreSQL database using the SQLx crate:
//! ```rust,no_run
//!     use page_hunter::*;
//!     use sqlx::postgres::{PgPool, Postgres};
//!     use sqlx::{FromRow, QueryBuilder};
//!     use uuid::Uuid;
//!
//!     #[tokio::main]
//!     async fn main() {
//!         #[derive(Clone, Debug, FromRow)]
//!         pub struct Country {
//!             id: Uuid,
//!             name: String,
//!         }
//!
//!         let pool: PgPool = PgPool::connect(
//!             "postgres://username:password@localhost/db"
//!         ).await.unwrap_or_else(|error| {
//!            panic!("Error connecting to database: {:?}", error);
//!        });
//!
//!         let query: QueryBuilder<Postgres> = QueryBuilder::new(
//!             "SELECT * FROM db.geo.countries"
//!         );
//!
//!         let page: Page<Country> =
//!             query.paginate(&pool, 0, 10).await.unwrap_or_else(|error| {
//!                 panic!("Error paginating records: {:?}", error);
//!             });
//!    }
//! ```
//!
//! ## CONTRIBUTIONS
//! The ***Page Hunter*** project is open source and therefore any interested software developer can contribute to its improvement. To contribute, take a look at the following recommendations:
//!
//! - **Bug Reports**: If you find a bug, please create an issue detailing the problem, the steps to reproduce it, and the expected behavior.
//! - **Feature Requests**: If you have an idea for a new feature or an enhancement to an existing one, please create an issue describing your idea.
//! - **Pull Requests**: If you've fixed a bug or implemented a new feature, we'd love to see your work! Please submit a pull request. Make sure your code follows the existing style and all tests pass.

mod page_hunter;

pub use page_hunter::errors::*;
pub use page_hunter::models::*;
pub use page_hunter::pagination::*;
