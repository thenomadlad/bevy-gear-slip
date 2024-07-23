//! Spawn the player.

use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        rotational_movement::RotationalMovement,
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

#[derive(Debug)]
pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Gear;

fn spawn_gear(
    trigger: Trigger<SpawnGear>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    let gear = trigger.event();
    let rotation_speed = match gear.direction {
        RotationDirection::Clockwise => -1.0,
        RotationDirection::CounterClockwise => 1.0,
    } * f32::to_radians(30.0);
    let initial_rotation = if gear.initial_gear_step {
        f32::to_radians(15.0)
    } else {
        0.0
    };

    commands.spawn((
        Name::new("Gear"),
        Gear,
        SpriteBundle {
            texture: image_handles[&ImageKey::Gear].clone_weak(),
            transform: Transform::from_translation(gear.position.extend(1.0))
                .with_rotation(Quat::from_rotation_z(initial_rotation)),
            ..Default::default()
        },
        RotationalMovement { rotation_speed },
        StateScoped(Screen::Playing),
    ));
}
