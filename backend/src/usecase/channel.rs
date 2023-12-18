use crate::domain::{model::Channel, repository::ChannelRepository};

#[derive(Clone)]
pub struct CreateChannelUseCase<CR> {
    channel_repository: CR,
}

impl<CR> CreateChannelUseCase<CR>
where
    CR: ChannelRepository,
{
    pub fn new(channel_repository: CR) -> Self {
        CreateChannelUseCase { channel_repository }
    }

    pub async fn create(&self, name: String) -> Channel {
        self.channel_repository.create(name).await
    }
}
