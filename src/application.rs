use bevy::{app::AppExit, prelude::*};
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
}

fn main_menu(
    mut contexts: EguiContexts,
    mut app_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    egui::SidePanel::left("Side panel")
        .default_width(200.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.allocate_space(egui::Vec2::new(1.0, 300.0));

            ui.heading("Welcome to not-pong");

            ui.horizontal(|ui| {
                ui.label("Write someting");
            });

            if ui.button("New game").clicked() {
                app_state.set(AppState::InGame);
            }

            if ui.button("Quit").clicked() {
                exit.send(AppExit);
            }
        });
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
            .insert_resource(AmbientLight {
                color: Color::default(),
                brightness: 0.75,
            })
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(EguiPlugin)
            .add_systems(
                Update,
                main_menu
                    .in_set(InGameSet::EntityUpdates)
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_state::<AppState>();
    }
}
