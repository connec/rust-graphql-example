@PHONY: start-db stop-db run

start-db:
	@scripts/start-db.sh

stop-db:
	@scripts/stop-db.sh

run: start-db
	@RUST_LOG=rust_graphql_sample=debug,tower_http=debug cargo run
