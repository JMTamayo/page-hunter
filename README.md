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
cargo add sqlx --features postgres   # or mysql, sqlite
```

### 2) Paginate a `Vec<T>` in 30 seconds

```rust,no_run
use page_hunter::prelude::*;

fn main() {
    let records = vec![1_u32, 2, 3, 4, 5];
    let page: Page<u32> = paginate_records(&records, 0, 2).unwrap();

    assert_eq!(page.get_items(), &[1, 2]);
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
let page_model: PaginationResult<Page<u32>> = Page::new(items, 0, 2, 5);
```

### `Book<T>`

Represents a full set of pages:

```rust,no_run
use page_hunter::{Book, Page};

let sheets: Vec<Page<u32>> = vec![
    Page::new(vec![1, 2], 0, 2, 5).unwrap(),
    Page::new(vec![3, 4], 1, 2, 5).unwrap(),
    Page::new(vec![5], 2, 2, 5).unwrap(),
];

let book: Book<u32> = Book::new(sheets);
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

Records are read by reference and only the items of the requested page are cloned. Any
collection whose reference can be iterated works: `Vec`, slices, arrays, `VecDeque`, `HashSet`...
To paginate a range or an iterator, collect it into a `Vec` first.

### Bind all records (`Book<T>`)

```rust,no_run
use page_hunter::{Book, RecordsPagination, bind_records};

let records = vec![1, 2, 3, 4, 5];

let a: Book<i32> = bind_records(&records, 2).unwrap();
let b: Book<i32> = records.bind(2).unwrap();

assert_eq!(a.get_sheets().len(), b.get_sheets().len());
```

### SQLx pagination (`sqlx` feature)

`SQLxPagination::paginate` consumes a `QueryBuilder` and accepts either:

- `&Pool<DB>`
- `&mut DB::Connection`

Values added with `push_bind` are sent as query parameters to both the `count(*)` query and
the page query. Always use `push_bind` for user input instead of formatting it into the SQL.

Query requirements:

- Do not add `LIMIT` or `OFFSET`: `paginate` appends them.
- Add an `ORDER BY` on a unique key (e.g. `ORDER BY id`). Without it the database may return rows
  in a different order on each query, so a row can appear on two pages or on none.

Works with Postgres, MySQL and SQLite using `sqlx` 0.9 (Rust 1.94 or newer): enable the database
in your own `sqlx` dependency.

#### Using bound parameters

```rust,no_run
use page_hunter::{Page, SQLxPagination};
use sqlx::postgres::{PgPool, Postgres};
use sqlx::{FromRow, QueryBuilder};

#[derive(Clone, Debug, FromRow)]
struct Country {
    id: i32,
    name: String,
}

async fn run(pool: PgPool, name_like: String) {
    let mut query: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT * FROM geo.countries WHERE name ILIKE ");
    query.push_bind(format!("%{name_like}%")).push(" ORDER BY id");

    let page: Page<Country> = query.paginate(&pool, 0, 10).await.unwrap();
    assert!(page.get_items().len() <= 10);
}
```

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
    let query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM geo.countries ORDER BY id");
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
    let query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM geo.countries ORDER BY id");

    let page: Page<Country> = query.paginate(&mut conn, 0, 10).await.unwrap();
    assert_eq!(page.get_size(), 10);
}
```

## Feature Flags

| Feature | What you get |
| --- | --- |
| `serde` | `Serialize` / `Deserialize` for `Page` and `Book` |
| `utoipa` | OpenAPI schemas via `ToSchema` with utoipa 6 (includes `serde`, Rust 1.88+) |
| `sqlx` | SQL query pagination for Postgres/MySQL/SQLite via SQLx 0.9 (Rust 1.94+) |

## Validation Rules

Every `Page` is validated when created (or deserialized). Some important rules:

- `size` must be greater than 0.
- `pages` must match `total` and `size` (`div_ceil`, minimum 1).
- `page` must be within range `[0, pages - 1]`.
- Non-last pages must have `items.len() == size`.
- Last page must satisfy total consistency.
- `previous_page` and `next_page` must be coherent.

If validation fails, you get a `PaginationError`.

Paginating a collection or query without results returns a single empty page (`page` 0,
`total` 0, `pages` 1). Requesting a page beyond the last one returns an error.

The crate uses Rust edition 2024 (Rust 1.85 or newer). The `utoipa` feature needs Rust 1.88+ and
the `sqlx` feature needs Rust 1.94+, as required by those dependencies.

## Migrating from 0.7 to 0.8

- `Page::new` and `Book::new` take their `Vec` by value: `Page::new(items, page, size, total)`.
- `Page::get_items` returns `&[E]` and `Book::get_sheets` returns `&[Page<E>]`. Use
  `into_items` / `into_sheets` to take ownership of the contents.
- A `size` of 0 is now an error everywhere (`Page::new`, deserialization, `paginate_records`,
  `bind_records` and `SQLxPagination::paginate`).
- `Page` and `Book` no longer implement `Default` (the `Page` default had `size` 0, which is not
  a valid page). Build an empty page with `Page::new(vec![], 0, size, 0)`. Paginating or binding
  a query or collection without results already returns one empty page.
- `SQLxPagination::paginate` consumes the `QueryBuilder` and now sends `push_bind` values as
  parameters. Queries that interpolated values into the SQL should switch to `push_bind`.
- `paginate_records`, `bind_records` and `RecordsPagination` read the records by reference and
  clone only the selected items: they accept any collection whose reference can be iterated
  (`Vec`, slices, arrays, `VecDeque`...). `RecordsPagination` is now generic over the item type
  (`RecordsPagination<T>`). Ranges and iterators must be collected into a `Vec` first.
- `ErrorKind` is `#[non_exhaustive]`: a `match` on it needs a wildcard arm (`_ => ...`).
- `PaginationError` implements `std::error::Error`, and `Page` and `Book` implement `PartialEq`
  and `Eq` when their items do.
