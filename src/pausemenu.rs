use bevy::{app::AppExit, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui};

use crate::{application::AppState, schedule::InGameSet};

fn pause_game(
    mut app_state: ResMut<NextState<AppState>>,
    keyboad_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboad_input.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::Paused);
    }
}

fn pause_menu(
    mut contexts: EguiContexts,
    mut app_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    egui::SidePanel::left("Paused")
        .default_width(200.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.allocate_space(egui::Vec2::new(1.0, 300.0));
            ui.label("Currently paused");
            if ui.button("Unpause").clicked() {
                app_state.set(AppState::InGame);
            };

            #[cfg(not(target_arch = "wasm"))]
            if ui.button("Quit").clicked() {
                exit.send(AppExit);
            }
        });
}

fn unpause_game(
    mut app_state: ResMut<NextState<AppState>>,
    keyboad_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboad_input.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::InGame);
    }
}

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            pause_game
                .in_set(InGameSet::UserInput)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            unpause_game
                .in_set(InGameSet::UserInput)
                .run_if(in_state(AppState::Paused)),
        )
        .add_systems(
            Update,
            pause_menu
                .in_set(InGameSet::EntityUpdates)
                .run_if(in_state(AppState::Paused)),
        );
    }
}
