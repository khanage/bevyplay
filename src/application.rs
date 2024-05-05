use bevy::{asset::AssetMetaCheck, log::LogPlugin, prelude::*};
use bevy_inspector_egui::{
    bevy_egui::{EguiContexts, EguiPlugin},
    egui,
};

use crate::asteroid::spawn_initial_asteroids;
use crate::schedule::InGameSet;
use crate::spaceship::spawn_spaceship;

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

            if ui.button("[N]ew game").clicked() {
                app_state.set(AppState::InGame);
            }

            #[cfg(not(target_arch = "wasm32"))]
            if ui.button("[Q]uit").clicked() {
                app_state.set(AppState::EndGame);
            }
        });
}

fn main_menu_keys(
    mut app_state: ResMut<NextState<AppState>>,
    keyboad_input: Res<ButtonInput<KeyCode>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    if keyboad_input.just_pressed(KeyCode::KeyN) {
        app_state.set(AppState::InGame);
    }

    #[cfg(not(target_arch = "wasm32"))]
    if keyboad_input.just_pressed(KeyCode::KeyQ) {
        exit.send(bevy::app::AppExit);
    }
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetMetaCheck::Never)
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            #[cfg(target_arch = "wasm32")]
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
                (main_menu, main_menu_keys)
                    .in_set(InGameSet::EntityUpdates)
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                OnTransition {
                    from: AppState::MainMenu,
                    to: AppState::InGame,
                },
                (spawn_spaceship, spawn_initial_asteroids).chain(),
            )
            .init_state::<AppState>();
    }
}
