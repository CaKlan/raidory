use std::default;

use bevy::{input::keyboard::KeyboardInput, prelude::*, winit::WinitPlugin};

use bevy_pkv::PkvStore;
use bevy_web_asset::WebAssetPlugin;
use game_object::{
    battle::{BattlePlugin, Stat},
    camera::{move_camera, select_gameobject, zoom_camera, CamPlugin},
    command::CommandPlugin,
    components::{damage_popup_system, spawn_damage_popup},
    monster::{spawn_monster, trig_monster_action, MonsterPlugin},
    player::{spawn_player, trig_player_action, Class, PlayerPlugin},
    projectile::{check_collisions, clear_projectile, move_projectile, ProjectilePlugin},
    skill::{base_attack, use_skill, SkillPlugin},
    system::{draw_healthbar, random_spawn_monster, spawn_timer, update_castingbar},
    tree::{animate_sprite, spawn_tree},
    GameObjectPlugin, Target,
};
mod save;
mod ui;
use resources::resource::SelectedList;
use ui::{
    ingame::selected_ui_list_system, main_menu::create_world_button, ui_navigation, CurrentPage,
};
use web_sys::{js_sys, wasm_bindgen};
mod event;
mod game_object;

mod resources;
mod states;

const BACKGROUND_COLOR: Color = Color::BEIGE;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,

    InGame,
    Paused,
}

fn main() {
    App::new()
        .add_plugins((
            WebAssetPlugin::default(),
            DefaultPlugins,
            SkillPlugin,
            CamPlugin,
            BattlePlugin,
            PlayerPlugin,
            ProjectilePlugin,
            MonsterPlugin,
            GameObjectPlugin,
            CommandPlugin,
        ))
        .add_state::<AppState>()
        .insert_resource(CurrentPage::MENU)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(PkvStore::new("nasdac", "raidory"))
        .add_event::<KeyboardInput>()
        .insert_resource(SelectedList::new())
        .add_systems(
            Startup,
            (setup_main_menu, ui::main_menu::setup_ui).run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(
            OnEnter(AppState::InGame),
            (setup_ingame, ui::ingame::setup_ui),
        )
        .add_systems(
            Update,
            (create_world_button, ui_navigation).run_if(in_state(AppState::MainMenu)),
        )
        //Player Update system
        .add_systems(
            Update,
            (
                spawn_damage_popup,
                damage_popup_system,
                selected_ui_list_system,
                draw_healthbar,
                update_castingbar,
                random_spawn_monster,
                animate_sprite,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}

// Add the game's entities to our world
fn setup_main_menu(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn((Camera2dBundle::default(), Target(None)));
}

fn setup_ingame(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // // Camera
    // commands.spawn((Camera2dBundle::default(), Target(None)));

    // Spawn Player
    let _player_id = spawn_player(
        &mut commands,
        String::from("Jason"),
        Stat::new(200., 500., 1, 3000., 200.),
        Class::MAGE,
        Transform::from_translation(Vec3::new(-50., 100., 0.)),
    );

    // Spawn Player
    let _player_id = spawn_player(
        &mut commands,
        String::from("James"),
        Stat::new(200., 500., 1, 3000., 200.),
        Class::MAGE,
        Transform::from_translation(Vec3::new(50., 70., 0.)),
    );

    // Spawn Player
    let _player_id = spawn_player(
        &mut commands,
        String::from("Kate"),
        Stat::new(200., 500., 1, 3000., 200.),
        Class::MAGE,
        Transform::from_translation(Vec3::new(-50., 20., 0.)),
    );

    // Spawn Player
    let _player_id = spawn_player(
        &mut commands,
        String::from("Kim"),
        Stat::new(200., 500., 1, 3000., 200.),
        Class::MAGE,
        Transform::from_translation(Vec3::new(-20., 200., 0.)),
    );

    // Spawn Player
    let _player_id = spawn_player(
        &mut commands,
        String::from("Scalar"),
        Stat::new(200., 500., 1, 3000., 200.),
        Class::MAGE,
        Transform::from_translation(Vec3::new(20., 200., 0.)),
    );

    // Spawn Monster
    let _monster_id = spawn_monster(
        &mut commands,
        String::from("Devil Cruise"),
        Stat::new(20., 500., 1, 3000., 200.),
        Transform::from_xyz(0., 0., 0.),
    );

    // Spawn Timer
    spawn_timer(&mut commands, 5.0);

    // spawn_tree(
    //     &mut commands,
    //     asset_server,
    //     texture_atlases,
    //     0,
    //     Transform::default(),
    // );
}
