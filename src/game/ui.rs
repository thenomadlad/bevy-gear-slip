use bevy::prelude::*;
use bevy_inspector_egui::egui::Align;

use crate::screen::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_gameplay);

    app.register_type::<GameButtonAction>();
    app.add_systems(
        Update,
        handle_game_button_action.run_if(in_state(Screen::Title)),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum GameButtonAction {
    IncreaseSpeed,
}

fn enter_gameplay(mut commands: Commands) {
    commands
        .ui_root_with_alignment(JustifyContent::End, AlignItems::End)
        .insert(StateScoped(Screen::Playing))
        .with_children(|children| {
            children
                .button("Increase")
                .insert(GameButtonAction::IncreaseSpeed);
        });
}

fn handle_game_button_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => next_screen.set(Screen::Playing),
                TitleAction::Credits => next_screen.set(Screen::Credits),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
