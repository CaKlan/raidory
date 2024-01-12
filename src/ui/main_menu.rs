use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::save::save::create_world;

use super::{CurrentPage, Page};

static MENU_WIDTH: f32 = 300.;
static MENU_HEIGHT: f32 = 400.;

pub fn setup_ui(mut commands: Commands, server: Res<AssetServer>) {
    let font = server.load("Consolas.ttf");
    //MAINMENU
    commands
        .spawn(
            //empty fullscreen ui
            (
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Vw(100.),
                        height: Val::Vh(100.),
                        ..Default::default()
                    },
                    background_color: Color::DARK_GRAY.into(),
                    ..Default::default()
                },
                Page(CurrentPage::MENU),
            ),
        )
        .with_children(|p| {
            p.spawn(
                //Menu
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(MENU_WIDTH),
                        height: Val::Px(MENU_HEIGHT),
                        ..Default::default()
                    },
                    background_color: Color::GRAY.into(),
                    ..Default::default()
                },
            )
            .with_children(|menu| {
                menu.spawn(
                    //menu buttons
                    menu_button(),
                )
                .with_children(|button| {
                    //Create World
                    button.spawn(button_text("Create World", font.clone()));
                });

                menu.spawn(
                    //menu buttons
                    menu_button(),
                )
                .with_children(|button| {
                    //Load World
                    button.spawn(button_text("Load World", font.clone()));
                });

                menu.spawn(
                    //menu buttons
                    menu_button(),
                )
                .with_children(|button| {
                    //Settings
                    button.spawn(button_text("Settings", font.clone()));
                });
            });
        });

    //CREATEWORLD
    commands
        .spawn(
            //empty fullscreen ui
            (
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Vw(100.),
                        height: Val::Vh(100.),
                        ..Default::default()
                    },
                    background_color: Color::DARK_GRAY.into(),
                    ..Default::default()
                },
                Page(CurrentPage::CREATEWORLD),
            ),
        )
        .with_children(|p| {
            //뒤로가기
            // p.spawn(NodeBundle {
            //     style: Style {
            //         display: Display::Flex,
            //         position_type: PositionType::Absolute,
            //         width: Val::Px(50.),
            //         height: Val::Px(50.),
            //         ..Default::default()
            //     },
            //     ..Default::default()
            // })
            // .with_children(|back| {
            //     back.spawn(Text2dBundle {
            //         text: Text::from_section(
            //             "<",
            //             TextStyle {
            //                 font: font.clone(),
            //                 font_size: 16.,
            //                 color: Color::WHITE.into(),
            //             },
            //         ),
            //         ..Default::default()
            //     });
            // });

            // Character 1
            p.spawn(
                //Menu
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::right(Val::Px(50.)),
                        width: Val::Px(MENU_WIDTH),
                        height: Val::Px(MENU_HEIGHT),
                        ..Default::default()
                    },
                    background_color: Color::GRAY.into(),
                    ..Default::default()
                },
            )
            .with_children(|menu| {
                //name
                menu.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.),
                        height: Val::Px(50.),
                        ..Default::default()
                    },
                    background_color: Color::RED.into(),
                    ..Default::default()
                });
                //sprite
                menu.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Px(200.),
                        ..Default::default()
                    },
                    background_color: Color::BLUE.into(),
                    ..Default::default()
                });
                //info
                menu.spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        width: Val::Percent(100.),
                        height: Val::Px(MENU_HEIGHT - 250.),
                        ..Default::default()
                    },
                    background_color: Color::ORANGE.into(),
                    ..Default::default()
                });
            });

            // Character 2
            p.spawn(
                //Menu
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::right(Val::Px(50.)),
                        width: Val::Px(MENU_WIDTH),
                        height: Val::Px(MENU_HEIGHT),
                        ..Default::default()
                    },
                    background_color: Color::GRAY.into(),
                    ..Default::default()
                },
            )
            .with_children(|menu| {});

            // Character 3
            p.spawn(
                //Menu
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(MENU_WIDTH),
                        height: Val::Px(MENU_HEIGHT),
                        ..Default::default()
                    },
                    background_color: Color::GRAY.into(),
                    ..Default::default()
                },
            )
            .with_children(|menu| {});
        });
}

fn menu_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            display: Display::Flex,
            width: Val::Px(MENU_WIDTH - 60.),
            height: Val::Px(50.),
            margin: UiRect::bottom(Val::Px(20.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: Color::DARK_GRAY.into(),
        ..Default::default()
    }
}

fn button_text(text: &str, font: Handle<Font>) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font,
                font_size: 16.,
                color: Color::WHITE,
            },
        ),
        ..Default::default()
    }
}

pub fn create_world_button(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut pkv: ResMut<PkvStore>,
    mut page: ResMut<CurrentPage>,
) {
    for (inter, child) in &mut interaction_query {
        match *inter {
            Interaction::Pressed => {
                *page = CurrentPage::CREATEWORLD;
                create_world(&mut pkv);
            }
            Interaction::Hovered => {
                //mouse cursor
            }
            _ => {}
        }
    }
}

//Select
