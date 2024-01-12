use bevy::{
    app::{App, FixedUpdate, Plugin, Update},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::EventWriter,
        query::{Or, With, Without},
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Commands, Query, Res},
    },
    log::info,
    math::{Vec2, Vec3},
    prelude::default,
    render::{camera::Camera, color::Color},
    sprite::{collide_aabb::collide, Sprite, SpriteBundle},
    time::Time,
    transform::components::Transform,
};

use crate::AppState;

use super::{
    battle::{Damage, DamageType, Heal, HealType, Stat},
    monster::Monster,
    player::Player,
    Target,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            check_collisions.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (move_projectile, clear_projectile).run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Debug, PartialEq)]
pub enum ProjectileType {
    Targeting,
    NonTargeting,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    projectile: Projectile,
    sprite: SpriteBundle,
    target: Target,
}

#[derive(Component, Debug)]
pub struct Projectile {
    projectile_type: ProjectileType,
    owner: Entity,
    speed: f32,
    damage: f32,
    damage_type: Option<DamageType>,
    heal_type: Option<HealType>,
    lifetime: f32,
}

impl Projectile {
    pub fn new(
        speed: f32,
        damage: f32,
        owner: Entity,
        damage_type: Option<DamageType>,
        heal_type: Option<HealType>,
        lifetime: f32,
        projectile_type: ProjectileType,
    ) -> Self {
        Self {
            projectile_type,
            speed,
            owner,
            damage,
            damage_type,
            heal_type,
            lifetime,
        }
    }
}

pub fn spawn_projectile(
    command: &mut Commands,
    projectile: Projectile,
    transform: Transform,
    target: Target,
) {
    command.spawn(ProjectileBundle {
        projectile,
        target,
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            transform,
            ..default()
        },
    });
}

//systems
pub fn move_projectile(
    mut command: Commands,
    time: Res<Time>,
    mut projectiles: Query<
        (Entity, &mut Transform, &Projectile, &Target),
        (With<Projectile>, Without<Monster>, Without<Player>),
    >,
    entities: Query<
        &Transform,
        (
            Or<(With<Monster>, With<Player>)>,
            Without<Camera>,
            Without<Projectile>,
        ),
    >,
) {
    for (ent, mut t, proj, target) in &mut projectiles {
        // info!("proj : {:?}", proj);
        match target.0 {
            Some(ent) => {
                if let Some(ent_t) = entities.get(ent).ok() {
                    let ent_t = ent_t.translation;
                    let mut t_temp = t.translation;
                    let dir = Vec3::new(ent_t.x - t_temp.x, ent_t.y - t_temp.y, 0.)
                        .normalize_or_zero()
                        * proj.speed
                        * time.delta_seconds();
                    t.translation += dir;
                }
            }
            None => {
                command.entity(ent).despawn();
                continue;
            }
        }
    }
}

pub fn check_collisions(
    mut command: Commands,
    projectiles: Query<
        (Entity, &Transform, &Sprite, &Projectile, &Target),
        (With<Projectile>, Without<Player>, Without<Monster>),
    >,
    mut monsters: Query<
        (Entity, &Transform, &Sprite, &mut Stat),
        (With<Monster>, Without<Player>, Without<Projectile>),
    >,
    mut players: Query<
        (Entity, &Transform, &Sprite, &mut Stat),
        (With<Player>, Without<Monster>, Without<Projectile>),
    >,
    mut damage_evt: EventWriter<Damage>,
    mut heal_evt: EventWriter<Heal>,
) {
    for (p_ent, p_t, p_sprite, projectile, target) in &projectiles {
        let Some(targ) = target.0 else {
            continue;
        };
        if monsters.contains(targ) {
            for (ent, ent_t, ent_sprite, mut ent_stat) in &mut monsters {
                if projectile.projectile_type == ProjectileType::Targeting && target.0 != Some(ent)
                {
                    continue;
                }
                let Some(p_size) = p_sprite.custom_size else {
                    continue;
                };
                let Some(ent_size) = ent_sprite.custom_size else {
                    continue;
                };

                if collide(p_t.translation, p_size, ent_t.translation, ent_size).is_some() {
                    if projectile.damage_type.is_some() {
                        damage_evt.send(Damage {
                            attacker: projectile.owner,
                            damage: projectile.damage,
                            damage_type: projectile.damage_type.clone().unwrap(),
                            attacked: ent,
                        })
                    }
                    if projectile.heal_type.is_some() {
                        heal_evt.send(Heal {
                            healer: projectile.owner,
                            value: projectile.damage,
                            heal_type: projectile.heal_type.clone().unwrap(),
                            healed: ent,
                        })
                    }
                    // info!(
                    //     "current : {:?}, damage: {:?}",
                    //     ent_stat.hp.current, projectile.damage
                    // );
                    command.entity(p_ent).despawn();
                }
            }
        }

        if players.contains(targ) {
            for (ent, ent_t, ent_sprite, mut ent_stat) in &mut players {
                if projectile.projectile_type == ProjectileType::Targeting && target.0 != Some(ent)
                {
                    continue;
                }
                let Some(p_size) = p_sprite.custom_size else {
                    continue;
                };
                let Some(ent_size) = ent_sprite.custom_size else {
                    continue;
                };

                if collide(p_t.translation, p_size, ent_t.translation, ent_size).is_some() {
                    if projectile.damage_type.is_some() {
                        damage_evt.send(Damage {
                            attacker: projectile.owner,
                            damage: projectile.damage,
                            damage_type: projectile.damage_type.clone().unwrap(),
                            attacked: ent,
                        })
                    }
                    if projectile.heal_type.is_some() {
                        heal_evt.send(Heal {
                            healer: projectile.owner,
                            value: projectile.damage,
                            heal_type: projectile.heal_type.clone().unwrap(),
                            healed: ent,
                        })
                    }
                    command.entity(p_ent).despawn();
                }
            }
        }
    }
}

pub fn clear_projectile(
    mut commands: Commands,
    projectiles: Query<(Entity, &Target), With<Projectile>>,
) {
    for (ent, target) in &projectiles {
        let Some(tar) = target.0 else {
            commands.entity(ent).despawn();
            continue;
        };
        if commands.get_entity(tar).is_none() {
            commands.entity(ent).despawn();
        }
    }
}
