//! Spawn the player.

use bevy::prelude::*;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        rotational_movement::{RevolutionMovement, RotationDirection},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer {
    pub position: Vec2,
    pub revolution_radius: f32,
    pub initial_gear_step: bool,
    pub rotation_direction: RotationDirection,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(300), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    let spawn_params = trigger.event();
    let starting_position = spawn_params.position.extend(5.0);

    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: image_handles[&ImageKey::Sootboi].clone_weak(),
            transform: Transform::from_scale(Vec2::splat(0.2).extend(1.0))
                .with_translation(starting_position),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        // MovementController::default(),
        // Movement { speed: 420.0 },
        // ConstrainWithinWindow,
        player_animation,
        RevolutionMovement::new(
            starting_position,
            spawn_params.rotation_direction,
            spawn_params.revolution_radius,
            spawn_params.initial_gear_step,
        ),
        StateScoped(Screen::Playing),
    ));
}
