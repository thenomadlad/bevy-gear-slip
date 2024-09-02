use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    game::{
        rotational_movement::{RevolutionMovement, RotationalMovement},
        spawn::{
            gear::{Gear, GearBoundingBox},
            player::Player,
        },
    },
    screen::GameButtonAction,
    ui::interaction::InteractionQuery,
    AppSet,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        detect_collision_move
            .in_set(AppSet::Update)
            .run_if(input_just_pressed(KeyCode::Space).or_else(run_if_interaction_query)),
    );
}

fn detect_collision_move(
    gears: Query<(&Transform, &RotationalMovement, &GearBoundingBox), With<Gear>>,
    mut player: Query<&mut RevolutionMovement, With<Player>>,
) {
    let mut player_movement = player.single_mut();
    let position = player_movement.position.xy();
    let anchor = player_movement.anchor.xy();

    for (gear_transform, gear_rotation, &GearBoundingBox(bounding_box)) in &gears {
        let gear_pos = gear_transform.translation.xy();

        if (gear_pos != anchor) && (bounding_box.contains(position)) {
            let player_z = player_movement.anchor.z;
            player_movement.move_onto_gear(gear_pos.extend(player_z), gear_rotation);
        }
    }
}

fn run_if_interaction_query(mut button_query: InteractionQuery<&GameButtonAction>) -> bool {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            if let GameButtonAction::Jump = action {
                return true;
            }
        }
    }

    false
}
