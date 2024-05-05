use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

const CAMERA_DISTANCE: f32 = 120.;

#[derive(Component, Debug, Reflect, Resource)]
pub struct CameraBounds {
    pub window_bounds: Rect,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, window: Query<&Window>) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    };

    let left = -50.0;
    let bottom = -50.0;
    let right = 50.0;
    let top = 50.0;

    commands.spawn(camera);

    commands.insert_resource(CameraBounds {
        window_bounds: Rect::new(left, bottom, right, top),
    });

    commands.spawn(PointLightBundle {
        // transform: Transform::from_xyz(5.0, 8.0, 2.0),
        transform: Transform::from_xyz(1.0, -1.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        point_light: PointLight {
            intensity: 100_000.0,
            color: Color::RED,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
}
