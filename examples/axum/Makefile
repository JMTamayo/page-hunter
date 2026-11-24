# Database settings used by the commands below; keep them in sync with `local.env`.
DB_PORT ?= 5432

.DEFAULT_GOAL := help

help: ## Show this help.
	@awk 'BEGIN {FS = ":.*## "} /^[a-zA-Z_-]+:.*## / {printf "  %-22s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

run-db-container: ## Start the PostgreSQL container (postgres:16) on 127.0.0.1, port DB_PORT (default 5432).
	docker run --name postgres-db-dev -e POSTGRES_PASSWORD=pswrd -e POSTGRES_DB=supermarket -e POSTGRES_USER=usr -p 127.0.0.1:$(DB_PORT):5432 -d postgres:16

remove-db-container: ## Stop and remove the PostgreSQL container.
	docker rm -f postgres-db-dev

install-sqlx-cli: ## Install sqlx-cli (only needed to revert migrations).
	cargo install sqlx-cli --no-default-features --features postgres

revert-db-migrations: ## Revert the last database migration.
	sqlx migrate revert --source src/db/migrations --database-url postgres://usr:pswrd@localhost:$(DB_PORT)/supermarket

run: ## Start the service in release mode (migrations run at startup).
	cargo run --release

.PHONY: help run-db-container remove-db-container install-sqlx-cli revert-db-migrations run
