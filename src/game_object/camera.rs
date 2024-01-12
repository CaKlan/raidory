use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{Or, With, Without},
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Query, Res, ResMut},
    },
    input::{
        keyboard::{KeyCode, KeyboardInput},
        mouse::{MouseButtonInput, MouseWheel},
        ButtonState, Input,
    },
    log::info,
    math::{Vec2, Vec3},
    render::camera::{Camera, OrthographicProjection},
    sprite::{collide_aabb::collide, Sprite},
    time::Time,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};
// use web_sys::KeyboardEvent;

use crate::{resources::resource::SelectedList, AppState};

use super::{monster::Monster, player::Player, Target};

pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_camera, zoom_camera, select_gameobject).run_if(in_state(AppState::InGame)),
        );
    }
}

pub fn move_camera(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut cam: Query<(&mut Transform, &Target), (With<Camera>, Without<Player>, Without<Monster>)>,
    // player: Query<&Transform, With<Player>>,
) {
    let mut dir = Vec3::default();

    let mut pressed = false;
    if keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::W) {
        dir.y += 1.;
        pressed = true;
    }
    if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::D) {
        dir.x += 1.;
        pressed = true;
    }
    if keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::S) {
        dir.y -= 1.;
        pressed = true;
    }
    if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::A) {
        dir.x -= 1.;
        pressed = true;
    }
    if pressed {
        let camera_speed = 500.;
        let c = cam.single_mut();
        let mut transform = c.0;
        transform.translation += dir.normalize_or_zero() * camera_speed * time.delta_seconds();
        info!("{:?}", transform.translation);
    }
}

pub fn zoom_camera(
    mut cam: Query<&mut OrthographicProjection, With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    let zoom = 1.;
    let min = 1.;
    let max = 4.0;
    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {}
            MouseScrollUnit::Pixel => {
                for mut projection in cam.iter_mut() {
                    projection.scale = (projection.scale - zoom * ev.y / 1000.).min(max).max(min);
                }
            }
        }
    }
}

pub fn select_gameobject(
    mut selected_list: ResMut<SelectedList>,
    mut mouse_event: EventReader<MouseButtonInput>,
    mut keys: Res<Input<KeyCode>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut entities: Query<
        (Entity, &GlobalTransform, &Sprite),
        (Or<(With<Monster>, With<Player>)>, Without<Camera>),
    >,
) {
    for ev in mouse_event.read() {
        match ev.state {
            ButtonState::Released => {
                for (cam, cam_t) in &mut cam_q {
                    let Some(mouse_pos) = window_q
                        .single()
                        .cursor_position()
                        .and_then(|cursor| cam.viewport_to_world_2d(cam_t, cursor))
                    else {
                        return;
                    };
                    // let mut entity_select = false;
                    for (ent, ent_t, ent_sprite) in &mut entities {
                        if collide(
                            Vec3::new(mouse_pos.x, mouse_pos.y, 0.),
                            Vec2::new(1., 1.),
                            ent_t.translation(),
                            if let Some(ent_size) = ent_sprite.custom_size {
                                ent_size
                            } else {
                                continue;
                            },
                        )
                        .is_some()
                        {
                            if keys.pressed(KeyCode::ShiftLeft) {
                                selected_list.entities.push(ent);
                                info!("select : {:?}", ent);
                            } else {
                                selected_list.entities = vec![ent];
                                info!("select : {:?}", ent);
                            }
                            // entity_select = true;
                        }
                    }
                    // if !entity_select {
                    //     selected_list.entities.clear();
                    // }
                    // info!("{:?}", mouse_pos);
                }
            }
            _ => {}
        }
    }
}
