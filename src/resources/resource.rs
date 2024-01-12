use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Resource)]
pub struct SelectedList {
    pub entities: Vec<Entity>,
}

impl SelectedList {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
}
