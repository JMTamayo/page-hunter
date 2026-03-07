//! `page-hunter` is a pagination library for Rust APIs and services.
//!
//! It provides:
//!
//! - [`Page`]: a validated model for one page of records.
//! - [`Book`]: a model that groups multiple pages.
//! - Record helpers: [`paginate_records`], [`bind_records`], and [`RecordsPagination`].
//! - SQLx integration via [`SQLxPagination`] when the `sqlx` feature is enabled.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use page_hunter::prelude::*;
//!
//! let records = vec![1_u32, 2, 3, 4, 5];
//! let page: Page<u32> = paginate_records(&records, 0, 2).unwrap();
//!
//! assert_eq!(page.get_items(), &vec![1, 2]);
//! assert_eq!(page.get_pages(), 3);
//! assert_eq!(page.get_next_page(), Some(1));
//! ```
//!
//! ## Core Models
//!
//! - [`Page`] stores `items`, `page`, `size`, `total`, `pages`, `previous_page`, and `next_page`.
//! - [`Book`] stores a list of [`Page`] values.
//!
//! Create a [`Page`] directly when you already know all values:
//!
//! ```rust,no_run
//! use page_hunter::{Page, PaginationResult};
//!
//! let items = vec![1_u32, 2];
//! let page_model: PaginationResult<Page<u32>> = Page::new(&items, 0, 2, 5);
//! ```
//!
//! Build a [`Book`] from existing pages:
//!
//! ```rust,no_run
//! use page_hunter::{Book, Page};
//!
//! let sheets: Vec<Page<u32>> = vec![
//!     Page::new(&vec![1, 2], 0, 2, 5).unwrap(),
//!     Page::new(&vec![3, 4], 1, 2, 5).unwrap(),
//!     Page::new(&vec![5], 2, 2, 5).unwrap(),
//! ];
//!
//! let book: Book<u32> = Book::new(&sheets);
//! assert_eq!(book.get_sheets().len(), 3);
//! ```
//!
//! ## In-Memory Pagination
//!
//! Use either free functions or the trait extension:
//!
//! ```rust,no_run
//! use page_hunter::{Page, RecordsPagination, paginate_records};
//!
//! let records = vec![10, 20, 30, 40, 50];
//! let a: Page<i32> = paginate_records(&records, 1, 2).unwrap();
//! let b: Page<i32> = records.paginate(1, 2).unwrap();
//!
//! assert_eq!(a.get_items(), b.get_items());
//! ```
//!
//! ## SQLx Pagination (`sqlx` feature)
//!
//! [`SQLxPagination::paginate`] accepts both:
//!
//! - a pool reference (`&Pool<DB>`)
//! - a single connection (`&mut DB::Connection`)
//!
//! Example with a connection:
//!
//! ```rust,no_run
//! # #[cfg(feature = "sqlx")]
//! # async fn run() {
//! use page_hunter::{Page, SQLxPagination};
//! use sqlx::postgres::{PgConnection, Postgres};
//! use sqlx::{Connection, FromRow, QueryBuilder};
//!
//! #[derive(Clone, Debug, FromRow)]
//! struct Country {
//!     id: i32,
//!     name: String,
//! }
//!
//! let mut conn = PgConnection::connect("postgres://username:password@localhost/db")
//!     .await
//!     .unwrap();
//! let query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM geo.countries");
//!
//! let page: Page<Country> = query.paginate(&mut conn, 0, 10).await.unwrap();
//! assert_eq!(page.get_size(), 10);
//! # }
//! ```
//!
//! ## Feature Flags
//!
//! - `serde`: adds `Serialize` and `Deserialize` support for [`Page`] and [`Book`].
//! - `utoipa`: adds `ToSchema` support for [`Page`] and [`Book`] (depends on `serde`).
//! - `sqlx`: enables SQL query pagination support via [`SQLxPagination`].
//!
//! ## Validation Rules
//!
//! Every [`Page`] is validated on construction (and on deserialization when `serde` is enabled).
//! Validation ensures internal consistency for pagination metadata, including page ranges,
//! element counts, and previous/next pointers.
//!
//! Invalid data returns [`PaginationError`].
//!
//! ## See Also
//!
//! - Repository examples: <https://github.com/JMTamayo/page-hunter/tree/main/examples>
mod book;
mod errors;
mod page;
mod pagination;
mod results;

pub use book::Book;
pub use errors::{ErrorKind, PaginationError};
pub use page::Page;
pub use pagination::records::{RecordsPagination, bind_records, paginate_records};
pub use results::PaginationResult;

#[cfg(feature = "sqlx")]
pub use pagination::sqlx::queries::SQLxPagination;

/// Re-exports of the most commonly used `page-hunter` items.
///
/// This module is designed for ergonomic imports:
///
/// ```rust,no_run
/// use page_hunter::prelude::*;
///
/// let records = vec![1_u32, 2, 3];
/// let page: Page<u32> = paginate_records(&records, 0, 2).unwrap();
/// assert_eq!(page.get_page(), 0);
/// ```
pub mod prelude {
    pub use crate::{
        Book, ErrorKind, Page, PaginationError, PaginationResult, RecordsPagination, bind_records,
        paginate_records,
    };

    #[cfg(feature = "sqlx")]
    pub use crate::SQLxPagination;
}
