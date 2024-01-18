use bevy::prelude::*;
use rand::Rng;
use std::{ops::Range, time::Duration};

use crate::{
    application::AppState,
    asset_loader::SceneAssets,
    collision_detection::Collider,
    end_game::DespawnAtEndgame,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    schedule::InGameSet,
};

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Y: Range<f32> = 0.0..25.0;

const SPAWN_TIMER: f32 = 1.0;

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;

const ROTATION_SPEED: f32 = 1.5;
const ASTEROID_RADIUS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.,
        rng.gen_range(SPAWN_RANGE_Y),
    );

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();

    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(acceleration),
            model: SceneBundle {
                scene: assets.asteroids.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
            collider: Collider::new(ASTEROID_RADIUS),
        },
        Asteroid,
        DespawnAtEndgame,
    ));
}

fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATION_SPEED * time.delta_seconds());
    }
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::new(Duration::from_secs_f32(SPAWN_TIMER), TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (spawn_asteroid, rotate_asteroids)
                .in_set(InGameSet::EntityUpdates)
                .run_if(in_state(AppState::InGame)),
        );
    }
}
