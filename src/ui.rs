use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::EguiContexts,
    egui::{self, Color32, RichText},
};

use crate::{application::AppState, spaceship::health::Health};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            update_ui.run_if(in_state(AppState::InGame).or_else(in_state(AppState::Paused))),
        );
    }
}

#[derive(Debug, Component)]
pub struct Ui;

fn update_ui(mut contexts: EguiContexts, health: Query<&Health>) {
    let Ok(health) = health.get_single() else {
        return;
    };

    bevy_inspector_egui::egui::SidePanel::right("Game")
        .default_width(200.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.allocate_space(egui::Vec2::new(1.0, 300.0));
            ui.label(RichText::new(format!("Health: {health}")).color(Color32::RED));
        });
}
