# Developer commands for page-hunter. Run `make` or `make help` to list them.
#
# Database settings come from `local.env` when it exists (see README), with these defaults.

-include local.env

DB_HOST ?= 127.0.0.1
DB_USER ?= test
DB_PASSWORD ?= docker
DB_NAME ?= test
PG_DB_PORT ?= 5432
MYSQL_DB_PORT ?= 3306

export DB_HOST DB_USER DB_PASSWORD DB_NAME PG_DB_PORT MYSQL_DB_PORT

PACKAGE := --package page-hunter
PG_CONTAINER := page-hunter-pg
MYSQL_CONTAINER := page-hunter-mysql

# Feature combinations checked by CI.
FEATURE_MATRIX := "" "--all-features" "--features serde" "--features utoipa" "--features sqlx"

.DEFAULT_GOAL := help

.PHONY: help fmt fmt-check lint check doc doc-open test-unit test test-doc coverage deny \
	examples update msrv ci db-up db-down db-status

help: ## Show this help.
	@awk 'BEGIN {FS = ":.*## "} /^[a-zA-Z_-]+:.*## / {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

fmt: ## Format the crate.
	cargo fmt $(PACKAGE) --all

fmt-check: ## Check formatting without changing files.
	cargo fmt $(PACKAGE) --all --check

lint: ## Run clippy (-D warnings) on every feature combination of CI.
	@for flags in $(FEATURE_MATRIX); do \
		echo "==> cargo clippy $$flags"; \
		cargo clippy $(PACKAGE) $$flags --all-targets --locked -- -D warnings || exit 1; \
	done

check: ## Build-check every feature combination of CI.
	@for flags in $(FEATURE_MATRIX); do \
		echo "==> cargo check $$flags"; \
		cargo check $(PACKAGE) $$flags --locked || exit 1; \
	done

doc: ## Build the documentation with all features (warnings are errors, like CI).
	RUSTDOCFLAGS="-D warnings" cargo doc $(PACKAGE) --all-features --document-private-items --locked

doc-open: ## Build the documentation and open it in the browser.
	cargo doc $(PACKAGE) --all-features --open

test-unit: ## Run tests that need no database (no `sqlx` feature).
	cargo test $(PACKAGE) --features utoipa --locked

test-doc: ## Run the doc tests.
	cargo test $(PACKAGE) --all-features --doc --locked

test: db-up ## Run all tests against Postgres, MySQL and SQLite (starts the databases).
	cargo test $(PACKAGE) --all-features --locked

coverage: db-up ## Run all tests with coverage into codecov.json (needs cargo-llvm-cov).
	cargo llvm-cov $(PACKAGE) --all-features --locked --codecov --output-path codecov.json

deny: ## Check advisories, licenses, bans and sources with cargo-deny.
	cargo deny --manifest-path page-hunter/Cargo.toml check

examples: ## Build-check the actix-web and axum examples.
	cd examples/actix-web && cargo check
	cd examples/axum && cargo check

update: ## Update Cargo.lock to the latest compatible versions.
	cargo update

msrv: ## Check the minimum Rust versions (1.85, 1.88 with utoipa, 1.94 with sqlx) like CI.
	rustup toolchain install 1.85 1.88 1.94 --profile minimal
	cargo +1.85 check $(PACKAGE) --features serde --locked
	cargo +1.88 check $(PACKAGE) --features utoipa --locked
	cargo +1.94 check $(PACKAGE) --all-features --locked

ci: fmt-check lint check doc test-doc test deny ## Run the same steps as CI (starts the databases).

db-up: ## Start the Postgres and MySQL test containers and wait until they are ready.
	@if ! docker ps -a --format '{{.Names}}' | grep -qx $(PG_CONTAINER); then \
		echo "==> Creating $(PG_CONTAINER) on port $(PG_DB_PORT)"; \
		docker run -d --name $(PG_CONTAINER) -e POSTGRES_USER=$(DB_USER) \
			-e POSTGRES_PASSWORD=$(DB_PASSWORD) -e POSTGRES_DB=$(DB_NAME) \
			-p 127.0.0.1:$(PG_DB_PORT):5432 postgres:16 >/dev/null || exit 1; \
	else \
		docker start $(PG_CONTAINER) >/dev/null || exit 1; \
	fi
	@if ! docker ps -a --format '{{.Names}}' | grep -qx $(MYSQL_CONTAINER); then \
		echo "==> Creating $(MYSQL_CONTAINER) on port $(MYSQL_DB_PORT)"; \
		docker run -d --name $(MYSQL_CONTAINER) -e MYSQL_ROOT_PASSWORD=root \
			-e MYSQL_USER=$(DB_USER) -e MYSQL_PASSWORD=$(DB_PASSWORD) -e MYSQL_DATABASE=$(DB_NAME) \
			-p 127.0.0.1:$(MYSQL_DB_PORT):3306 mysql:8.4 >/dev/null || exit 1; \
	else \
		docker start $(MYSQL_CONTAINER) >/dev/null || exit 1; \
	fi
	@echo "==> Waiting for the databases"
	@for i in $$(seq 1 60); do \
		docker exec $(PG_CONTAINER) pg_isready -U $(DB_USER) -d $(DB_NAME) >/dev/null 2>&1 && \
		docker exec $(MYSQL_CONTAINER) mysqladmin ping -h 127.0.0.1 -u$(DB_USER) -p$(DB_PASSWORD) --silent >/dev/null 2>&1 && \
		{ echo "==> Databases ready"; exit 0; }; \
		sleep 2; \
	done; \
	echo "ERROR: the databases did not become ready in time"; exit 1

db-down: ## Stop and remove the test containers.
	-docker rm -f $(PG_CONTAINER) $(MYSQL_CONTAINER)

db-status: ## Show the state of the test containers.
	@docker ps -a --filter name=$(PG_CONTAINER) --filter name=$(MYSQL_CONTAINER) \
		--format 'table {{.Names}}\t{{.Status}}\t{{.Ports}}'
