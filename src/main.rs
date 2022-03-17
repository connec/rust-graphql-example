mod db;
mod gql;
mod model;

use std::{
    net::{Ipv4Addr, SocketAddr, TcpListener},
    sync::Arc,
};

use axum::{
    body::Body,
    http::Request,
    routing::{get, post},
    Router,
};
use juniper::{EmptySubscription, RootNode};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use self::{
    db::Db,
    gql::{Context, Mutation, Query},
};

type Tx = axum_sqlx_tx::Tx<sqlx::Postgres>;

#[derive(Debug, serde::Deserialize)]
struct Config {
    database_url: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config: Config = envy::from_env().unwrap();
    let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();

    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(axum_sqlx_tx::Layer::new(pool));

    let root_node = Arc::new(RootNode::new(Query, Mutation, EmptySubscription::new()));

    let app = Router::new()
        .route(
            "/",
            get(|| async { juniper_hyper::graphiql("/query", None).await }),
        )
        .route(
            "/query",
            post(|tx: Tx, request: Request<Body>| async {
                let context = Context::new(Db::new(tx));
                juniper_hyper::graphql(root_node, Arc::new(context), request).await
            }),
        )
        .layer(middleware);

    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
    let listener = TcpListener::bind(address).unwrap();

    info!("Listening on {}", address);

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}
