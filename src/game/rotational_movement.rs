//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Speed>();

    // Apply movement based on controls.
    app.register_type::<RotationalMovement>();
    app.add_systems(Update, apply_rotational_movement.in_set(AppSet::Update));
}

#[derive(Resource)]
pub struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RotationalMovement {
    pub rotation_speed: f32,
}

fn apply_rotational_movement(
    time: Res<Time>,
    speed: Res<Speed>,
    mut movement_query: Query<(&RotationalMovement, &mut Transform)>,
) {
    for (movement, mut transform) in &mut movement_query {
        transform.rotate_z(movement.rotation_speed * speed.0 * time.delta_seconds());
    }
}
