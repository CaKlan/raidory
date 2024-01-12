use bevy::{
    app::{App, Plugin, Update},
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
    game_object::{system::spawn_healthbar, Target},
    states::{ActionState, BattleState},
    AppState, Stat,
};

use super::{
    components::{GameObjectBundle, Team, TeamType},
    player::Player,
    skill::SkillInfo,
    MoveTarget,
};

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            trig_monster_action.run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct Monster {
    name: String,
    exp: f32,
    //drop_item,
    //drop_item chance,
}

#[derive(Bundle)]
pub struct MonsterBundle {
    monster: Monster,
    game_object: GameObjectBundle,
}

impl Monster {
    pub fn new(name: String, exp: f32) -> Self {
        Self { name, exp }
    }
}

pub fn spawn_monster(
    commands: &mut Commands,
    name: String,
    stat: Stat,
    transform: Transform,
) -> Entity {
    let id = commands
        .spawn(MonsterBundle {
            monster: Monster::new(name, 10.),
            game_object: GameObjectBundle {
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(50., 80.)),
                        ..default()
                    },
                    transform,
                    ..default()
                },
                move_target: MoveTarget(None),
                stat,
                target: Target(None),
                action_state: ActionState::IDLE,
                battle_state: BattleState::IDLE,
                skill_info: SkillInfo::new(),
                team: Team(TeamType::MONSTER),
            },
        })
        .id();
    let t = Transform::from_xyz(10000., 10000., 10000.);
    spawn_healthbar(commands, id, t);
    id
}

pub fn trig_monster_action(
    mut command: Commands,
    time: Res<Time>,
    mut monsters: Query<
        (
            &mut Transform,
            &Stat,
            &Target,
            &mut ActionState,
            &mut BattleState,
            &mut SkillInfo,
        ),
        (With<Monster>, Without<Player>),
    >,
    players: Query<&mut Transform, With<Player>>,
) {
    for (m_transform, m_stat, m_target, mut action_state, mut battle_state, mut skill_info) in
        &mut monsters
    {
        //info!(
        //    "astate : {:?}, bastate: {:?}, p_target : {:?}",
        //    action_state, battle_state, p_target.0
        //);

        match m_target.0 {
            // if target exists.
            Some(ent) => {
                // if get monster entity successfully.
                match players.get(ent).ok() {
                    Some(p_t) => {
                        let dist = m_transform.translation.distance(p_t.translation);
                        // info!("dist : {:?}", dist);
                        if dist <= m_stat.detect_range as f32 {
                            *action_state = ActionState::BATTLE;
                        } else {
                            if *battle_state != BattleState::CASTING && dist > m_stat.attack_range {
                                *action_state = ActionState::MOVE;
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
            None => *action_state = ActionState::IDLE,
        }
    }
}
