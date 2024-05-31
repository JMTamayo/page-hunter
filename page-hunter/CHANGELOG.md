# Changelog  🗒️

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 🚀 v0.2.0 [2024-XX-XX]

### Added:

- 🧑🏻‍💻 Implement `ErrorKind::SQLxError` when ***mysql-sqlx*** or ***pg-sqlx*** features are enabled **[BREAKING CHANGE]**.
- 🧑🏻‍💻 Implement `is_sqlx_error()` method for `ErrorKind` according to its new structure  when ***mysql-sqlx*** or ***pg-sqlx*** features are enabled **[BREAKING CHANGE]**.
- 🧑🏻‍💻 Implement **From**<**sqlx::Error**> for `PaginationError` when ***mysql-sqlx*** or ***pg-sqlx*** features are enabled. 

### Changed:

- 🔨 **Derive** and **Display** implementations for `PaginationError` and `ErrorKind` according to the new `ErrorKind` structure **[BREAKING CHANGE]**.

### Removed:

- ❌ Remove `ErrorKind::DatabaseError` and `ErrorKind::FromRowError` when ***mysql-sqlx*** or ***pg-sqlx*** features are enabled **[BREAKING CHANGE]**.
- ❌ Remove `is_database_error()` and `is_from_row_error()` methods from `ErrorKind` according to its new structure **[BREAKING CHANGE]**.
- ❌ Remove **Clone** implementation for `PaginationError` and `ErrorKind` **[BREAKING CHANGE]**.

## 🚀 v0.1.4 [2024-05-29]

### Added:

- 🧑🏻‍💻 Implement **utoipa::ToSchema** for `Page` and `Book`.  Only available when ***utoipa*** feature is enabled.
- 🧑🏻‍💻 Implement examples folder.

### Fixed:

- 🪚 Fix external crates importation for module sqlx_pagination.rs.

### Docs:

- 📝 Update project documentation.

## 🚀 v0.1.3 [2024-05-24]

### Added:

- 🧑🏻‍💻 Implement `ErrorKind::FromRowError`.
- 🧑🏻‍💻 Implement DB migrations with sqlx for the creation of the MySQL test database.
- 🧑🏻‍💻 Implement `SqlxPagination` to generalize the implementation of pagination methods using *sqlx*.
- 🧑🏻‍💻 Implement `SqlxPagination` for `QueryBuilder<MySQL>` to paginate results from a SQL query into a `Page`. Only available when ***mysql-sqlx*** feature is enabled.
- 🧑🏻‍💻 Implement `SqlxPagination` for `QueryBuilder<Postgres>` to paginate results from a SQL query into a `Page`. Only available when ***pg-sqlx*** feature is enabled.
- 🧑🏻‍💻 Implement integration test for pagination with ***mysql-sqlx***.

### Changed:

- 🔨 Refactor of the pagination.rs module to create the records_pagination.rs and sqlx_pagination.rs modules. Renaming the test module test_sqlx_postgres_pagination.rs to test_sqlx_pagination.rs.

### Removed:

- ❌ Remove **From**<**sqlx::Error**> for `PaginationError`.

### Docs:

- 📝 Update project documentation.

## 🚀 v0.1.2 [2024-05-22]

### Added:

- 🧑🏻‍💻 Implement `ErrorKind::DatabaseError`.
- 🧑🏻‍💻 Implement **From** **sqlx::Error** for `PaginationError`.
- 🧑🏻‍💻 Paginate results from a SQL query into a `Page` from a PostgreSQL database using *sqlx*. Implementation of the `PgSqlxPagination` for `QueryBuilder`. Only available when ***pg-sqlx*** feature is enabled.
- 🧑🏻‍💻 Include unitary test for the **Debug** implementation for `Book`.
- 🧑🏻‍💻 Include checking project format in ci.yml.
- 🧑🏻‍💻 Implement DB migrations with *sqlx* for the creation of the postgres test database.
- 🧑🏻‍💻 Implement integration tests for pagination with ***pg-sqlx***.

### Changed:

- 🔨 Change the implementation of **Clone** and **Debug** using derive to implement directly in `ErrorKind` and `PaginationError`.
- 🔨 Change the implementation of **Clone** and **Debug** using derive to implement directly in `Page` and `Book`.
- 🔨 Update unit tests to get 100% coverage on the errors module.

### Docs:

- 📝 Update project documentation to include new implementations, utilities and development section.

## 🚀 v0.1.1 [2024-05-18]

### Added:

- 🧑🏻‍💻 Implement `Book` model for binding uses.
- 🧑🏻‍💻 Implement **Default**, **Clone**, **Debug**, **Display** and **IntoIterator** in `Book` model for default feature.
- 🧑🏻‍💻 Implement **Serialize** and **Deserialize** in `Book` model for feature ***serde***.
- 🧑🏻‍💻 Implement `bind_records()` function for binding uses.
- 🧑🏻‍💻 Implement unitary tests for `bind_records()` and `Book` model.
- 🧑🏻‍💻 Codecov implementation to verify 90% of coverage in unit tests.

### Changed:

- 🔨 Change the implementation of **Serialize** trait using derive to implement directly in `Page` and `Book` models.

### Docs:

- 📝 Include badges in README.md
- 📝 Fix minor typos from documentation in traits implementation.


## 🚀 v0.1.0 [2024-05-15]

### Added:

- 🧑🏻‍💻 Implement `Page` model for pagination uses.
- 🧑🏻‍💻 Implementat **Default**, **Clone**, **Debug**, **Display** and **IntoIterator** in `Page` model for default feature.
- 🧑🏻‍💻 Implement **Serialize** and **Deserialize** in `Page` model for feature ***serde***.
- 🧑🏻‍💻 Implement `paginate_records()` function for record pagination.
- 🧑🏻‍💻 Implement unitary tests for `paginate_records()` and `Page` model.
- 🧑🏻‍💻 Implement CI workflow with GitHub actions to verify unit testing.

### Docs:

- 📝 Implement project documentation in README.md file, functions, methods, implementations, models and library.
- 📝 Implement CHANGELOG.md file.