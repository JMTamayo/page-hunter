install-sqlx-cli:
	cargo install sqlx-cli --no-default-features --features postgres

run-db-container:
	docker run --name postgres-db-dev -e POSTGRES_PASSWORD=pswrd -e POSTGRES_DB=supermarket -e POSTGRES_USER=usr -p 5432:5432 -d postgres

run-db-migrations:
	 sqlx migrate run --source src/db/migrations --database-url postgres://usr:pswrd@localhost:5432/supermarket

revert-db-migrations:
	sqlx migrate revert --source src/db/migrations --database-url postgres://usr:pswrd@localhost:5432/supermarket

run:
	cargo run --release