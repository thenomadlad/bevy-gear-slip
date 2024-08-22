//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{screen::GameButtonAction, ui::prelude::*, AppSet};

const MAX_ANGULAR_VELOCITY: f32 = 4.0;
const MIN_ANGULAR_VELOCITY: f32 = 0.25;
const ANGULAR_VELOCITY_STEP: f32 = 2.0;

const DEFAULT_GEAR_ROTATION: f32 = 30.0;
const DEFAULT_GEAR_ROTATION_INITIAL_STEP: f32 = 15.0;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<AngularVelocity>();

    // Apply movement based on controls.
    app.register_type::<RotationalMovement>();
    app.add_systems(Update, apply_rotational_movement.in_set(AppSet::Update));
    app.add_systems(Update, apply_revolutional_movement.in_set(AppSet::Update));
    app.add_systems(Update, handle_increase_velocity.in_set(AppSet::Update));
}

#[derive(Resource)]
pub struct AngularVelocity(f32);

impl Default for AngularVelocity {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Debug, Reflect, Clone, Copy)]
pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

impl RotationDirection {
    fn to_angular_velocity(&self) -> f32 {
        DEFAULT_GEAR_ROTATION.to_radians()
            * match self {
                RotationDirection::Clockwise => -1.0,
                RotationDirection::CounterClockwise => 1.0,
            }
    }

    fn opposite(&self) -> RotationDirection {
        match self {
            RotationDirection::Clockwise => RotationDirection::CounterClockwise,
            RotationDirection::CounterClockwise => RotationDirection::Clockwise,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RotationalMovement {
    angular_velocity: f32,
    direction: RotationDirection,
    include_initial_step: bool,
}

impl RotationalMovement {
    pub fn new(direction: RotationDirection, include_initial_step: bool) -> RotationalMovement {
        RotationalMovement {
            angular_velocity: direction.to_angular_velocity(),
            direction,
            include_initial_step,
        }
    }

    pub fn get_initial_rotation(&self) -> f32 {
        if self.include_initial_step {
            DEFAULT_GEAR_ROTATION_INITIAL_STEP.to_radians()
        } else {
            0.0
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RevolutionMovement {
    pub anchor: Vec3,
    angular_velocity: f32,
    rotation_direction: RotationDirection,
    revolution_radius: f32,
    current_rotation: f32,
    pub position: Vec3,
}

impl RevolutionMovement {
    pub fn new(
        anchor: Vec3,
        rotation_direction: RotationDirection,
        revolution_radius: f32,
        initial_gear_step: bool,
    ) -> Self {
        Self {
            anchor,
            angular_velocity: rotation_direction.to_angular_velocity(),
            rotation_direction,
            revolution_radius,
            current_rotation: if initial_gear_step {
                f32::to_radians(DEFAULT_GEAR_ROTATION_INITIAL_STEP)
            } else {
                0.0
            },
            position: anchor, // is ok let it be
        }
    }

    pub fn move_onto_gear(&mut self, anchor: Vec3, rotational_movement: &RotationalMovement) {
        self.anchor = anchor;
        self.angular_velocity = rotational_movement.angular_velocity;
        self.rotation_direction = self.rotation_direction.opposite();
        self.current_rotation = {
            let diff = self.position.xy() - anchor.xy();
            let normed = diff.normalize();
            println!("{diff} -> {normed}");

            let cos_t = normed.x;
            let sin_t = normed.y;

            let principal_value = (sin_t / cos_t).atan();

            principal_value + if cos_t < 0.0 { PI } else { 0.0 }
        } + match self.rotation_direction {
            RotationDirection::CounterClockwise => {
                -1.0 * DEFAULT_GEAR_ROTATION_INITIAL_STEP.to_radians()
            }
            RotationDirection::Clockwise => DEFAULT_GEAR_ROTATION_INITIAL_STEP.to_radians(),
        };
    }

    pub fn update_position(&mut self, speed: f32, delta: f32) {
        let rotation_diff = self.angular_velocity * speed * delta;
        let total_rotation = self.current_rotation + rotation_diff;

        let rotated_position = self.revolution_radius
            * Vec2::new(f32::cos(total_rotation), f32::sin(total_rotation)).extend(0.0);

        self.position = self.anchor + rotated_position;
        self.current_rotation = total_rotation;
    }
}

fn apply_rotational_movement(
    time: Res<Time>,
    speed: Res<AngularVelocity>,
    mut movement_query: Query<(&RotationalMovement, &mut Transform)>,
) {
    for (movement, mut transform) in &mut movement_query {
        transform.rotate_z(movement.angular_velocity * speed.0 * time.delta_seconds());
    }
}

fn apply_revolutional_movement(
    time: Res<Time>,
    speed: Res<AngularVelocity>,
    mut movement_query: Query<(&mut RevolutionMovement, &mut Transform)>,
) {
    for (mut movement, mut transform) in &mut movement_query {
        movement.update_position(speed.0, time.delta_seconds());
        transform.translation = movement.position;
    }
}

fn handle_increase_velocity(
    mut angular_velocity: ResMut<AngularVelocity>,
    mut button_query: InteractionQuery<&GameButtonAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            angular_velocity.0 = match action {
                GameButtonAction::IncreaseSpeed => f32::min(
                    MAX_ANGULAR_VELOCITY,
                    angular_velocity.0 * ANGULAR_VELOCITY_STEP,
                ),
                GameButtonAction::DecreaseSpeed => f32::max(
                    MIN_ANGULAR_VELOCITY,
                    angular_velocity.0 / ANGULAR_VELOCITY_STEP,
                ),
            }
        }
    }
}
