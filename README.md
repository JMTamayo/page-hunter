# Page Hunter

<div align="left">
  <img src="https://img.shields.io/github/license/JMTamayo/page-hunter">
  <a href="https://deps.rs/repo/github/JMTamayo/page-hunter">
    <img src="https://deps.rs/repo/github/JMTamayo/page-hunter/status.svg">
  </a>
  <a href="https://github.com/JMTamayo/page-hunter/actions/workflows/ci.yml">
    <img src="https://github.com/JMTamayo/page-hunter/actions/workflows/ci.yml/badge.svg">
  </a>
  <a href="https://codecov.io/gh/JMTamayo/page-hunter">
    <img src="https://codecov.io/gh/JMTamayo/page-hunter/graph/badge.svg?token=R1LAPNSV5J">
  </a>
  <a href="https://crates.io/crates/page-hunter">
    <img src="https://img.shields.io/crates/v/page-hunter.svg?label=crates.io&color=orange&logo=rust">
  </a>
  <a href="https://docs.rs/page-hunter/latest/">
    <img src="https://img.shields.io/static/v1?label=docs.rs&message=latest&color=blue&logo=docsdotrs">
  </a>
</div>

Strong, reusable pagination models for Rust APIs and services.

`page-hunter` gives you:

- A validated `Page<T>` model for one page of records.
- A `Book<T>` model for full paging snapshots.
- Helpers for in-memory collections and SQLx queries.
- Optional `serde` and `utoipa` support for API contracts.

## Why Page Hunter?

Most projects eventually build pagination by hand, then repeat the same logic in multiple services.
`page-hunter` turns that into a single, tested abstraction.

- Consistent metadata (`page`, `size`, `total`, `pages`, `previous_page`, `next_page`).
- Validation rules enforced by construction.
- One API that works with records in memory and SQLx queries.

## Quick Start

### 1) Add dependency

```bash
cargo add page-hunter
```

With optional features:

```bash
cargo add page-hunter --features serde
cargo add page-hunter --features utoipa
cargo add page-hunter --features sqlx
```

### 2) Paginate a `Vec<T>` in 30 seconds

```rust,no_run
use page_hunter::prelude::*;

fn main() {
    let records = vec![1_u32, 2, 3, 4, 5];
    let page: Page<u32> = paginate_records(&records, 0, 2).unwrap();

    assert_eq!(page.get_items(), &vec![1, 2]);
    assert_eq!(page.get_pages(), 3);
    assert_eq!(page.get_next_page(), Some(1));
}
```

`prelude` is optional. You can always import each item manually.

## Core Models

### `Page<T>`

Represents one page of records, including:

- `items`
- `page`
- `size`
- `total`
- `pages`
- `previous_page`
- `next_page`

Create directly when you already know values:

```rust,no_run
use page_hunter::{Page, PaginationResult};

let items = vec![1, 2];
let page_model: PaginationResult<Page<u32>> = Page::new(&items, 0, 2, 5);
```

### `Book<T>`

Represents a full set of pages:

```rust,no_run
use page_hunter::{Book, Page};

let sheets: Vec<Page<u32>> = vec![
    Page::new(&vec![1, 2], 0, 2, 5).unwrap(),
    Page::new(&vec![3, 4], 1, 2, 5).unwrap(),
    Page::new(&vec![5], 2, 2, 5).unwrap(),
];

let book: Book<u32> = Book::new(&sheets);
assert_eq!(book.get_sheets().len(), 3);
```

## Common Use Cases

### Paginate records (`Page<T>`)

```rust,no_run
use page_hunter::{Page, RecordsPagination, paginate_records};

let records = vec![10, 20, 30, 40, 50];
let page = 1;
let size = 2;

let a: Page<i32> = paginate_records(&records, page, size).unwrap();
let b: Page<i32> = records.paginate(page, size).unwrap();

assert_eq!(a.get_items(), b.get_items());
```

### Bind all records (`Book<T>`)

```rust,no_run
use page_hunter::{Book, RecordsPagination, bind_records};

let records = vec![1, 2, 3, 4, 5];

let a: Book<i32> = bind_records(&records, 2).unwrap();
let b: Book<i32> = records.bind(2).unwrap();

assert_eq!(a.get_sheets().len(), b.get_sheets().len());
```

