use axum::extract::{Json, Path, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::repository::{ChannelRepository, MessageRepository},
    usecase::{channel::CreateChannelUseCase, message::SendMessageUsecase},
};

#[derive(Clone)]
pub struct ChannelState<CR, MR> {
    create_channel_usecase: CreateChannelUseCase<CR>,
    send_message_usecase: SendMessageUsecase<MR>,
}

impl<CR, MR> ChannelState<CR, MR>
where
    CR: ChannelRepository,
    MR: MessageRepository,
{
    pub fn new(
        create_channel_usecase: CreateChannelUseCase<CR>,
        send_message_usecase: SendMessageUsecase<MR>,
    ) -> Self {
        ChannelState {
            create_channel_usecase,
            send_message_usecase,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateChannelRequest {
    name: String,
}

#[derive(Serialize)]
pub struct CreateChannelResponse {
    id: String,
    name: String,
}

pub async fn create_channel<CR, MR>(
    State(state): State<Arc<ChannelState<CR, MR>>>,
    Json(request): Json<CreateChannelRequest>,
) -> Json<CreateChannelResponse>
where
    CR: ChannelRepository,
    MR: MessageRepository,
{
    let channel = state.create_channel_usecase.create(request.name).await;
    Json(CreateChannelResponse {
        id: channel.id().to_string(),
        name: channel.name(),
    })
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    text: String,
}

#[derive(Serialize)]
pub struct SendMessageResponse {
    id: String,
    text: String,
    channel_id: String,
}

pub async fn send_message<CR, MR>(
    Path(channel_id): Path<Uuid>,
    State(state): State<Arc<ChannelState<CR, MR>>>,
    Json(request): Json<SendMessageRequest>,
) -> Json<SendMessageResponse>
where
    CR: ChannelRepository,
    MR: MessageRepository,
{
    let message = state
        .send_message_usecase
        .send(request.text, channel_id)
        .await;
    Json(SendMessageResponse {
        id: message.id().to_string(),
        text: message.text(),
        channel_id: message.channel_id().to_string(),
    })
}
