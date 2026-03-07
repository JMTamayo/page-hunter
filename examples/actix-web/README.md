# Actix Web Example

This example shows how to use `page-hunter` with [actix-web](https://docs.rs/actix-web/latest/actix_web/) for in-memory pagination APIs.

It consumes data from [API Colombia](https://api-colombia.com) and demonstrates:

- `paginate_records` for paged responses.
- `bind_records` for grouped "book" responses.
- OpenAPI documentation with `utoipa`.

## Prerequisites

- Rust toolchain (stable)
- Internet connection (the app depends on API Colombia)

## Run locally

From this directory:

```bash
cargo run --release
```

The server starts at `http://127.0.0.1:8080`.

## API documentation

- Swagger UI: `http://localhost:8080/swagger-ui/`
- RapiDoc: `http://localhost:8080/rapidoc`
- Redoc: `http://localhost:8080/redoc`
- Scalar: `http://localhost:8080/scalar`

## Useful endpoints

- `GET /departments/paged-list?page=1&size=10`
- `GET /departments/book?size=10`