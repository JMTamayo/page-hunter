# Changelog  🗒️

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 🚀 Unreleased [Pending]

### Changed:

- 🔨 Remove Makefile from project by [@JMTamayo](https://github.com/JMTamayo).

### Docs:

- 📝 Remove makefile commands from README.md by [@JMTamayo](https://github.com/JMTamayo).

## 🚀 v0.5.0 [Pending]

### Changed:

- 🔨 `SqlxPagination` requires a single connection and not a complete connection pool **[⚠️ BREAKING CHANGE]** by [@JMTamayo](https://github.com/JMTamayo).

## 🚀 v0.4.2 [2025-01-11]

### Changed:

- 🔨 Remove assets from repo by [@JMTamayo](https://github.com/JMTamayo).

## 🚀 v0.4.1 [2025-01-11]

### Fixed:

- 🪚 Fix README.md posting on crates.io by [@JMTamayo](https://github.com/JMTamayo).

## 🚀 v0.4.0 [2025-01-11]

### Added:

- 🧑🏻‍💻 Implement `RecordsPagination` to paginate and bind records into a `Page` and `Book` by [@JMTamayo](https://github.com/JMTamayo).

### Changed:

- 🔨 Include unit tests inside each module by [@JMTamayo](https://github.com/JMTamayo).
- 🔨 Implement `SqlxPagination` in a more general way that applies to PostgreSQL, MySQL, and SQLite. Unify ***pg-sqlx***, ***mysql-sqlx*** and ***sqlite-sqlx*** features into ***sqlx*** feature **[⚠️ BREAKING CHANGE]** by [@JMTamayo](https://github.com/JMTamayo).
- 🔨 Upgrade ***utoipa*** version to greater than or equal to **0.5.3** by [@JMTamayo](https://github.com/JMTamayo).
- 🔨 Upgrade ***serde*** version to greater than or equal to **1.0.210** by [@JMTamayo](https://github.com/JMTamayo).
- 🔨 Optimize the implementation of the `Debug`, `Clone`, `Serialize` and `ToSchema` traits by [@JMTamayo](https://github.com/JMTamayo).

## 🚀 v0.3.1 [2025-01-05]

### Fixed:

- 🪚 Restrict ***utoipa*** version to greater than or equal to **0.4.2** and less than **0.5.0** by [@JMTamayo](https://github.com/JMTamayo).

### Changed:

- 🔨 Improve Cargo.toml file format [@JMTamayo](https://github.com/JMTamayo).

## 🚀 v0.3.0 [2024-10-13]

### Added:

- 🧑🏻‍💻 Implement `SqlxPagination` for `QueryBuilder<Sqlite>` to paginate results from a SQL query into a `Page`. Only available when ***sqlite-sqlx*** feature is enabled by [@JMTamayo](https://github.com/JMTamayo).

### Fixed:

- 🪚 Update *sqlx* version to **>=0.8.1** to prevent issue [RUSTSEC-2024-0363](https://rustsec.org/advisories/RUSTSEC-2024-0363.html) 🚨 by [@JosiahParry](https://github.com/JosiahParry).

### Changed:

- 🔨 Rename `ErrorKind::FieldValueError` to `ErrorKind::InvalidValue` **[⚠️ BREAKING CHANGE]** by [@JMTamayo](https://github.com/JMTamayo).
- 🔨 Rename `ErrorKind::SQLxError` to `ErrorKind::SQLx` **[⚠️ BREAKING CHANGE]** by [@JMTamayo](https://github.com/JMTamayo).
- 🔨 Rename method `is_field_value_error()` to `is_invalid_value_error()` in `ErrorKind` **[⚠️ BREAKING CHANGE]** by [@JMTamayo](https://github.com/JMTamayo).
- 🔨 Redefine the package structure into new modules, while maintaining the way each artifact is imported from lib by [@JMTamayo](https://github.com/JMTamayo).

### Docs:

- 📝 Update project documentation to include feature ***sqlite-sqlx*** by [@JMTamayo](https://github.com/JMTamayo).


## 🚀 v0.2.0 [2024-06-01]

### Added:

- 🧑🏻‍💻 Implement `ErrorKind::SQLxError` when ***mysql-sqlx*** or ***pg-sqlx*** features are enabled **[⚠️ BREAKING CHANGE]** by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement `is_sqlx_error()` method for `ErrorKind` according to its new structure  when ***mysql-sqlx*** or ***pg-sqlx*** features are enabled **[⚠️ BREAKING CHANGE]** by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement **From**<**sqlx::Error**> for `PaginationError` when ***mysql-sqlx*** or ***pg-sqlx*** features are enabled by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻  Including example of use with **axum** by [@JMTamayo](https://github.com/JMTamayo).

### Changed:

- 🔨 **Derive** and **Display** implementations for `PaginationError` and `ErrorKind` according to the new `ErrorKind` structure **[⚠️ BREAKING CHANGE]** by [@JMTamayo](https://github.com/JMTamayo).
- 🔨 Rename `SqlxPagination` to `SQLxPagination` **[⚠️ BREAKING CHANGE]** by [@JMTamayo](https://github.com/JMTamayo).

### Removed:

- ❌ Remove `ErrorKind::DatabaseError` and `ErrorKind::FromRowError` when ***mysql-sqlx*** or ***pg-sqlx*** features are enabled **[⚠️ BREAKING CHANGE]** by [@JMTamayo](https://github.com/JMTamayo).
- ❌ Remove `is_database_error()` and `is_from_row_error()` methods from `ErrorKind` according to its new structure **[⚠️ BREAKING CHANGE]** by [@JMTamayo](https://github.com/JMTamayo).
- ❌ Remove **Clone** implementation for `PaginationError` and `ErrorKind` **[⚠️ BREAKING CHANGE]** by [@JMTamayo](https://github.com/JMTamayo).


## 🚀 v0.1.4 [2024-05-29]

### Added:

- 🧑🏻‍💻 Implement **utoipa::ToSchema** for `Page` and `Book`.  Only available when ***utoipa*** feature is enabled by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement examples folder by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Including example of use with **actix-web** by [@JMTamayo](https://github.com/JMTamayo).

### Fixed:

- 🪚 Fix external crates importation for module sqlx_pagination.rs by [@JMTamayo](https://github.com/JMTamayo).

### Docs:

- 📝 Update project documentation by [@JMTamayo](https://github.com/JMTamayo).


## 🚀 v0.1.3 [2024-05-24]

### Added:

- 🧑🏻‍💻 Implement `ErrorKind::FromRowError` by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement DB migrations with sqlx for the creation of the MySQL test database by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement `SqlxPagination` to generalize the implementation of pagination methods using *sqlx* by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement `SqlxPagination` for `QueryBuilder<MySQL>` to paginate results from a SQL query into a `Page`. Only available when ***mysql-sqlx*** feature is enabled by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement `SqlxPagination` for `QueryBuilder<Postgres>` to paginate results from a SQL query into a `Page`. Only available when ***pg-sqlx*** feature is enabled by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement integration test for pagination with ***mysql-sqlx*** by [@JMTamayo](https://github.com/JMTamayo).

### Changed:

- 🔨 Refactor of the pagination.rs module to create the records_pagination.rs and sqlx_pagination.rs modules. Renaming the test module test_sqlx_postgres_pagination.rs to test_sqlx_pagination.rs by [@JMTamayo](https://github.com/JMTamayo).

### Removed:

- ❌ Remove **From**<**sqlx::Error**> for `PaginationError` by [@JMTamayo](https://github.com/JMTamayo).

### Docs:

- 📝 Update project documentation by [@JMTamayo](https://github.com/JMTamayo).


## 🚀 v0.1.2 [2024-05-22]

### Added:

- 🧑🏻‍💻 Implement `ErrorKind::DatabaseError` by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement **From** **sqlx::Error** for `PaginationError` by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Paginate results from a SQL query into a `Page` from a PostgreSQL database using *sqlx*. Implementation of the `PgSqlxPagination` for `QueryBuilder`. Only available when ***pg-sqlx*** feature is enabled by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Include unitary test for the **Debug** implementation for `Book` by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Include checking project format in ci.yml by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement DB migrations with *sqlx* for the creation of the postgres test database by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement integration tests for pagination with ***pg-sqlx*** by [@JMTamayo](https://github.com/JMTamayo).

### Changed:

- 🔨 Change the implementation of **Clone** and **Debug** using derive to implement directly in `ErrorKind` and `PaginationError` by [@JMTamayo](https://github.com/JMTamayo).
- 🔨 Change the implementation of **Clone** and **Debug** using derive to implement directly in `Page` and `Book` by [@JMTamayo](https://github.com/JMTamayo).
- 🔨 Update unit tests to get 100% coverage on the errors module by [@JMTamayo](https://github.com/JMTamayo).

### Docs:

- 📝 Update project documentation to include new implementations, utilities and development section by [@JMTamayo](https://github.com/JMTamayo).


## 🚀 v0.1.1 [2024-05-18]

### Added:

- 🧑🏻‍💻 Implement `Book` model for binding uses by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement **Default**, **Clone**, **Debug**, **Display** and **IntoIterator** in `Book` model for default feature by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement **Serialize** and **Deserialize** in `Book` model for feature ***serde*** by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement `bind_records()` function for binding uses by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement unitary tests for `bind_records()` and `Book` model by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Codecov implementation to verify 90% of coverage in unit tests by [@JMTamayo](https://github.com/JMTamayo).

### Changed:

- 🔨 Change the implementation of **Serialize** trait using derive to implement directly in `Page` and `Book` models by [@JMTamayo](https://github.com/JMTamayo).

### Docs:

- 📝 Include badges in README.md by [@JMTamayo](https://github.com/JMTamayo).
- 📝 Fix minor typos from documentation in traits implementation by [@JMTamayo](https://github.com/JMTamayo).


## 🚀 v0.1.0 [2024-05-15]

### Added:

- 🧑🏻‍💻 Implement `Page` model for pagination uses by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implementat **Default**, **Clone**, **Debug**, **Display** and **IntoIterator** in `Page` model for default feature by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement **Serialize** and **Deserialize** in `Page` model for feature ***serde*** by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement `paginate_records()` function for record pagination by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement unitary tests for `paginate_records()` and `Page` model by [@JMTamayo](https://github.com/JMTamayo).
- 🧑🏻‍💻 Implement CI workflow with GitHub actions to verify unit testing by [@JMTamayo](https://github.com/JMTamayo).

### Docs:

- 📝 Implement project documentation in README.md file, functions, methods, implementations, models and library by [@JMTamayo](https://github.com/JMTamayo).
- 📝 Implement CHANGELOG.md file by [@JMTamayo](https://github.com/JMTamayo).