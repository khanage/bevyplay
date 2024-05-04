use bevy::prelude::*;
use bevy_inspector_egui::egui;

use crate::application::AppState;

#[derive(Debug, Component)]
pub struct DespawnAtEndgame;

pub struct EndGamePlugin;

impl Plugin for EndGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (end_game, keyboad_controls).run_if(in_state(AppState::EndGame)),
        )
        .add_systems(
            OnTransition {
                from: AppState::InGame,
                to: AppState::EndGame,
            },
            despawn_everything,
        );
    }
}

fn despawn_everything(mut commands: Commands, despawners: Query<Entity, With<DespawnAtEndgame>>) {
    info!("Despawning all entities");
    for entity in despawners.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn keyboad_controls(
    mut app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        app_state.set(AppState::MainMenu);
    }
    #[cfg(not(target_arch = "wasm32"))]
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        exit.send(bevy::app::AppExit);
    }
}

fn end_game(
    mut contexts: bevy_inspector_egui::bevy_egui::EguiContexts,
    mut app_state: ResMut<NextState<AppState>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.allocate_space(egui::Vec2::new(1.0, 300.0));
        ui.label("End game");
        if ui.button("[M]ain Menu").clicked() {
            app_state.set(AppState::MainMenu);
        };

        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("[Q]uit").clicked() {
            exit.send(bevy::app::AppExit);
        }
    });
}
