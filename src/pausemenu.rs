use bevy::prelude::*;
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
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    egui::SidePanel::left("Paused")
        .default_width(200.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.allocate_space(egui::Vec2::new(1.0, 300.0));
            ui.label("Currently paused");
            if ui.button("[U]npause").clicked() {
                app_state.set(AppState::InGame);
            };

            #[cfg(not(target_arch = "wasm32"))]
            if ui.button("[Q]uit").clicked() {
                exit.send(bevy::app::AppExit);
            }
        });
}

fn unpause_game(
    mut app_state: ResMut<NextState<AppState>>,
    keyboad_input: Res<ButtonInput<KeyCode>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    if keyboad_input.just_pressed(KeyCode::Escape) || keyboad_input.just_pressed(KeyCode::KeyU) {
        app_state.set(AppState::InGame);
    }

    #[cfg(not(target_arch = "wasm32"))]
    if keyboad_input.just_pressed(KeyCode::KeyQ) {
        exit.send(bevy::app::AppExit);
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
