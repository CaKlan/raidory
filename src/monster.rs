use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    math::{Vec2, Vec3},
    prelude::default,
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    time::Time,
    transform::components::Transform,
};

use crate::{player::PlayerInfo, Stat};

#[derive(Component)]
pub struct MonsterInfo {
    name: String,
}

#[derive(Bundle)]
pub struct MonsterBundle {
    transform: Transform,
    monster_info: MonsterInfo,
    stat: Stat,
}

pub fn spawn_monster(commands: &mut Commands, name: String, stat: Stat) -> Entity {
    commands
        .spawn(MonsterBundle {
            transform: Transform::default(),
            monster_info: MonsterInfo { name },
            stat,
        })
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(40.0, 80.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
            ..default()
        })
        .id()
}

pub fn move_monster(
    time: Res<Time>,
    mut monsters: Query<(&mut Transform, &Stat), (With<MonsterInfo>, Without<PlayerInfo>)>,
) {
    // for (t, s) in &player {
    //     info!("{:?}, {:?}", t, s);
    // }
    for (mut transform, stat) in &mut monsters {
        transform.translation.x += (5. * stat.speed) as f32 * time.delta_seconds();
    }
}
