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

    let db = Db::connect(&config.database_url).await.unwrap();
    let context = Arc::new(Context::new(db));

    let root_node = Arc::new(RootNode::new(Query, Mutation, EmptySubscription::new()));

    let middleware = ServiceBuilder::new().layer(TraceLayer::new_for_http());

    let app = Router::new()
        .route(
            "/",
            get(|| async { juniper_hyper::graphiql("/query", None).await }),
        )
        .route(
            "/query",
            post(|request: Request<Body>| async {
                juniper_hyper::graphql(root_node, context, request).await
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
