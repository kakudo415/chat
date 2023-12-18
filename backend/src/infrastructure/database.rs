use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::model::Channel;
use crate::domain::repository::ChannelRepository;

#[derive(Clone)]
pub struct PostgresChannelRepository {
    pool: Pool<Postgres>,
}

impl PostgresChannelRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        PostgresChannelRepository { pool }
    }
}

#[async_trait]
impl ChannelRepository for PostgresChannelRepository {
    async fn create(&self, name: String) -> Channel {
        let channel = Channel::new(name);
        sqlx::query!(
            "INSERT INTO channels (id, name) VALUES ($1, $2)",
            channel.id(),
            channel.name(),
        )
        .execute(&self.pool)
        .await
        .unwrap();
        channel
    }
}
