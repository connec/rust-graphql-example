@PHONY: run

run:
	@RUST_LOG=rust_graphql_sample=debug,tower_http=debug cargo run
