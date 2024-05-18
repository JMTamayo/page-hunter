# Changelog  🗒️

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 🚀 v0.1.1 [2024-05-18]

### Added:

- 🧑🏻‍💻 Implementation of `Book` model for binding uses.
- 🧑🏻‍💻 Implementation of **Default**, **Clone**, **Debug**, **Display** and **IntoIterator** in `Book` model for default feature.
- 🧑🏻‍💻 Implementation of **Serialize** and **Deserialize** in `Book` model for feature **serde**.
- 🧑🏻‍💻 Implementation of `bind_records()` function for binding uses.
- 🧑🏻‍💻 Implementation of unitary tests for `bind_records()` and `Book` model.
- 🧑🏻‍💻 Codecov implementation to verify 90% of coverage in unit tests.

### Changed:

- 🔨 Changing the implementation of Serialize trait using derive to implement directly in `Page` and `Book` models.

### Docs:

- 📝 Including badges in README.md
- 📝 Fixing minor typos from documentation in traits implementation.


## 🚀 v0.1.0 [2024-05-15]

### Added:

- 🧑🏻‍💻 Implementation of `Page` model for pagination uses.
- 🧑🏻‍💻 Implementation of **Default**, **Clone**, **Debug**, **Display** and **IntoIterator** in `Page` model for default feature.
- 🧑🏻‍💻 Implementation of **Serialize** and **Deserialize** in `Page` model for feature **serde**.
- 🧑🏻‍💻 Implementation of `paginate_records()` function for record pagination.
- 🧑🏻‍💻 Implementation of unitary tests for `paginate_records()` and `Page` model.
- 🧑🏻‍💻 Implementation of CI workflow with GitHub actions to verify unit testing.

### Docs:

- 📝 Implementation of project documentation in README.md file, functions, methods, implementations, models and library.
- 📝 Implementation of CHANGELOG.md file.