### SQLx pagination (`sqlx` feature)

`SQLxPagination::paginate` accepts either:

- `&Pool<DB>`
- `&mut DB::Connection`

#### Using a connection pool

```rust,no_run
use page_hunter::{Page, SQLxPagination};
use sqlx::postgres::{PgPool, Postgres};
use sqlx::{FromRow, QueryBuilder};

#[derive(Clone, Debug, FromRow)]
struct Country {
    id: i32,
    name: String,
}

async fn run(pool: PgPool) {
    let query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM geo.countries");
    let page: Page<Country> = query.paginate(&pool, 0, 10).await.unwrap();
    assert_eq!(page.get_page(), 0);
}
```

#### Using a single connection

```rust,no_run
use page_hunter::{Page, SQLxPagination};
use sqlx::postgres::{PgConnection, Postgres};
use sqlx::{Connection, FromRow, QueryBuilder};

#[derive(Clone, Debug, FromRow)]
struct Country {
    id: i32,
    name: String,
}

async fn run() {
    let mut conn = PgConnection::connect("postgres://username:password@localhost/db")
        .await
        .unwrap();
    let query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM geo.countries");

    let page: Page<Country> = query.paginate(&mut conn, 0, 10).await.unwrap();
    assert_eq!(page.get_size(), 10);
}
```

## Feature Flags

| Feature | What you get |
| --- | --- |
| `serde` | `Serialize` / `Deserialize` for `Page` and `Book` |
| `utoipa` | OpenAPI schemas via `ToSchema` (includes `serde`) |
| `sqlx` | SQL query pagination support for Postgres/MySQL/SQLite via SQLx |

## Validation Rules

Every `Page` is validated when created (or deserialized). Some important rules:

- `pages` must match `total` and `size` (`div_ceil`, minimum 1).
- `page` must be within range `[0, pages - 1]`.
- Non-last pages must have `items.len() == size`.
- Last page must satisfy total consistency.
- `previous_page` and `next_page` must be coherent.

If validation fails, you get a `PaginationError`.

## Typical Developer Commands

### Format and lint

```bash
cargo fmt --package page-hunter --all
cargo fmt --package page-hunter --all --check
cargo clippy --package page-hunter --all-features
```

### Build and test

```bash
cargo check --package page-hunter --all-features
cargo test --package page-hunter --all-features
cargo test --package page-hunter --all-features --doc
```

### Generate docs

```bash
cargo doc --package page-hunter --all-features --open
```

### Security scan

```bash
cargo deny --log-level error check
```

### Run full CI locally

Use the local helper script to run the same core flow as the GitHub CI (format, clippy matrix, check matrix, docs, tests, coverage, and security):

```bash
bash scripts/ci-local.sh
```

Optional shortcuts:

```bash
SKIP_DB=true bash scripts/ci-local.sh
SKIP_COVERAGE=true bash scripts/ci-local.sh
```

## Local SQLx Test Setup

Create `local.env` at workspace root:

```text
DB_HOST=localhost
DB_USER=test
DB_PASSWORD=docker
DB_NAME=test
PG_DB_PORT=5432
PG_MIGRATIONS_PATH=page-hunter/src/pagination/sqlx/tests/pg/migrations
```

Run test DB and migrations:

```bash
bash ./page-hunter/src/pagination/sqlx/tests/pg/scripts/run_db.sh
bash ./page-hunter/src/pagination/sqlx/tests/pg/scripts/run_migrations.sh
```

Revert migrations:

```bash
bash ./page-hunter/src/pagination/sqlx/tests/pg/scripts/revert_migration.sh
```

## Examples

Real projects live in [examples/](https://github.com/JMTamayo/page-hunter/tree/main/examples):

- [examples/actix-web](https://github.com/JMTamayo/page-hunter/tree/main/examples/actix-web): paginate external API results.
- [examples/axum](https://github.com/JMTamayo/page-hunter/tree/main/examples/axum): SQLx + PostgreSQL + OpenAPI.

## Contributing

Contributions are welcome.

- Report bugs with reproduction steps.
- Open feature proposals with clear use cases.
- Send PRs with tests and consistent style.

If you are building APIs in Rust and want predictable pagination behavior, `page-hunter` is ready for production workflows.
