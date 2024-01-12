use bevy::ecs::component::Component;

#[derive(Component, PartialEq, Debug)]
pub enum BattleState {
    IDLE,
    CASTING,
    MOVE,
    RUNAWAY,
}

#[derive(Component, PartialEq, Debug)]
pub enum ActionState {
    IDLE,
    MOVE,
    BATTLE,
}
