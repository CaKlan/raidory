use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    log::info,
    math::{Vec2, Vec3},
    prelude::default,
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    time::Time,
    transform::components::Transform,
};

use crate::{
    component::{Stat, Target},
    monster::MonsterInfo,
};

#[derive(Component)]
pub struct PlayerInfo {
    name: String,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player_info: PlayerInfo,
    stat: Stat,
    target: Target,
    sprite: SpriteBundle,
}

pub fn spawn_player(commands: &mut Commands, name: String, stat: Stat) -> Entity {
    commands
        .spawn(PlayerBundle {
            player_info: PlayerInfo { name },
            stat,
            target: Target(None),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(50.0, 100.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-50., 100., 0.)),
                ..default()
            },
        })
        .id()
}

pub fn move_player(
    time: Res<Time>,
    mut player: Query<(&mut Transform, &Stat, &Target), (With<PlayerInfo>, Without<MonsterInfo>)>,
    monsters: Query<&mut Transform, With<MonsterInfo>>,
) {
    // for (t, s) in &player {
    //     info!("{:?}, {:?}", t, s);
    // }
    for (mut transform, stat, target) in &mut player {
        match target.0 {
            Some(ent) => {
                let target_transform = match monsters.get(ent).ok() {
                    Some(t) => t,
                    None => return,
                };
                let p_x = transform.translation.x;
                let p_y = transform.translation.y;
                let t_x = target_transform.translation.x;
                let t_y = target_transform.translation.y;

                let direction = Vec3::new(t_x - p_x, t_y - p_y, 0.).normalize();
                info!("{:?}", direction);
                // update the ship translation with our new translation delta
                transform.translation += stat.speed as f32 * direction;
            }
            None => {
                transform.translation.x += (5. * stat.speed * time.delta_seconds_f64()) as f32;
            }
        };
    }
}

pub fn target_monster(
    monsters: Query<(Entity, &Transform), (With<MonsterInfo>, Without<PlayerInfo>)>,
    mut players: Query<(&Transform, &mut Target, &Stat), With<PlayerInfo>>,
) {
    for (p_transform, mut target, stat) in &mut players {
        let mut _target: Option<Entity> = None;
        let mut min = f32::MAX;
        for (ent, m_transform) in &monsters {
            let dist = p_transform.translation.distance(m_transform.translation);
            info!("{:?}", dist);
            if dist <= stat.detect_range as f32 {
                if dist < min {
                    _target = Some(ent);
                    min = dist;
                }
            }
        }
        *target = Target(_target);
        info!("{:?}", target);
    }
}
