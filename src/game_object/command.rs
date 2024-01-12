use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::{Or, With, Without},
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Query, ResMut},
    },
    input::{
        mouse::{MouseButton, MouseButtonInput},
        ButtonState,
    },
    log::info,
    math::{Vec2, Vec3},
    sprite::{collide_aabb::collide, Sprite},
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
    winit::{self, WinitWindows},
};
use bevy_render::camera::Camera;

use crate::{resources::resource::SelectedList, states::ActionState, AppState};

use super::{monster::Monster, player::Player, MoveTarget};

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, order_move.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct Command {}

pub fn order_move(
    mut selected_list: ResMut<SelectedList>,
    mut mouse_event: EventReader<MouseButtonInput>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<
        (&Camera, &GlobalTransform),
        (With<Camera>, Without<Player>, Without<Monster>),
    >,

    mut entities: Query<
        (
            Entity,
            &Transform,
            &Sprite,
            &mut MoveTarget,
            &mut ActionState,
        ),
        (Or<(With<Player>, With<Monster>)>),
    >,
) {
    let (cam, cam_t) = cam_q.single();

    let Some(mouse_pos) = window_q
        .single()
        .cursor_position()
        .and_then(|cursor| cam.viewport_to_world_2d(cam_t, cursor))
    else {
        return;
    };

    for ev in mouse_event.read() {
        match ev.state {
            ButtonState::Released => match ev.button {
                MouseButton::Left => {
                    for (ent, t, sprite, mut mv_targ, mut a_state) in &mut entities {
                        if collide(
                            Vec3::from((mouse_pos, 0.)),
                            Vec2::new(1., 1.),
                            t.translation,
                            sprite.custom_size.unwrap(),
                        )
                        .is_some()
                        {
                            return;
                        }
                        if selected_list.entities.contains(&ent) {
                            let vec = Vec3::from((mouse_pos, 0.));
                            *mv_targ = MoveTarget(Some(vec));
                            *a_state = ActionState::MOVE;
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
