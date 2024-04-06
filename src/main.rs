use bevy::prelude::*;

mod application;
mod asset_loader;
mod asteroid;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod end_game;
mod movement;
mod pausemenu;
mod schedule;
mod spaceship;
mod ui;

fn main() {
    let mut application = App::new();

    application
        .add_plugins(application::AppPlugin)
        .add_plugins(pausemenu::PauseMenuPlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(spaceship::SpaceshipPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(asteroid::AsteroidPlugin)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(collision_detection::CollisionDetectionPlugin)
        .add_plugins(despawn::DespawnPlugin)
        .add_plugins(schedule::SchedulePlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(end_game::EndGamePlugin)
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default());

    if cfg!(feature = "diagnostics") {
        application.add_plugins(debug::DebugPlugin);
    }

    application.run();
}
