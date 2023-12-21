use uuid::Uuid;

use crate::domain::{model::Message, repository::MessageRepository};

#[derive(Clone)]
pub struct SendMessageUsecase<MR> {
    message_repository: MR,
}

impl<MR> SendMessageUsecase<MR>
where
    MR: MessageRepository,
{
    pub fn new(message_repository: MR) -> Self {
        SendMessageUsecase { message_repository }
    }

    pub async fn send(&self, text: String, channel_id: Uuid) -> Message {
        self.message_repository.create(text, channel_id).await
    }
}

#[derive(Clone)]
pub struct ListMessagesUsecase<MR> {
    messages_repository: MR,
}

impl<MR> ListMessagesUsecase<MR>
where
    MR: MessageRepository,
{
    pub fn new(messages_repository: MR) -> Self {
        ListMessagesUsecase {
            messages_repository,
        }
    }

    pub async fn list(&self, channel_id: Uuid) -> Vec<Message> {
        self.messages_repository.list(channel_id).await
    }
}
