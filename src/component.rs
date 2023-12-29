use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::Query,
    },
    render::camera::Camera,
    transform::components::Transform,
};

use crate::{monster::MonsterInfo, player::PlayerInfo};

#[derive(Component, Debug)]
pub struct Stat {
    pub speed: f64,
    pub hp: f64,
    pub level: u64,
    pub detect_range: f64,
}

impl Stat {
    pub fn new(speed: f64, hp: f64, level: u64) -> Self {
        Self {
            speed,
            hp,
            level,
            detect_range: 300.,
        }
    }
}

#[derive(Component, Debug)]
pub struct Target(pub Option<Entity>);

#[derive(Component)]
pub struct Projectile {
    speed: f64,
    damage: f64,
    target: Entity,
}

impl Projectile {
    pub fn new(speed: f64, damage: f64, target: Entity) -> Self {
        Self {
            speed,
            damage,
            target,
        }
    }
}

pub fn move_camera(
    mut cam: Query<
        (&mut Transform, &Target),
        (With<Camera>, Without<PlayerInfo>, Without<MonsterInfo>),
    >,
    player: Query<&Transform, With<PlayerInfo>>,
) {
    let c = cam.single_mut();
    let mut transform = c.0;
    let target = c.1 .0;
    if let Some(ent) = target {
        if let Some(t) = player.get(ent).ok() {
            transform.translation = t.translation
        }
    }
}
