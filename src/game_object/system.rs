use std::{cell::RefCell, rc::Rc, time::Duration};

use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::EventReader,
        query::{Changed, Or, With, Without},
        system::{Commands, Query, Res},
    },
    log::info,
    math::{Vec2, Vec3},
    prelude::default,
    render::color::Color,
    sprite::{Anchor, Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
};
use rand::Rng;

use crate::{
    resources::resource::SelectedList,
    states::{ActionState, BattleState},
};

use super::{
    battle::Stat,
    components::{
        CastingBar, CastingBarBundle, CastingBarType, HealthBar, HealthBarBundle, HealthBarType,
        Team,
    },
    monster::{spawn_monster, Monster},
    player::Player,
    projectile::Projectile,
    skill::SkillInfo,
};

pub fn spawn_healthbar(command: &mut Commands, id: Entity, transform: Transform) {
    command.spawn(HealthBarBundle {
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(70., 10.)),
                anchor: Anchor::CenterLeft,
                ..default()
            },
            transform,
            ..default()
        },
        health_bar: HealthBar {
            bar_type: HealthBarType::FRONT,
            target: Some(id),
            color: Color::RED,
            width: 70.,
            height: 10.,
            current: 1.,
            max: 1.,
        },
    });
    //Animation health_bar
    command.spawn(HealthBarBundle {
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(70., 10.)),
                anchor: Anchor::CenterLeft,
                ..default()
            },
            transform,
            ..default()
        },
        health_bar: HealthBar {
            bar_type: HealthBarType::ANIMATION,
            target: Some(id),
            color: Color::YELLOW,
            width: 70.,
            height: 10.,
            current: 1.,
            max: 1.,
        },
    });
    //BACKGROUND health_bar
    command.spawn(HealthBarBundle {
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(74., 14.)),
                anchor: Anchor::CenterLeft,
                ..default()
            },
            transform,
            ..default()
        },
        health_bar: HealthBar {
            bar_type: HealthBarType::BACKGROUND,
            target: Some(id),
            color: Color::BLACK,
            width: 74.,
            height: 14.,
            current: 1.,
            max: 1.,
        },
    });
}

pub fn draw_healthbar(
    entities: Query<
        (&Transform, &Stat, &Sprite),
        (
            Or<(With<Monster>, With<Player>)>,
            Without<HealthBar>,
            Without<Projectile>,
        ),
    >,
    mut bars: Query<(&mut Transform, &mut HealthBar, &mut Sprite), With<HealthBar>>,
) {
    let top_margin = 30.;

    for (mut bar_t, bar, mut sprite) in &mut bars {
        if let Some(mut size) = sprite.custom_size {
            if let Some(ent) = bar.target {
                let (ent_t, ent_stat, ent_sprite) = if entities.get(ent).is_ok() {
                    entities.get(ent).unwrap()
                } else {
                    continue;
                };

                let margin = if let Some(size) = ent_sprite.custom_size {
                    size.y / 2. + top_margin
                } else {
                    top_margin
                };

                let target_width = ent_stat.hp.ratio() * bar.width;
                if bar.bar_type == HealthBarType::FRONT {
                    //FRONT
                    size.x = target_width;
                } else if bar.bar_type == HealthBarType::ANIMATION {
                    // ANIMATION
                    if size.x >= target_width {
                        size.x = target_width.max(size.x - 0.5);
                    }
                }

                sprite.custom_size = Some(size);
                bar_t.translation = Vec3::new(
                    ent_t.translation.x - bar.width / 2.,
                    ent_t.translation.y + margin,
                    if bar.bar_type == HealthBarType::FRONT {
                        3.
                    } else if bar.bar_type == HealthBarType::ANIMATION {
                        2.
                    } else {
                        1.
                    },
                );
                // info!("size : {:?}", size);
            }
        }
    }
}

pub fn spawn_castingbar(command: &mut Commands, id: Entity, transform: Transform) {
    let width = 74.;
    let height = 10.;
    command.spawn(CastingBarBundle {
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(width, height)),
                anchor: Anchor::CenterLeft,
                ..Default::default()
            },
            transform,
            ..Default::default()
        },
        casting_bar: CastingBar {
            bar_type: CastingBarType::FRONT,
            target: Some(id),
            color: Color::BLUE,
            width,
            height,
            current: 1.,
            max: 1.,
            visibility: false,
        },
    });
}

pub fn update_castingbar(
    mut bars: Query<(&mut Transform, &CastingBar, &mut Sprite), With<CastingBar>>,
    entities: Query<
        (&Transform, &Stat, &Sprite, &BattleState, &SkillInfo),
        (
            Or<(With<Monster>, With<Player>)>,
            Without<CastingBar>,
            Without<Projectile>,
        ),
    >,
) {
    for (mut t, bar, mut sprite) in &mut bars {
        let top_margin = 15.;
        let mut size = if sprite.custom_size.is_some() {
            sprite.custom_size.unwrap()
        } else {
            //info!("continue here.");
            continue;
        };
        let ent = if bar.target.is_some() {
            bar.target.unwrap()
        } else {
            //info!("continue here.");
            continue;
        };

        let (ent_t, ent_stat, ent_sprite, b_state, skill) = if entities.get(ent).is_ok() {
            entities.get(ent).unwrap()
        } else {
            //info!("continue here.");
            continue;
        };
        if *b_state == BattleState::CASTING {
            let margin = if let Some(size) = ent_sprite.custom_size {
                size.y / 2. + top_margin
            } else {
                top_margin
            };

            let target_width = skill.ratio() * bar.width;
            //info!("targ :{:}", target_width);
            if bar.bar_type == CastingBarType::FRONT {
                //FRONT
                size.x = target_width;
            }

            sprite.custom_size = Some(size);
            t.translation = Vec3::new(
                ent_t.translation.x - bar.width / 2.,
                ent_t.translation.y + margin,
                if bar.bar_type == CastingBarType::FRONT {
                    2.
                } else {
                    1.
                },
            );
        } else {
            size.x = 0.;
            sprite.custom_size = Some(size);
        }
    }
}

#[derive(Component)]
pub struct SpawnTimer(Timer);

#[derive(Bundle)]
pub struct SpawnTimerBundle {
    timer: SpawnTimer,
}
pub fn spawn_timer(command: &mut Commands, secs: f32) {
    command.spawn(SpawnTimer(Timer::new(
        Duration::from_secs_f32(secs),
        TimerMode::Repeating,
    )));
}

pub fn random_spawn_monster(
    mut command: Commands,
    time: Res<Time>,
    mut timer: Query<&mut SpawnTimer>,
) {
    let mut timer = timer.single_mut();

    timer.0.tick(time.delta());

    if timer.0.finished() {
        let mut rng = rand::thread_rng();
        let size = 500.;
        let rand_x = rng.gen::<f32>() * size;
        let rand_y = rng.gen::<f32>() * size;
        let rand_z = rng.gen::<f32>() * size;
        spawn_monster(
            &mut command,
            String::from("Devil Cruise"),
            Stat::new(20., 30., 1, 5000., 50.),
            Transform::from_xyz(400. + rand_x, rand_y, rand_z),
        );
    }
}
