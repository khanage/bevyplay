use bevy::{app::AppExit, prelude::*};
use bevy_inspector_egui::egui;

use crate::application::AppState;

#[derive(Debug, Component)]
pub struct DespawnAtEndgame;

pub struct EndGamePlugin;

impl Plugin for EndGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, end_game.run_if(in_state(AppState::EndGame)))
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

fn end_game(
    mut contexts: bevy_inspector_egui::bevy_egui::EguiContexts,
    mut app_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.allocate_space(egui::Vec2::new(1.0, 300.0));
        ui.label("End game");
        if ui.button("Main Menu").clicked() {
            app_state.set(AppState::MainMenu);
        };

        if ui.button("Quit").clicked() {
            exit.send(AppExit);
        }
    });
}
