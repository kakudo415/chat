use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Channel {
    id: Uuid,
    name: String,
}

impl Channel {
    pub fn new(name: String) -> Self {
        Channel {
            id: Uuid::now_v7(),
            name,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct Message {
    id: Uuid,
    text: String,
    channel_id: Uuid,
    created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(text: String, channel_id: Uuid) -> Self {
        Message {
            id: Uuid::now_v7(),
            text,
            channel_id,
            created_at: Utc::now(),
        }
    }

    pub fn from_raw_parts(
        id: Uuid,
        text: String,
        channel_id: Uuid,
        created_at: DateTime<Utc>,
    ) -> Self {
        Message {
            id,
            text,
            channel_id,
            created_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn channel_id(&self) -> Uuid {
        self.channel_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
