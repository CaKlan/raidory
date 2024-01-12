use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Commands, Query, Res, ResMut},
    },
    math::Vec2,
    prelude::{Deref, DerefMut},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer},
    transform::components::Transform,
};

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct AnimatedObject {
    name: String,
    // collidable: bool,
}

#[derive(Bundle)]
pub struct AnimatedObjectBundle {
    object: AnimatedObject,
    sprite: SpriteSheetBundle,
    animation_indices: AnimationIndices,
    timer: AnimationTimer,
}

pub fn spawn_tree(
    commands: &mut Commands,
    server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    tree_type: u32,
    transform: Transform,
) {
    let w = 129.;
    let h = 137.;
    let texture = server.load("sprites/trees/anim_tree/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(w, h), 6, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 1, last: 29 };

    commands.spawn(AnimatedObjectBundle {
        object: AnimatedObject {
            name: "tree1".to_string(),
        },
        sprite: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform,
            ..Default::default()
        },
        animation_indices,
        timer: AnimationTimer(Timer::from_seconds(0.1, bevy::time::TimerMode::Repeating)),
    });
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}
