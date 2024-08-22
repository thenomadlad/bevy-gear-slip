//! Spawn the player.

use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        rotational_movement::{RotationDirection, RotationalMovement},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_gear);
    app.register_type::<Gear>();
}

#[derive(Event, Debug)]
pub struct SpawnGear {
    pub position: Vec2,
    pub initial_gear_step: bool,
    pub direction: RotationDirection,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Gear;

#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct GearBoundingBox(pub Rect);

fn spawn_gear(
    trigger: Trigger<SpawnGear>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    assets: Res<Assets<Image>>,
) {
    let gear = trigger.event();
    let rotational_movement = RotationalMovement::new(gear.direction, gear.initial_gear_step);
    let transform = Transform::from_translation(gear.position.extend(1.0)).with_rotation(
        Quat::from_rotation_z(rotational_movement.get_initial_rotation()),
    );

    let texture = image_handles[&ImageKey::Gear].clone_weak();
    let image_dimensions = assets.get(&texture).unwrap().size();
    let scaled_image_dimension = image_dimensions.as_vec2() * transform.scale.truncate();
    let bounding_box =
        Rect::from_center_size(transform.translation.truncate(), scaled_image_dimension);

    commands.spawn((
        Name::new("Gear"),
        Gear,
        SpriteBundle {
            texture,
            transform,
            ..Default::default()
        },
        GearBoundingBox(bounding_box),
        rotational_movement,
        StateScoped(Screen::Playing),
    ));
}
