# Install the required tools to run databases, manage migrations, and run tests
install-tools:
	cargo install sqlx-cli --no-default-features --features postgres,mysql,sqlite && \
	cargo install cargo-llvm-cov

# Manage PostgreSQL database:
## Start the PostgreSQL database in a Docker container
pg-db-docker:
	bash ./page-hunter/tests/migrations/postgres/scripts/run_db.sh

## Run the PostgreSQL migrations
run-postgres-migrations:
	bash ./page-hunter/tests/migrations/postgres/scripts/run_migrations.sh

## Revert the last PostgreSQL migration
revert-postgres-migration:
	bash ./page-hunter/tests/migrations/postgres/scripts/revert_migration.sh

# Manage MySQL database:
## Start the MySQL database in a Docker container
mysql-db-docker:
	bash ./page-hunter/tests/migrations/mysql/scripts/run_db.sh

## Run the MySQL migrations
run-mysql-migrations:
	bash ./page-hunter/tests/migrations/mysql/scripts/run_migrations.sh

## Revert the last MySQL migration
revert-mysql-migration:
	bash ./page-hunter/tests/migrations/mysql/scripts/revert_migration.sh

# Manage SQLite database:
## Create local SQLite database file
sqlite-db-local:
	bash ./page-hunter/tests/migrations/sqlite/scripts/run_db.sh

## Run the SQLite migrations
run-sqlite-migrations:
	bash ./page-hunter/tests/migrations/sqlite/scripts/run_migrations.sh

## Revert the last SQLite migration
revert-sqlite-migrations:
	bash ./page-hunter/tests/migrations/sqlite/scripts/revert_migrations.sh

# Check the rust code:
## Check all features
check:
	cargo check --all-features

## Check feature serde
check-serde:
	cargo check --features serde

## Check feature utoipa
check-utoipa:
	cargo check --features utoipa

## Check feature pg-sqlx
check-pg-sqlx:
	cargo check --features pg-sqlx

## Check feature mysql-sqlx
check-mysql-sqlx:
	cargo check --features mysql-sqlx

## Check feature sqlite-sqlx
check-sqlite-sqlx:
	cargo check --features sqlite-sqlx

# Code formatting and linting:
## Format the code
fmt:
	cargo fmt --all

## Check the code formatting
fmt-check:
	cargo fmt --all --check

## Run the linter for all features
clippy:
	cargo clippy --all-features

# Documentation:
## Create and open the documentation
doc-open:
	cargo doc --all-features --open

## Create the documentation
doc:
	cargo doc --all-features

# Run the tests:
## Run the tests with coverage
test-llvm-cov:
	cargo llvm-cov --html --workspace --all-features

## Run the tests
test:
	cargo test --all-features