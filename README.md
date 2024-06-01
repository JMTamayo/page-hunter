# PAGE HUNTER

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
[![dependency status](https://deps.rs/repo/github/JMTamayo/page-hunter/status.svg)](https://deps.rs/repo/github/JMTamayo/page-hunter)
[![CI](https://github.com/JMTamayo/page-hunter/actions/workflows/ci.yml/badge.svg)](https://github.com/JMTamayo/page-hunter/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/JMTamayo/page-hunter/graph/badge.svg?token=R1LAPNSV5J)](https://codecov.io/gh/JMTamayo/page-hunter)
[![crates.io](https://img.shields.io/crates/v/page-hunter.svg?label=crates.io&color=orange&logo=rust)](https://crates.io/crates/page-hunter)
[![docs.rs](https://img.shields.io/static/v1?label=docs.rs&message=latest&color=blue&logo=docsdotrs)](http://docs.rs/page-hunter/latest/)


***Page Hunter*** library is a Rust-based pagination tool that provides a way to manage and navigate through pages of data.
It offers a set of resources that encapsulates all the necessary pagination information such as the current page, total pages, previous page, next page and the items on the current page.

The library also includes validation methods to ensure the integrity of the pagination data.
It's designed to be flexible and easy to integrate into any Rust project that requires pagination functionality and standard data validation.

To use **page-hunter** from GitHub repository with specific version, set the dependency in Cargo.toml file as follows:

```ini
[dependencies]
page-hunter = { git = "https://github.com/JMTamayo/page-hunter.git", version = "0.2.0", features = ["serde"] }
```

You can depend on it via cargo by adding the following dependency to your `Cargo.toml` file:

```ini
[dependencies]
page-hunter = { version = "0.2.0", features = ["utoipa", "pg-sqlx"] }
```

## CRATE FEATURES
- `serde`: Add [Serialize](https://docs.rs/serde/1.0.203/serde/trait.Serialize.html) and [Deserialize](https://docs.rs/serde/1.0.203/serde/trait.Deserialize.html) support for `Page` and `Book` based on [serde](https://crates.io/crates/serde/1.0.203). This feature is useful for implementing pagination models as a request or response body in REST APIs, among other implementations.
- `utoipa`: Add [ToSchema](https://docs.rs/utoipa/4.2.3/utoipa/trait.ToSchema.html) support for `Page` and  `Book` based on [utoipa](https://crates.io/crates/utoipa/4.2.3). This feature is useful for generating OpenAPI schemas for pagination models. This feature depends on the `serde` feature and therefore you only need to implement `utoipa` to get both.
- `pg-sqlx`: Add support for pagination with [SQLx](https://docs.rs/sqlx/0.7.4/sqlx/) for PostgreSQL database.
- `mysql-sqlx`: Add support for pagination with [SQLx](https://docs.rs/sqlx/0.7.4/sqlx/) for MySQL database.

## BASIC OPERATION
The **page-hunter** library provides two main models to manage pagination:
- `Page`: Represents a page of records with the current page, total pages, previous page, next page, and the items on the current page.
- `Book`: Represents a book of pages with a collection of `Page` instances.

The library also provides a set of functions to paginate records into a `Page` model and bind records into a `Book` model. The following examples show how to use the **page-hunter** library:

### Paginate records:
If you need to paginate records and get a specific `Page`:
```rust,no_run
    use page_hunter::*;

    let records: Vec<u32> = vec![1, 2, 3, 4, 5];
    let page: usize = 0;
    let size: usize = 2;

    let pagination_result: PaginationResult<Page<u32>> =
        paginate_records(&records, page, size);
```

To create a new instance of a `Page` from known parameters:
```rust,no_run
    use page_hunter::*;

    let items: Vec<u32> = vec![1, 2];
    let page: usize = 0;
    let size: usize = 2;
    let total_elements: usize = 5;

    let page_model_result: PaginationResult<Page<u32>> = Page::new(
        &items,
        page,
        size,
        total_elements,
    );
```

On feature `serde` enabled, you can serialize and deserialize a `Page` as follows:
```rust,no_run
    use page_hunter::*;

    let items: Vec<u32> = vec![1, 2];
    let page: usize = 0;
    let size: usize = 2;
    let total_elements: usize = 5;

    let page_model: PaginationResult<Page<u32>> = Page::new(
        &items,
        page,
        size,
        total_elements,
    ).unwrap_or_else(|error| {
        panic!("Error creating page model: {:?}", error);
    });

    let serialized_page: String = serde_json::to_string(&page_model).unwrap_or_else(|error| {
        panic!("Error serializing page model: {:?}", error);
    });

    let deserialized_page: Page<u32> = serde_json::from_str(&serialized_page).unwrap_or_else(|error| {
        panic!("Error deserializing page model: {:?}", error);
    });
```

When you create a new `Page` instance from the constructor or deserialization, the following rules are validated for the fields on the page:
- ***pages*** must be equal to ***total*** divided by ***size*** rounded up. When ***size*** is 0, ***pages*** must be 1.
- ***page*** must be less than or equal to ***pages*** - 1.
- if ***page*** is less than ***pages*** - 1, ***items*** length must be equal to ***size***.
- if ***page*** is equal to ***pages*** - 1, ***total*** must be equal to (***pages*** - 1) * ***size*** + ***items*** length.
- ***previous_page*** must be equal to ***page*** - 1 if ***page*** is greater than 0, otherwise it must be `None`.
- ***next_page*** must be equal to ***page*** + 1 if ***page*** is less than ***pages*** - 1, otherwise it must be `None`.

If any of these rules are violated, a `PaginationError` will be returned.

### Bind records:
If you need to bind records into a `Book` model:
```rust,no_run
    use page_hunter::*;

    let records: Vec<u32> = vec![1, 2, 3, 4, 5];
    let size: usize = 2;

    let book_result: PaginationResult<Book<u32>> =
        bind_records(&records, size);
```

To create a new `Book` instance from known parameters:
```rust,no_run
    use page_hunter::*;

    let sheets: Vec<Page<u32>> = vec![
        Page::new(&vec![1, 2], 0, 2, 5).unwrap(),
        Page::new(&vec![3, 4], 1, 2, 5).unwrap(),
    ];

    let book: Book<u32> = Book::new(&sheets);
```

On feature `serde` enabled, you can serialize and deserialize a `Book` as follows:
```rust,no_run
    use page_hunter::*;

    let sheets: Vec<Page<u32>> = vec![
        Page::new(&vec![1, 2], 0, 2, 5).unwrap(),
        Page::new(&vec![3, 4], 1, 2, 5).unwrap(),
    ];

    let book: Book<u32> = Book::new(&sheets);

    let serialized_book: String = serde_json::to_string(&book).unwrap_or_else(|error| {
        panic!("Error serializing book model: {:?}", error);
    });

    let deserialized_book: Book<u32> = serde_json::from_str(&serialized_book).unwrap_or_else(|error| {
        panic!("Error deserializing book model: {:?}", error);
    });
```

#### Generate OpenAPI schemas:
 On feature `utoipa` enabled, you can generate OpenAPI schemas for `Page` and `Book` models as follows:

 ```rust,no_run
	use page_hunter::*;
	use utoipa::ToSchema;
	use serde::{Deserialize, Serialize};

	#[derive(Clone, ToSchema)]
	pub struct Person {
        id: u16,
        name: String,
        last_name: String,
        still_alive: bool,
	}

	pub type PeoplePage = Page<Person>;
	pub type PeopleBook = Book<Person>;

    #[derive(OpenApi)]
    #[openapi(
        components(schemas(PeoplePage, PeopleBook))
    )]
    pub struct ApiDoc;
```

Take a look at the [examples](https://github.com/JMTamayo/page-hunter/tree/main/examples) folder where you can find practical uses in REST API implementations with some web frameworks.

#### Paginate records from a PostgreSQL database with SQLx:
To paginate records from a PostgreSQL database:
```rust,no_run
    use page_hunter::*;
    use sqlx::postgres::{PgPool, Postgres};
    use sqlx::{FromRow, QueryBuilder};
    use uuid::Uuid;

    #[tokio::main]
    async fn main() {
        #[derive(Clone, Debug, FromRow)]
        pub struct Country {
            id: Uuid,
            name: String,
        }

        let pool: PgPool = PgPool::connect(
            "postgres://username:password@localhost/db"
        ).await.unwrap_or_else(|error| {
           panic!("Error connecting to database: {:?}", error);
       });

        let query: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT * FROM db.geo.countries"
        );

        let page: Page<Country> =
            query.paginate(&pool, 0, 10).await.unwrap_or_else(|error| {
                panic!("Error paginating records: {:?}", error);
            });
    }
```

To paginate records from a MySQL database:
```rust,no_run
    use page_hunter::*;
    use sqlx::mysql::{MySqlPool, MySql};
    use sqlx::{FromRow, QueryBuilder};
    use uuid::Uuid;

    #[tokio::main]
    async fn main() {
        #[derive(Clone, Debug, FromRow)]
        pub struct Country {
            id: Uuid,
            name: String,
        }

        let pool: MySqlPool = MySqlPool::connect(
            "mysql://username:password@localhost/db"
        ).await.unwrap_or_else(|error| {
            panic!("Error connecting to database: {:?}", error);
        });

        let query: QueryBuilder<MySql> = QueryBuilder::new(
            "SELECT * FROM countries"
        );

        let page: Page<Country> =
            query.paginate(&pool, 0, 10).await.unwrap_or_else(|error| {
               panic!("Error paginating records: {:?}", error);
            });
    }
```

## DEVELOPMENT
To test `page-hunter`, follow these recommendations:

#### Set env variables:
Create `local.env` file at workspace folder to store the required environment variables
```text
    DB_HOST=localhost
    DB_USER=test
    DB_PASSWORD=docker
    DB_NAME=test
    PG_DB_PORT=5432
    MYSQL_DB_PORT=3306
```

#### Setup databases:
Run databases as Docker containers using the following commands:

##### Postgres SQL:
```bash
    make pg-db-docker 
```

##### MySQL:
```bash
    make mysql-db-docker
```

#### Run database migrations:
- Install sqlx-cli:
```bash
    make install-sqlx-cli
```

##### Postgres SQL
- Run migrations:
```bash
    make run-postgres-migrations
```

- Revert migrations:
```bash
    make revert-postgres-migrations
```

##### MySQL
- Run migrations:
```bash
    make run-mysql-migrations
```

- Revert migrations:
```bash
    make revert-mysql-migrations
```

#### To test:
```bash
    make test
```

#### To test using tarpaulin:
- Install cargo-tarpaulin:
```bash
    make install-tarpaulin
```

- Run tests:
```bash
    make test-tarpaulin
```

#### To test using llvm-cov:
- Install llvm-cov:
```bash
    make install-llvm-cov
```

- Run tests:
```bash
    make test-llvm-cov
```

## CONTRIBUTIONS
The ***Page Hunter*** project is open source and therefore any interested software developer can contribute to its improvement. To contribute, take a look at the following recommendations:

- **Bug Reports**: If you find a bug, please create an issue detailing the problem, the steps to reproduce it, and the expected behavior.
- **Feature Requests**: If you have an idea for a new feature or an enhancement to an existing one, please create an issue describing your idea.
- **Pull Requests**: If you've fixed a bug or implemented a new feature, we'd love to see your work! Please submit a pull request. Make sure your code follows the existing style and all tests pass.
