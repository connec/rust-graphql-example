# `rust-graphql-sample`

A sample of GraphQL in Rust.

## Setup

1. [Get Rust](https://rustup.rs/).
2. Install [`sqlx-cli`](https://github.com/launchbadge/sqlx/blob/master/sqlx-cli/README.md)
   ```sh
   cargo install sqlx-cli --no-default-features --features postgres
   ```
3. `git clone https://github.com/connec/rust-graphql-sample`
4. `make run` – this will start a PostgreSQL container, prepare a sample database, and start the server.

[GraphiQL](https://github.com/graphql/graphiql) is served from http://localhost:3000/, and the GraphQL API itself from http://localhost:3000/query.