- The `sqlx` feature now uses `sqlx` 0.9: update your own `sqlx` dependency to 0.9 and your
  toolchain to Rust 1.94 or newer.
- The `utoipa` feature now uses `utoipa` 6 (Rust 1.88+): update your own `utoipa` dependency
  (and UI crates such as `utoipa-swagger-ui` 10) so `Page` and `Book` schemas fit in your `OpenApi`.

## Typical Developer Commands

The `Makefile` at the repository root wraps the most useful commands. Run `make` to list them:

| Command | What it does |
| --- | --- |
| `make fmt` / `make fmt-check` | Format the crate / check formatting |
| `make lint` | Clippy (including tests) with `-D warnings` on every CI feature combination |
| `make check` | `cargo check` on every CI feature combination |
| `make doc` / `make doc-open` | Build the docs / build and open them |
| `make test-unit` | Tests that need no database |
| `make test` | All tests against Postgres, MySQL and SQLite (starts the databases) |
| `make test-doc` | Doc tests |
| `make coverage` | All tests with coverage into `codecov.json` (needs `cargo-llvm-cov`) |
| `make deny` | `cargo-deny` checks (advisories, licenses, bans, sources) |
| `make examples` | Build-check the `actix-web` and `axum` examples |
| `make update` | Update `Cargo.lock` to the latest compatible versions |
| `make msrv` | Check the minimum Rust versions like CI (installs the toolchains with `rustup`) |
| `make ci` | The same steps as CI: format, lint, check, docs, tests and `cargo-deny` |
| `make db-up` / `make db-down` / `make db-status` | Start, remove or inspect the test databases |

## Local SQLx Test Setup

SQLite tests run in memory. Postgres and MySQL tests need running databases: `make test` and
`make ci` start them in Docker (`postgres:16` and `mysql:8.4`, bound to `127.0.0.1`). Each test
creates its own tables and data, so no migrations are required.

The settings come from an optional `local.env` at the repository root (these are the defaults):

```text
DB_HOST=127.0.0.1
DB_USER=test
DB_PASSWORD=docker
DB_NAME=test
PG_DB_PORT=5432
MYSQL_DB_PORT=3306
```

If a port is already in use, change it in `local.env` or on the command line:

```bash
make test PG_DB_PORT=55432
```

Remove the containers when you are done:

```bash
make db-down
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
