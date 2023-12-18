use async_trait::async_trait;

use super::model::Channel;

#[async_trait]
pub trait ChannelRepository {
    async fn create(&self, name: String) -> Channel;
}
