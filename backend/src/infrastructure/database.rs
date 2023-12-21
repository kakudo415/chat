use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::model::{Channel, Message};
use crate::domain::repository::{ChannelRepository, MessageRepository};

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

#[derive(Clone)]
pub struct PostgresMessageRepository {
    pool: Pool<Postgres>,
}

impl PostgresMessageRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        PostgresMessageRepository { pool }
    }
}

#[async_trait]
impl MessageRepository for PostgresMessageRepository {
    async fn create(&self, text: String, channel_id: Uuid) -> Message {
        let message = Message::new(text, channel_id);
        sqlx::query!(
            "INSERT INTO messages (id, text, channel_id) VALUES ($1, $2, $3)",
            message.id(),
            message.text(),
            message.channel_id(),
        )
        .execute(&self.pool)
        .await
        .unwrap();
        message
    }

    async fn list(&self, channel_id: Uuid) -> Vec<Message> {
        let rows = sqlx::query!("SELECT * FROM messages WHERE channel_id = $1", channel_id)
            .fetch_all(&self.pool)
            .await
            .unwrap();
        let mut messages = Vec::new();
        for row in rows {
            let message =
                Message::from_raw_parts(row.id, row.text, row.channel_id, row.created_at.and_utc());
            messages.push(message);
        }
        messages
    }
}
