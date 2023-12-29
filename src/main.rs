//! A simplified implementation of the classic game "Breakout".

mod event;
mod monster;
use bevy::prelude::*;
use event::TargetDropped;
use monster::{move_monster, spawn_monster};
use player::{move_player, spawn_player, target_monster};
mod component;
use component::{move_camera, Stat, Target};
mod player;

const BACKGROUND_COLOR: Color = Color::BEIGE;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        //Player Update system
        .add_systems(
            Update,
            (target_monster, move_camera, move_monster, move_player),
        )
        .add_event::<TargetDropped>()
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .run();
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    // Spawn Player
    let _player_id = spawn_player(&mut commands, String::from("Jason"), Stat::new(1., 50., 1));

    // Spawn Monster
    let _monster_id = spawn_monster(
        &mut commands,
        String::from("Devil Cruise"),
        Stat::new(10., 50., 1),
    );

    // Camera
    commands.spawn((Camera2dBundle::default(), Target(Some(_player_id))));
}
