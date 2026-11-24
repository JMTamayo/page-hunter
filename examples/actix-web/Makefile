# Commands for the actix-web example. The service listens on 127.0.0.1:8080.

BASE_URL ?= http://127.0.0.1:8080

.DEFAULT_GOAL := help

help: ## Show this help.
	@awk 'BEGIN {FS = ":.*## "} /^[a-zA-Z_-]+:.*## / {printf "  %-22s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

run: ## Start the service in release mode.
	cargo run --release

run-dev: ## Start the service in debug mode.
	cargo run

check: ## Build-check the example.
	cargo check

lint: ## Run clippy with warnings as errors.
	cargo clippy -- -D warnings

try: ## Call the example endpoints (the service must be running).
	curl -s "$(BASE_URL)/departments/paged-list?page=0&size=3"; echo
	curl -s "$(BASE_URL)/departments/book?size=10" | head -c 300; echo

docs: ## Open the Swagger UI in the browser (macOS `open`; the service must be running).
	open "$(BASE_URL)/swagger-ui/"

.PHONY: help run run-dev check lint try docs
