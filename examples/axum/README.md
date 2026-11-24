# Axum + SQLx Example

This example shows how to use `page-hunter` with [axum](https://docs.rs/axum/latest/axum/) and PostgreSQL through `sqlx`.

It demonstrates:

- SQL-backed pagination with `SQLxPagination`, using `push_bind` for every user-provided filter.
- API handlers for categories and products.
- OpenAPI docs with Swagger UI.

## Prerequisites

- Rust 1.94 or newer (required by `sqlx` 0.9)
- Docker
- `make`

## Run locally

From this directory (run `make` to list all the shortcuts):

1. Start a PostgreSQL container (`postgres:16`, bound to `127.0.0.1:5432`):

```bash
make run-db-container
```

If port `5432` is already in use, pick another one, e.g. `make run-db-container DB_PORT=55433`,
and use the same port in `DB_PORT` below.

2. Create a `local.env` file with the service settings:

```text
DB_HOST=localhost
DB_PORT=5432
DB_USERNAME=usr
DB_PASSWORD=pswrd
DB_DATABASE=supermarket
```

Optional variables: `PORT` (default `8080`), `LOG_LEVEL` (default `debug`), `SERVICE_NAME` and
`MAX_POOL_DB_CONNECTIONS` (default `10`).

3. Export the variables and start the API:

```bash
set -a && source local.env && set +a && make run
```

The database migrations (schema and sample data) run automatically when the service starts.
The server listens on `http://localhost:8080`.

## API documentation

- Swagger UI: `http://localhost:8080/swagger-ui/`

## Useful endpoints

- `GET /categories?page=0&size=10`
- `GET /products?page=0&size=10&name_like=milk`
- `GET /products/{id}`

`page` starts at 0. A `size` of 0 or a `page` beyond the last one returns `400 Bad Request`.

## Cleanup

```bash
make remove-db-container
```

To undo the migrations manually, install `sqlx-cli` (`make install-sqlx-cli`) and run
`make revert-db-migrations`.
