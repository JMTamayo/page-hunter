# Axum + SQLx Example

This example shows how to use `page-hunter` with [axum](https://docs.rs/axum/latest/axum/) and PostgreSQL through `sqlx`.

It demonstrates:

- SQL-backed pagination with `SQLxPagination`.
- API handlers for categories and products.
- OpenAPI docs with Swagger UI.

## Prerequisites

- Rust toolchain (stable)
- Docker
- `make`

## Run locally

From this directory:

1. Install `sqlx-cli`:

```bash
make install-sqlx-cli
```

2. Start PostgreSQL container:

```bash
make run-db-container
```

3. Run migrations:

```bash
make run-db-migrations
```

4. Export environment variables:

```bash
set -a
source local.env
set +a
```

5. Start the API:

```bash
make run
```

The server starts on `http://localhost:8080`.

## API documentation

- Swagger UI: `http://localhost:8080/swagger-ui/`

## Useful endpoints

- `GET /categories?page=1&size=10`
- `GET /products?page=1&size=10`
- `GET /products/{id}`

## Notes

- The default PostgreSQL mapping uses port `5432`.
- If you already have a local PostgreSQL running on `5432`, stop it or run the container with a different mapped port.