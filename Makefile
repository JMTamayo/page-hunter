pg-db-docker:
	docker run --name postgres-db -e POSTGRES_PASSWORD=docker -e POSTGRES_DB=test -e POSTGRES_USER=test -p 5432:5432 -d postgres

mysql-db-docker:
	docker run --name mysql-db -e MYSQL_ROOT_PASSWORD=docker -e MYSQL_DATABASE=test -e MYSQL_USER=test -e MYSQL_PASSWORD=docker -p 3306:3306 -d mysql

install-sqlx-cli:
	cargo install sqlx-cli --no-default-features --features postgres,mysql,sqlite

install-tarpaulin:
	cargo install cargo-tarpaulin

install-llvm-cov:
	cargo install cargo-llvm-cov

run-postgres-migrations:
	sqlx migrate run --source page-hunter/tests/migrations/postgres --database-url postgres://test:docker@localhost:5432/test

revert-postgres-migrations:
	sqlx migrate revert --source page-hunter/tests/migrations/postgres --database-url postgres://test:docker@localhost:5432/test

run-mysql-migrations:
	sqlx migrate run --source page-hunter/tests/migrations/mysql --database-url mysql://test:docker@localhost:3306/test

revert-mysql-migrations:
	sqlx migrate revert --source page-hunter/tests/migrations/mysql --database-url mysql://test:docker@localhost:3306/test

fmt-check:
	cargo fmt --all --check

fmt:
	cargo fmt --all

doc-open:
	cargo doc --all-features --open

doc:
	cargo doc --all-features

check:
	cargo check --all-features

clippy:
	cargo clippy --all-features

test-tarpaulin:
	export $(shell cat local.env | xargs) && cargo tarpaulin --all-features --out Html --output-dir page-hunter/tests

test-llvm-cov:
	export $(shell cat local.env | xargs) && cargo llvm-cov --html --workspace --all-features

test:
	export $(shell cat local.env | xargs) && cargo test --all-features