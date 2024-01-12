use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, EntityCommands, Query, Res},
    },
    hierarchy::{AddChild, BuildChildren, ChildBuilder, ReplaceChildren},
    prelude::default,
    ui::{node_bundles::NodeBundle, *},
};
use bevy_render::color::Color;

use crate::resources::resource::SelectedList;

use super::CurrentPage;

static SELECTED_UI_WIDTH: f32 = 748.;
static SELECTED_UI_HEIGHT: f32 = 180.;

pub fn setup_ui(mut commands: Commands) {
    commands
        .spawn(background())
        .with_children(|parent| {
            parent.spawn(object_info());
        })
        .with_children(|parent| {
            parent.spawn(selected_ui()).with_children(|parent| {
                parent.spawn(selected_ui_center()).with_children(|parent| {
                    parent.spawn(selected_ui_list()).with_children(|builder| {
                        select_ui_item(builder, Color::BLUE);
                        select_ui_item(builder, Color::BLUE);
                        select_ui_item(builder, Color::BLUE);
                        select_ui_item(builder, Color::BLUE);
                        select_ui_item(builder, Color::BLUE);
                        select_ui_item(builder, Color::BLUE);
                        select_ui_item(builder, Color::BLUE);
                        select_ui_item(builder, Color::BLUE);
                        select_ui_item(builder, Color::BLUE);
                        select_ui_item(builder, Color::BLUE);
                    });
                });
            });
        }); // ui
}

pub fn background() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            width: Val::Vw(100.),
            height: Val::Vh(100.),
            ..Default::default()
        },
        ..default()
    }
}

pub fn object_info() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Px(250.),
            height: Val::Px(100.),

            ..default()
        },
        background_color: Color::ALICE_BLUE.into(),
        ..default()
    }
}

pub fn selected_ui() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::top(Val::Auto),
            width: Val::Percent(100.),
            height: Val::Px(SELECTED_UI_HEIGHT),
            ..default()
        },

        ..default()
    }
}

pub fn selected_ui_center() -> NodeBundle {
    NodeBundle {
        // List
        style: Style {
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Px(SELECTED_UI_WIDTH),
            height: Val::Percent(100.),
            padding: UiRect::all(Val::Px(10.)),
            ..Default::default()
        },
        background_color: Color::rgba(0.05, 0.05, 0.05, 0.85).into(),
        ..Default::default()
    }
}

// pub fn selected_ui_info() -> (NodeBundle, SelectedUiInfo){}

#[derive(Component)]
pub struct SelectedUiList;
pub fn selected_ui_list() -> (NodeBundle, SelectedUiList) {
    (
        NodeBundle {
            // List
            style: Style {
                display: Display::Grid,
                aspect_ratio: Some(1.0),
                padding: UiRect::all(Val::Px(8.0)),
                grid_template_columns: RepeatedGridTrack::flex(10, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                row_gap: Val::Px(4.0),
                column_gap: Val::Px(4.0),
                width: Val::Px(SELECTED_UI_WIDTH - 20.),
                height: Val::Px(SELECTED_UI_HEIGHT - 20.),
                ..Default::default()
            },
            background_color: Color::RED.into(),
            ..Default::default()
        },
        SelectedUiList,
    )
}

pub fn select_ui_item(builder: &mut ChildBuilder, color: Color) {
    builder.spawn(NodeBundle {
        style: Style {
            display: Display::Grid,
            padding: UiRect::all(Val::Px(3.0)),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..Default::default()
    });
}

pub fn selected_ui_list_system(
    mut commands: Commands,
    page: Res<CurrentPage>,
    selected: Res<SelectedList>,
    mut selected_ui_q: Query<Entity, With<SelectedUiList>>,
) {
    let mut ui = selected_ui_q.single_mut();
    // let childs = selected.entities.as_slice();
    let mut childs = Vec::new();
    for ent in &selected.entities {
        let id = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(64.),
                    height: Val::Px(64.),
                    ..Default::default()
                },
                background_color: Color::GOLD.into(),
                ..default()
            })
            .id();
        childs.push(id);
    }
    commands.entity(ui).replace_children(childs.as_slice());
}
