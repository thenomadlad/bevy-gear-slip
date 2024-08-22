//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use crate::game::{
    rotational_movement::RotationDirection,
    spawn::{gear::SpawnGear, player::SpawnPlayer},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    let first_position = Vec2::splat((100.0 / f32::sqrt(2.0)) - 8.0);

    // let's start with two gears
    commands.trigger(SpawnGear {
        position: first_position,
        initial_gear_step: true,
        direction: RotationDirection::Clockwise,
    });
    commands.trigger(SpawnGear {
        position: Vec2::splat((-100.0 / f32::sqrt(2.0)) + 8.0),
        initial_gear_step: false,
        direction: RotationDirection::CounterClockwise,
    });

    commands.trigger(SpawnPlayer {
        position: first_position,
        revolution_radius: 92.0,
        initial_gear_step: true,
        rotation_direction: RotationDirection::Clockwise,
    });
}
