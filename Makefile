# Install the required tools
install-tools:
	cargo install sqlx-cli --no-default-features --features postgres && \
	cargo install cargo-llvm-cov \
	cargo install cargo-nextest \
	cargo install cargo-deny

# Manage PostgreSQL database
## Start the PostgreSQL database in a Docker container:
run-pg-db-docker:
	bash ./page-hunter/src/pagination/sqlx/tests/pg/scripts/run_db.sh

## Run the PostgreSQL migrations:
run-pg-db-migrations:
	bash ./page-hunter/src/pagination/sqlx/tests/pg/scripts/run_migrations.sh

## Revert the last PostgreSQL migration:
revert-pg-db-migration:
	bash ./page-hunter/src/pagination/sqlx/tests/pg/scripts/revert_migration.sh

# Check the rust code
## Check all features:
check-all:
	cargo check --package page-hunter --all-features

## Check feature serde:
check-serde:
	cargo check --package page-hunter --features serde

## Check feature utoipa:
check-utoipa:
	cargo check --package page-hunter --features utoipa

## Check feature sqlx:
check-sqlx:
	cargo check --package page-hunter --features sqlx

# Code formatting and linters
## Format the code:
fmt-all:
	cargo fmt --package page-hunter --all

## Check the code formatting:
fmt-all-check:
	cargo fmt --package page-hunter --all --check

## Run the linter for all features:
clippy-all:
	cargo clippy --package page-hunter --all-features

## Run the linter for the feature serde:
clippy-serde:
	cargo clippy --package page-hunter --features serde

## Run the linter for the feature utoipa:
clippy-utoipa:
	cargo clippy --package page-hunter --features utoipa

## Run the linter for the feature sqlx:
clippy-sqlx:
	cargo clippy --package page-hunter --features sqlx

# Documentation
## Create and open the documentation:
doc-open:
	cargo doc --package page-hunter --all-features --open

## Create the documentation:
doc:
	cargo doc --package page-hunter --all-features

# Run the tests
## Run the tests with coverage and generate the report:
test-llvm-cov-report:
	cargo llvm-cov nextest --workspace --all-features --show-missing-lines --open

## Run the doctests:
doctests:
	cargo test --package page-hunter --all-features --doc

# Security
## Check the dependencies for vulnerabilities:
deny-check:
	cargo deny --log-level error check