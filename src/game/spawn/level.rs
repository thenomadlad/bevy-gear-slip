//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use crate::game::spawn::gear::{RotationDirection, SpawnGear};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.

    // let's start with two gears
    commands.trigger(SpawnGear {
        position: Vec2::splat((100.0 / f32::sqrt(2.0)) - 8.0),
        initial_gear_step: true,
        direction: RotationDirection::Clockwise,
    });
    commands.trigger(SpawnGear {
        position: Vec2::splat((-100.0 / f32::sqrt(2.0)) + 8.0),
        initial_gear_step: false,
        direction: RotationDirection::CounterClockwise,
    });

    // one day..
    // commands.trigger(SpawnPlayer);
}
