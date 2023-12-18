use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::model::{Channel, Message};

#[async_trait]
pub trait ChannelRepository {
    async fn create(&self, name: String) -> Channel;
}

#[async_trait]
pub trait MessageRepository {
    async fn create(&self, text: String, channel_id: Uuid) -> Message;
}
