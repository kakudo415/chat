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
