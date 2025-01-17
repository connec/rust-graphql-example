@PHONY: prepare-db start-db stop-db run

export PGHOST ?= localhost
export PGPORT ?= 5432
export PGUSER ?= postgres
export PGPASSWORD ?= password
export PGDATABASE ?= sample
export DATABASE_URL ?= postgres://$(PGUSER):$(PGPASSWORD)@$(PGHOST):$(PGPORT)/$(PGDATABASE)
export RUST_LOG ?= rust_graphql_sample=debug,tower_http=debug

prepare-db: start-db
	@sqlx database create
	@sqlx migrate run
# 	Write a .env file with DATABASE_URL, so that sqlx will always pick it up (e.g. from editor or language server)
	@echo "DATABASE_URL=$(DATABASE_URL)" > .env

start-db:
	@scripts/start-db.sh

stop-db:
	@scripts/stop-db.sh

run: start-db prepare-db
	@cargo run
