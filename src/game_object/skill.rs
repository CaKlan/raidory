use super::{
    battle::{Attacked, DamageType, Stat},
    components::{Team, TeamType},
    monster::Monster,
    player::{Class, Player},
    projectile::{spawn_projectile, Projectile, ProjectileType},
    Target,
};
use crate::{
    states::{ActionState, BattleState},
    AppState,
};
use bevy::{
    app::{App, FixedUpdate, Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventWriter,
        query::{Or, With},
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Commands, Query, Res},
    },
    time::{Stopwatch, Time},
    transform::components::Transform,
};
use std::time::Duration;

pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (use_skill, base_attack).run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum SkillCode {
    BaseAttack,
    FireBall,
}

#[derive(Component)]
pub struct Skill {
    id: SkillCode,
    name: String,
}

#[derive(Component)]
pub struct SkillInfo {
    current_skill: Option<SkillCode>,
    casting_time: Stopwatch,
    global_cooltime: Stopwatch,
    cast_time: f32,
}

impl SkillInfo {
    pub fn new() -> Self {
        Self {
            current_skill: None,
            casting_time: Stopwatch::default(),
            global_cooltime: Stopwatch::default(),
            cast_time: 0.,
        }
    }

    pub fn current_skill(&self) -> &Option<SkillCode> {
        &self.current_skill
    }

    pub fn set_skill(&mut self, skill: SkillCode) {
        self.current_skill = Some(skill);
    }

    pub fn casting(&mut self, duration: f32) {
        self.casting_time.unpause();
        self.casting_time.reset();
        self.cast_time = duration;
    }

    pub fn break_casting(&mut self) {
        self.casting_time.reset();
        self.casting_time.set_elapsed(Duration::from_secs(0));

        self.casting_time.pause();
        self.cast_time = 0.;
    }

    pub fn is_cast_done(&self) -> bool {
        self.casting_time.elapsed_secs() >= self.cast_time
    }

    pub fn tick(&mut self, delta: Duration) {
        self.casting_time.tick(delta);
        self.global_cooltime.tick(delta);
    }

    pub fn is_casting(&self) -> bool {
        !self.casting_time.paused() && self.cast_time > 0.
    }

    pub fn cast(&mut self, duration: f32, delta: Duration) -> bool {
        if !self.is_casting() {
            self.casting(duration);
        } else {
            self.tick(delta);
        }
        if self.is_cast_done() {
            self.break_casting();
        }
        self.is_cast_done()
    }

    pub fn ratio(&self) -> f32 {
        if self.cast_time == 0. {
            return 0.;
        }
        self.casting_time.elapsed_secs() / self.cast_time
    }
    // pub fn
}

pub fn use_skill(
    mut entities: Query<
        (
            &Transform,
            &mut Target,
            &Stat,
            &ActionState,
            &mut BattleState,
            &mut SkillInfo,
        ),
        Or<(With<Player>, With<Monster>)>,
    >,
) {
    for (_t, _targ, _stat, a_state, b_state, mut skill_info) in &mut entities {
        if *a_state == ActionState::BATTLE {
            if *b_state != BattleState::CASTING && *b_state != BattleState::RUNAWAY {
                skill_info.set_skill(SkillCode::BaseAttack);
            }
        }
    }
}

pub fn base_attack(
    mut command: Commands,
    time: Res<Time>,
    mut attacked_evt: EventWriter<Attacked>,
    mut entities: Query<
        (
            Entity,
            &Transform,
            &Target,
            &mut Stat,
            &ActionState,
            &mut BattleState,
            &mut SkillInfo,
            Option<&Class>,
            &Team,
        ),
        Or<(With<Player>, With<Monster>)>,
    >,
) {
    let mut combinations = entities.iter_combinations_mut();
    while let Some([a, b]) = combinations.fetch_next() {
        if a.0 == b.0 {
            continue;
        }

        let (ent, t, target, stat, _a_state, mut b_state, mut skill, class, team) = a;
        let cur_skill = skill.current_skill();
        let code = if cur_skill.is_some() {
            cur_skill.clone().unwrap()
        } else {
            continue;
        };

        let Some(targ_ent) = target.0 else {
            continue;
        };

        //only target
        if targ_ent != b.0 {
            continue;
        }

        let mut targ_stat = b.3;

        if code == SkillCode::BaseAttack {
            if team.0 == TeamType::PLAYER {
                let Some(class) = class else {
                    continue;
                };
                match class {
                    Class::NONE => {
                        if skill.cast(0.2, time.delta()) {
                            targ_stat.hp.current -= (stat.level * 5) as f32;
                            attacked_evt.send(Attacked::new(ent, targ_ent));
                            //info!("base attack!");
                        }
                    }
                    Class::KNIGHT => todo!(),
                    Class::MAGE => {
                        *b_state = if skill.is_casting() {
                            BattleState::CASTING
                        } else {
                            BattleState::IDLE
                        };
                        if skill.cast(1.0, time.delta()) {
                            spawn_projectile(
                                &mut command,
                                Projectile::new(
                                    1000.,
                                    10.,
                                    ent,
                                    Some(DamageType::Magic),
                                    None,
                                    2.,
                                    ProjectileType::Targeting,
                                ),
                                t.clone(),
                                Target(Some(targ_ent)),
                            );
                        }
                    }
                    Class::PRIEST => todo!(),
                    Class::ROGUE => todo!(),
                    Class::HUNTER => todo!(),
                }
            } else if team.0 == TeamType::MONSTER {
                if code == SkillCode::BaseAttack {
                    *b_state = if skill.is_casting() {
                        BattleState::CASTING
                    } else {
                        BattleState::IDLE
                    };
                    if skill.cast(1.0, time.delta()) {
                        spawn_projectile(
                            &mut command,
                            Projectile::new(
                                1000.,
                                10.,
                                ent,
                                Some(DamageType::Magic),
                                None,
                                2.,
                                ProjectileType::Targeting,
                            ),
                            t.clone(),
                            Target(Some(targ_ent)),
                        );
                    }
                }
            } else {
                continue;
            }
        }
    }
}
