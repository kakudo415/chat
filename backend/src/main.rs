mod controller;
mod domain;
mod infrastructure;
mod usecase;

use axum::{routing::post, Router};
use controller::handler::ChannelState;
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};

use infrastructure::database::PostgresChannelRepository;
use usecase::channel::CreateChannelUseCase;

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("error: DATABASE_URL is missing.");
    let pgpool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&database_url)
        .await
        .unwrap();

    let channel_repository = PostgresChannelRepository::new(pgpool);
    let create_channel_usecase = CreateChannelUseCase::new(channel_repository);
    let channel_state = Arc::new(ChannelState::new(create_channel_usecase));

    let channel_routes = Router::new()
        .route("/", post(controller::handler::create_channels))
        .with_state(channel_state);

    let routes = Router::new().nest("/channels", channel_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
