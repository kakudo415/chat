use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{domain::repository::ChannelRepository, usecase::channel::CreateChannelUseCase};

#[derive(Clone)]
pub struct ChannelState<CR> {
    create_channel_usecase: CreateChannelUseCase<CR>,
}

impl<CR> ChannelState<CR>
where
    CR: ChannelRepository,
{
    pub fn new(create_channel_usecase: CreateChannelUseCase<CR>) -> Self {
        ChannelState {
            create_channel_usecase,
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

pub async fn create_channels<CR>(
    State(state): State<Arc<ChannelState<CR>>>,
    Json(request): Json<CreateChannelRequest>,
) -> Json<CreateChannelResponse>
where
    CR: ChannelRepository,
{
    let channel = state.create_channel_usecase.create(request.name).await;
    Json(CreateChannelResponse {
        id: channel.id().to_string(),
        name: channel.name(),
    })
}
