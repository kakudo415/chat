mod controller;
mod domain;
mod infrastructure;
mod usecase;

use axum::{
    routing::{get, post},
    Router,
};
use controller::handler::ChannelState;
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};

use infrastructure::database::{PostgresChannelRepository, PostgresMessageRepository};
use usecase::{
    channel::CreateChannelUseCase,
    message::{ListMessagesUsecase, SendMessageUsecase},
};

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("error: DATABASE_URL is missing.");
    let pgpool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&database_url)
        .await
        .unwrap();

    let channel_repository = PostgresChannelRepository::new(pgpool.clone());
    let create_channel_usecase = CreateChannelUseCase::new(channel_repository);

    let message_repository = PostgresMessageRepository::new(pgpool);
    let send_message_usecase = SendMessageUsecase::new(message_repository.clone());
    let list_messages_usecase = ListMessagesUsecase::new(message_repository);

    let channel_state = Arc::new(ChannelState::new(
        create_channel_usecase,
        send_message_usecase,
        list_messages_usecase,
    ));

    let channel_routes = Router::new()
        .route("/", post(controller::handler::create_channel))
        .route(
            "/:channel_id/messages",
            get(controller::handler::list_messages).post(controller::handler::send_message),
        )
        .with_state(channel_state);

    let routes = Router::new().nest("/channels", channel_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
