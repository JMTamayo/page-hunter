# Actix Web Example

This example shows how to use `page-hunter` with [actix-web](https://docs.rs/actix-web/latest/actix_web/) for in-memory pagination APIs.

It consumes data from [API Colombia](https://api-colombia.com) and demonstrates:

- `paginate_records` for paged responses.
- `bind_records` for grouped "book" responses.
- OpenAPI documentation with `utoipa`.

## Prerequisites

- Rust 1.88 or newer (required by `utoipa` 6)
- Internet connection (the app depends on API Colombia)
- `make` (optional, for the shortcuts below)

## Run locally

From this directory:

```bash
make run
```

The server starts at `http://127.0.0.1:8080`. In another terminal, `make try` calls the example
endpoints and `make docs` opens the Swagger UI.

Other shortcuts: `make run-dev` (debug build), `make check` and `make lint`.

## API documentation

- Swagger UI: `http://localhost:8080/swagger-ui/`
- RapiDoc: `http://localhost:8080/rapidoc`
- Redoc: `http://localhost:8080/redoc`
- Scalar: `http://localhost:8080/scalar`

## Useful endpoints

- `GET /departments/paged-list?page=0&size=10`
- `GET /departments/book?size=10`

`page` starts at 0. A `size` of 0 or a `page` beyond the last one returns `417` with the
pagination error.