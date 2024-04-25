use bevy::{asset::AssetMetaCheck, log::LogPlugin, prelude::*};
use bevy_inspector_egui::{
    bevy_egui::{EguiContexts, EguiPlugin},
    egui,
};

use crate::schedule::InGameSet;

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    Paused,
    EndGame,
}

fn main_menu(mut contexts: EguiContexts, mut app_state: ResMut<NextState<AppState>>) {
    egui::SidePanel::left("Side panel")
        .default_width(200.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.allocate_space(egui::Vec2::new(1.0, 300.0));

            ui.heading("Welcome to not-pong");

            ui.horizontal(|ui| {
                ui.label("Testing that deployment works");
            });

            if ui.button("New game").clicked() {
                app_state.set(AppState::InGame);
            }

            #[cfg(not(target_arch = "wasm32"))]
            if ui.button("Quit").clicked() {
                app_state.set(AppState::EndGame);
            }
        });
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetMetaCheck::Never)
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            canvas: Some("#pong-canvas".into()),
                            prevent_default_event_handling: false,
                            ..default()
                        }),
                        ..default()
                    })
                    .set(LogPlugin {
                        filter: "wgpu=error,bevy_render=info,bevy_ecs=info".into(),
                        level: bevy::log::Level::INFO,
                        ..default()
                    }),
            )
            .add_plugins(EguiPlugin)
            .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 0.02,
            })
            .add_systems(
                Update,
                main_menu
                    .in_set(InGameSet::EntityUpdates)
                    .run_if(in_state(AppState::MainMenu)),
            )
            .init_state::<AppState>();
    }
}
