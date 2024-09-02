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
    let first_position = Vec2::new(0.0, (100.0 / f32::sqrt(2.0)) + 2.0);

    // let's start with two gears
    commands.trigger(SpawnGear {
        position: first_position,
        initial_gear_step: false,
        direction: RotationDirection::Clockwise,
        color: Color::srgba(0.412, 1.000, 0.917, 1.000),
    });
    commands.trigger(SpawnGear {
        position: Vec2::new(108.0, -100.0 / f32::sqrt(2.0)),
        initial_gear_step: false,
        direction: RotationDirection::CounterClockwise,
        color: Color::srgba(0.919, 0.971, 0.463, 1.000),
    });
    commands.trigger(SpawnGear {
        position: Vec2::new(-108.0, -100.0 / f32::sqrt(2.0)),
        initial_gear_step: false,
        direction: RotationDirection::CounterClockwise,
        color: Color::srgba(1.000, 0.625, 0.625, 1.000),
    });

    commands.trigger(SpawnPlayer {
        position: first_position,
        revolution_radius: 92.0,
        initial_gear_step: true,
        rotation_direction: RotationDirection::Clockwise,
    });
}
