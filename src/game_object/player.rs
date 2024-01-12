use bevy::{
    app::{App, FixedUpdate, Plugin},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::{With, Without},
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Commands, Query, Res},
    },
    math::{Vec2, Vec3},
    prelude::default,
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    time::Time,
    transform::components::Transform,
};

use crate::{
    game_object::system::{spawn_castingbar, spawn_healthbar},
    states::{ActionState, BattleState},
    AppState,
};

use super::{
    battle::Stat,
    components::{GameObjectBundle, Team, TeamType},
    monster::Monster,
    skill::SkillInfo,
    MoveTarget, Target,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (trig_player_action).run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component)]
pub enum Class {
    NONE,
    KNIGHT,
    MAGE,
    PRIEST,
    ROGUE,
    HUNTER,
}

#[derive(Component)]
pub struct Player {
    name: String,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    game_object: GameObjectBundle,
    class: Class,
}

pub fn spawn_player(
    commands: &mut Commands,
    name: String,
    stat: Stat,
    class: Class,
    transform: Transform,
) -> Entity {
    let id = commands
        .spawn(PlayerBundle {
            player: Player { name },
            game_object: GameObjectBundle {
                stat,
                target: Target(None),
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
                        custom_size: Some(Vec2::new(50.0, 100.0)),
                        ..default()
                    },
                    transform,
                    ..default()
                },
                move_target: MoveTarget(None),
                action_state: ActionState::IDLE,
                battle_state: BattleState::IDLE,
                skill_info: SkillInfo::new(),
                team: Team(TeamType::PLAYER),
            },
            class,
        })
        .id();
    let t = Transform::from_xyz(10000., 10000., 10000.);
    spawn_healthbar(commands, id, t);
    spawn_castingbar(commands, id, t);
    id
}

pub fn trig_player_action(
    mut command: Commands,
    time: Res<Time>,
    mut player: Query<
        (
            &mut Transform,
            &Stat,
            &Target,
            &mut ActionState,
            &mut BattleState,
            &mut SkillInfo,
        ),
        (With<Player>, Without<Monster>),
    >,
    monsters: Query<&mut Transform, With<Monster>>,
) {
    for (p_transform, p_stat, p_target, mut a_state, mut b_state, mut skill_info) in &mut player {
        //info!(
        //    "astate : {:?}, bastate: {:?}, p_target : {:?}",
        //    action_state, battle_state, p_target.0
        //);
        if *a_state != ActionState::MOVE {
            match p_target.0 {
                // if target exists.
                Some(ent) => {
                    // if get monster entity successfully.
                    match monsters.get(ent).ok() {
                        Some(m_t) => {
                            let dist = p_transform.translation.distance(m_t.translation);
                            // info!("dist : {:?}", dist);
                            if dist <= p_stat.detect_range as f32 {
                                *a_state = ActionState::BATTLE;
                            } else {
                                if *b_state != BattleState::CASTING && dist > p_stat.attack_range {
                                    *b_state = BattleState::MOVE;
                                }
                            }
                        }
                        None => {
                            skill_info.break_casting();
                            continue;
                        }
                    }
                }
                // if not target exists.
                None => {
                    *a_state = ActionState::IDLE;
                    *b_state = BattleState::IDLE
                }
            }
        }
    }
}
