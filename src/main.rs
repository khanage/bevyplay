use bevy::prelude::*;

mod application;
mod asset_loader;
mod asteroid;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod movement;
mod pausemenu;
mod schedule;
mod spaceship;

fn main() {
    App::new()
        .add_plugins(application::AppPlugin)
        .add_plugins(pausemenu::PauseMenuPlugin)
        .add_plugins(debug::DebugPlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(spaceship::SpaceshipPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(asteroid::AsteroidPlugin)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(collision_detection::CollisionDetectionPlugin)
        .add_plugins(despawn::DespawnPlugin)
        .add_plugins(schedule::SchedulePlugin)
        .run();
}
