use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res, Resource},
    },
    ui::Style,
};
use bevy_render::view::Visibility;

#[derive(Resource, PartialEq, Eq)]
pub enum CurrentPage {
    ALWAYS,
    MENU,
    CREATEWORLD,
    LOADWORLD,
    SETTINGS,
}

#[derive(Component)]
pub struct Page(pub CurrentPage);

pub fn ui_navigation(page: Res<CurrentPage>, mut ui: Query<(&mut Visibility, &Page), With<Page>>) {
    for (mut visibility, ui_page) in &mut ui {
        *visibility = if *page == ui_page.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
