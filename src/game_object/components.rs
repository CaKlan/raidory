use bevy::{
    asset::AssetServer,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::EventReader,
        query::{Or, With, Without},
        system::{Commands, Query, Res},
    },
    log::info,
    sprite::{Sprite, SpriteBundle},
    text::{Text, Text2dBounds, Text2dBundle, TextStyle},
    time::{Stopwatch, Time, Timer},
    transform::components::{GlobalTransform, Transform},
    ui::{node_bundles::TextBundle, PositionType, Style, Val},
};
use bevy_render::{camera::Camera, color::Color};

use crate::states::{ActionState, BattleState};

use super::{
    battle::{Damage, Stat},
    monster::Monster,
    player::Player,
    skill::SkillInfo,
    MoveTarget, Target,
};

#[derive(Bundle)]
pub struct GameObjectBundle {
    pub stat: Stat,
    pub target: Target,
    pub move_target: MoveTarget,
    pub sprite: SpriteBundle,
    pub action_state: ActionState,
    pub battle_state: BattleState,
    pub skill_info: SkillInfo,
    pub team: Team,
}

#[derive(PartialEq)]
pub enum TeamType {
    PLAYER,
    MONSTER,
}

#[derive(PartialEq)]
pub enum HealthBarType {
    FRONT,
    ANIMATION,
    BACKGROUND,
}

#[derive(Component)]
pub struct HealthBar {
    pub bar_type: HealthBarType,
    pub target: Option<Entity>,
    pub color: Color,
    pub width: f32,
    pub height: f32,
    pub current: f32,
    pub max: f32,
}

#[derive(Bundle)]
pub struct HealthBarBundle {
    pub sprite: SpriteBundle,
    pub health_bar: HealthBar,
}

#[derive(PartialEq)]
pub enum CastingBarType {
    FRONT,
    BACKGROUND,
}

#[derive(Component)]
pub struct CastingBar {
    pub bar_type: CastingBarType,
    pub target: Option<Entity>,
    pub color: Color,
    pub width: f32,
    pub height: f32,
    pub current: f32,
    pub max: f32,
    pub visibility: bool,
}

#[derive(Bundle)]
pub struct CastingBarBundle {
    pub sprite: SpriteBundle,
    pub casting_bar: CastingBar,
}

#[derive(Component)]
pub struct DamagePopup {
    value: f32,
    is_crit: bool,
    up: f32,
    offset: f32,
    timer: Stopwatch,
    target: Option<Entity>,
}

#[derive(Bundle)]
pub struct DamagePopupBundle {
    popup: DamagePopup,
    text: Text2dBundle,
}

pub fn spawn_damage_popup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    entities: Query<(Entity, &Transform, &Sprite), Or<(With<Player>, With<Monster>)>>,
    mut events: EventReader<Damage>,
) {
    for e in events.read() {
        for (ent, t, sprite) in &entities {
            if ent == e.attacked {
                info!("spawned popup!");
                let pos = t.clone().translation;
                commands.spawn(DamagePopupBundle {
                    popup: DamagePopup {
                        value: e.damage,
                        is_crit: false,
                        up: 0.,
                        offset: sprite.custom_size.unwrap().y / 2.,
                        timer: Stopwatch::new(),
                        target: Some(ent),
                    },
                    text: Text2dBundle {
                        text: Text::from_section(
                            e.damage.to_string(),
                            TextStyle {
                                font: asset_server.load("Consolas.ttf"),
                                font_size: 32.,
                                color: Color::YELLOW,
                            },
                        ),
                        // text_mesh: TextMesh::new_with_color(
                        //     e.damage.to_string(),
                        //     asset_server.load("Consolas.ttf"),
                        //     Color::YELLOW,
                        // ),
                        transform: Transform::from_xyz(pos.x, pos.y, 3.),
                        ..Default::default()
                    },
                });
            }
        }
    }
}

pub fn damage_popup_system(
    mut commands: Commands,
    mut popups: Query<
        (Entity, &mut Transform, &mut DamagePopup, &mut Text),
        (With<DamagePopup>, Without<Player>, Without<Monster>),
    >,
    entities: Query<(Entity, &Transform), Or<(With<Player>, With<Monster>)>>,
    time: Res<Time>,
) {
    let speed = 20.;
    let lifetime = 3.;

    for (ent, mut t, mut d_popup, mut text) in &mut popups {
        info!("popup : {:?}", t.translation);
        d_popup.up += speed * time.delta_seconds();
        let alpha = 1. - d_popup.timer.elapsed_secs() / lifetime;
        info!("alpha : {:?}", alpha);
        text.sections[0]
            .style
            .color
            .set_a(1. - d_popup.timer.elapsed_secs() / lifetime);
        let Some(target) = (*d_popup).target else {
            commands.entity(ent).despawn();
            continue;
        };
        let mut target_exist = false;
        for (t_ent, t_t) in &entities {
            if t_ent == target {
                target_exist = true;
                t.translation.x = t_t.translation.x;
                t.translation.y = t_t.translation.y + d_popup.up + d_popup.offset;
            }
        }
        if !target_exist {
            t.translation.y += speed * time.delta_seconds();
        }

        d_popup.timer.tick(time.delta());
        if d_popup.timer.elapsed_secs() > lifetime {
            commands.entity(ent).despawn();
        }
    }
}

#[derive(Component, PartialEq)]
pub struct Team(pub TeamType);

#[derive(Component)]
pub struct Building {}
