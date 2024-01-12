use bevy::{ecs::system::ResMut, log::info};
use bevy_pkv::PkvStore;

pub fn create_world(pkv: &mut ResMut<PkvStore>) {
    if let Ok(username) = pkv.get::<String>("username") {
        info!("Welcome back {username}");
    } else {
        pkv.set_string("username", "alice")
            .expect("failed to store username");
    }
}
