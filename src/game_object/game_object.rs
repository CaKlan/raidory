use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        query::{Or, With, Without},
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Query, Res},
    },
    math::Vec3,
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
};
use bevy_render::color::Color;

use crate::{
    resources::resource::SelectedList,
    states::{ActionState, BattleState},
    AppState,
};

use super::{battle::Stat, monster::Monster, player::Player, skill::SkillInfo};

pub struct GameObjectPlugin;

impl Plugin for GameObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (selected_gameobject, move_gameobject).run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component, Debug)]
pub struct Target(pub Option<Entity>);

#[derive(Component, Debug)]
pub struct MoveTarget(pub Option<Vec3>);

pub fn selected_gameobject(
    selected_list: Res<SelectedList>,
    mut entities: Query<
        (Entity, &mut Sprite, Option<&Monster>, Option<&Player>),
        Or<(With<Monster>, With<Player>)>,
    >,
) {
    for (ent, mut sprite, monster, player) in &mut entities {
        if (*selected_list).entities.contains(&ent) {
            sprite.color = Color::GREEN;
        } else {
            sprite.color = if monster.is_some() {
                Color::RED
            } else if player.is_some() {
                Color::BLUE
            } else {
                continue;
            }
        }
    }
}

/**
 if all entities's [`ActionState`] is [`ActionState::MOVE`]

 and then they move toward target
*/
pub fn move_gameobject(
    time: Res<Time>,
    mut players: Query<
        (
            Entity,
            &mut Transform,
            &Stat,
            &Target,
            &MoveTarget,
            &ActionState,
            &mut BattleState,
            &mut SkillInfo,
        ),
        (With<Player>, Without<Monster>),
    >,
    mut monsters: Query<
        (
            Entity,
            &mut Transform,
            &Stat,
            &Target,
            &MoveTarget,
            &ActionState,
            &mut BattleState,
            &mut SkillInfo,
        ),
        (With<Monster>, Without<Player>),
    >,
) {
    for (ent, mut t, stat, target, mv_targ, a_state, mut b_state, mut skill) in &mut players {
        let Some(tar) = target.0 else {
            continue;
        };
        let Some((t_ent, mut t_t, t_stat, t_target, t_mv_targ, t_a_state, t_b_state, t_skill)) =
            monsters.get(tar).ok()
        else {
            continue;
        };

        match *a_state {
            ActionState::IDLE => {}
            ActionState::MOVE => {
                skill.break_casting();
                match mv_targ.0 {
                    Some(_ent) => {
                        let p_x = t.translation.x;
                        let p_y = t.translation.y;
                        let t_x = t_t.translation.x;
                        let t_y = t_t.translation.y;

                        let direction = Vec3::new(t_x - p_x, t_y - p_y, 0.).normalize();

                        t.translation += stat.speed * direction * time.delta_seconds();
                    }
                    None => {
                        t.translation.x += stat.speed * time.delta_seconds();
                    }
                };
            }
            ActionState::BATTLE => {
                if *b_state == BattleState::MOVE {
                    skill.break_casting();
                    match target.0 {
                        Some(_ent) => {
                            let p_x = t.translation.x;
                            let p_y = t.translation.y;
                            let t_x = t_t.translation.x;
                            let t_y = t_t.translation.y;

                            let direction = Vec3::new(t_x - p_x, t_y - p_y, 0.).normalize();

                            t.translation += stat.speed * direction * time.delta_seconds();
                        }
                        None => {
                            t.translation.x += stat.speed * time.delta_seconds();
                        }
                    };
                }
            }
        }
    }
    for (ent, mut t, stat, target, mv_targ, a_state, mut b_state, mut skill) in &mut monsters {
        let Some(tar) = target.0 else {
            continue;
        };
        let Some((t_ent, mut t_t, t_stat, t_target, t_mv_targ, t_a_state, t_b_state, t_skill)) =
            players.get(tar).ok()
        else {
            continue;
        };

        match *a_state {
            ActionState::IDLE => {}
            ActionState::MOVE => {
                skill.break_casting();
                match mv_targ.0 {
                    Some(_ent) => {
                        let p_x = t.translation.x;
                        let p_y = t.translation.y;
                        let t_x = t_t.translation.x;
                        let t_y = t_t.translation.y;

                        let direction = Vec3::new(t_x - p_x, t_y - p_y, 0.).normalize();

                        t.translation += stat.speed * direction * time.delta_seconds();
                    }
                    None => {
                        t.translation.x += stat.speed * time.delta_seconds();
                    }
                };
            }
            ActionState::BATTLE => {
                if *b_state == BattleState::MOVE {
                    skill.break_casting();
                    match target.0 {
                        Some(_ent) => {
                            let p_x = t.translation.x;
                            let p_y = t.translation.y;
                            let t_x = t_t.translation.x;
                            let t_y = t_t.translation.y;

                            let direction = Vec3::new(t_x - p_x, t_y - p_y, 0.).normalize();

                            t.translation += stat.speed * direction * time.delta_seconds();
                        }
                        None => {
                            t.translation.x += stat.speed * time.delta_seconds();
                        }
                    };
                }
            }
        }
    }
}
