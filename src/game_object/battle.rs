use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader},
        query::{Changed, Or, With, Without},
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Commands, Query},
    },
    transform::components::Transform,
};

use crate::{states::ActionState, AppState};

use super::{
    components::{spawn_damage_popup, CastingBar, HealthBar},
    monster::Monster,
    player::Player,
    projectile::Projectile,
    Target,
};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Attacked>()
            .add_event::<Damage>()
            .add_event::<Heal>()
            .add_systems(
                Update,
                (damage, heal, die, attacked, detect_enemy).run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component, Debug)]
pub struct Stat {
    pub speed: f32,
    pub hp: Health,
    pub level: u32,
    pub detect_range: f32,
    pub attack_range: f32,
}

impl Stat {
    pub fn new(speed: f32, hp: f32, level: u32, detect_range: f32, attack_range: f32) -> Self {
        Self {
            speed,
            hp: Health {
                current: hp,
                max: hp,
            },
            level,
            detect_range,
            attack_range,
        }
    }
}

#[derive(Debug)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn ratio(&self) -> f32 {
        self.current / self.max
    }
}

#[derive(Component)]
pub struct Exp {
    current: f32,
    max: f32,
}

impl Exp {
    pub fn from_level(&mut self, level: u32) {
        self.max = level as f32 * level as f32 * 15.5 - 10.5 * level as f32 + 4.5;
    }
}

#[derive(Event)]
pub struct Attacked {
    attacker: Entity,
    attacked: Entity,
}

impl Attacked {
    pub fn new(attacker: Entity, attacked: Entity) -> Self {
        Self { attacker, attacked }
    }

    pub fn attacker(&self) -> Entity {
        self.attacker
    }

    pub fn attacked(&self) -> Entity {
        self.attacked
    }
}

pub fn damage(
    mut damage_evt: EventReader<Damage>,
    mut players: Query<(Entity, &mut Stat), (With<Player>, Without<Monster>)>,
    mut monsters: Query<(Entity, &mut Stat), (With<Monster>, Without<Player>)>,
) {
    for d in damage_evt.read() {
        for (ent, mut stat) in &mut players {
            if d.attacked == ent {
                stat.hp.current -= d.damage;
            }
        }
        for (ent, mut stat) in &mut monsters {
            if d.attacked == ent {
                stat.hp.current -= d.damage;
            }
        }
    }
}

pub fn heal(
    mut commands: Commands,
    mut heal_evt: EventReader<Heal>,
    mut players: Query<(Entity, &mut Stat, &Transform), (With<Player>, Without<Monster>)>,
    mut monsters: Query<(Entity, &mut Stat, &Transform), (With<Monster>, Without<Player>)>,
) {
    for h in heal_evt.read() {
        for (ent, mut stat, t) in &mut players {
            if h.healed == ent {
                stat.hp.current -= h.value;
            }
        }
        for (ent, mut stat, t) in &mut monsters {
            if h.healed == ent {
                stat.hp.current -= h.value;
            }
        }
    }
}

#[derive(Event)]
pub struct Damage {
    pub attacker: Entity,
    pub damage: f32,
    pub damage_type: DamageType,
    pub attacked: Entity,
}

#[derive(Event)]
pub struct Heal {
    pub healer: Entity,
    pub value: f32,
    pub heal_type: HealType,
    pub healed: Entity,
}

#[derive(Debug, Clone)]
pub enum DamageType {
    Melee,
    Magic,
}

#[derive(Debug, Clone)]
pub enum HealType {
    direct,
    dot,
}

pub fn die(
    mut command: Commands,
    mut entities: Query<
        (Entity, &Stat),
        (
            Or<(With<Monster>, With<Player>)>,
            Changed<Stat>,
            Without<HealthBar>,
            Without<Projectile>,
        ),
    >,
    mut bars: Query<
        (Entity, Option<&HealthBar>, Option<&CastingBar>),
        Or<(With<HealthBar>, With<CastingBar>)>,
    >,
) {
    for (ent, stat) in &mut entities {
        if stat.hp.current <= 0. {
            for (bar_ent, h_bar, c_bar) in &mut bars {
                if h_bar.is_some() {
                    if h_bar.unwrap().target == Some(ent) {
                        command.entity(bar_ent).despawn();
                    }
                } else if c_bar.is_some() {
                    if c_bar.unwrap().target == Some(ent) {
                        command.entity(bar_ent).despawn();
                    }
                } else {
                    continue;
                }
            }
            command.entity(ent).despawn();
        }
    }
}

pub fn attacked(
    mut attacked_evt: EventReader<Attacked>,
    mut entities: Query<(Entity, &mut Target, &mut ActionState), Or<(With<Monster>, With<Player>)>>,
) {
    for (ent, mut targ, mut a_state) in &mut entities {
        if targ.0.is_some() {
            continue;
        }
        //if target is None,
        for attacked in attacked_evt.read() {
            if attacked.attacked() == ent {
                *targ = Target(Some(attacked.attacker()));
                *a_state = ActionState::BATTLE;
            }
        }
    }
}

pub fn detect_enemy(
    mut commands: Commands,
    mut players: Query<(Entity, &Transform, &Stat, &mut Target), (With<Player>, Without<Monster>)>,
    mut monsters: Query<(Entity, &Transform, &Stat, &mut Target), (With<Monster>, Without<Player>)>,
) {
    //players
    for (ent, t, stat, mut target) in &mut players {
        for (m_ent, m_t, m_stat, m_target) in &mut monsters {
            if target.0 == None {
                let mut _target: Option<Entity> = None;
                let mut min = f32::MAX;

                // closest
                let distance = t.translation.distance(m_t.translation);
                if distance <= stat.detect_range as f32 {
                    if distance < min {
                        _target = Some(m_ent);

                        min = distance;
                    }
                }

                *target = Target(_target);
            } else {
                if commands.get_entity(target.0.unwrap()).is_none() {
                    *target = Target(None);
                }
            }
        }
    }

    for (ent, t, stat, mut target) in &mut monsters {
        for (p_ent, p_t, p_stat, p_target) in &mut players {
            if target.0 == None {
                let mut _target: Option<Entity> = None;
                let mut min = f32::MAX;

                // closest
                let distance = t.translation.distance(p_t.translation);
                if distance <= stat.detect_range as f32 {
                    if distance < min {
                        _target = Some(p_ent);

                        min = distance;
                    }
                }

                *target = Target(_target);
            } else {
                if commands.get_entity(target.0.unwrap()).is_none() {
                    *target = Target(None);
                }
            }
        }
    }
}
