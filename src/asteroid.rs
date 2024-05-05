use bevy::{core::Zeroable, prelude::*};
use rand::{distributions::Distribution, Rng};
use std::{ops::Range, time::Duration};

use crate::{
    application::AppState,
    asset_loader::SceneAssets,
    collision_detection::Collider,
    end_game::DespawnAtEndgame,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    schedule::InGameSet,
    spaceship::Spaceship,
};

const SPAWN_RANGE_X: Range<f32> = -50.0..50.0;
const SPAWN_RANGE_Y: Range<f32> = -50.0..50.0;

const SPAWN_TIMER: f32 = 1.0;

const VELOCITY_SCALAR: f32 = 5.0;

const ROTATION_SPEED: f32 = 1.5;
const ASTEROID_RADIUS: f32 = 1.0;

#[derive(Component, Debug, Reflect)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

fn spawn_asteroid_on_interval(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    assets: Res<SceneAssets>,
    spaceship: Query<(&GlobalTransform, &Collider), With<Spaceship>>,
) {
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.just_finished() {
        let Ok(spaceship) = spaceship.get_single() else {
            error!("Didn't find a spaceship");
            return;
        };
        spawn_asteroid(&mut commands, &assets, spaceship);
    }
}

fn spawn_asteroid(
    commands: &mut Commands,
    assets: &Res<SceneAssets>,
    (spaceship_transform, spaceship_collider): (&GlobalTransform, &Collider),
) {
    let mut rng = rand::thread_rng();
    let distribution = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);

    let translation = loop {
        let potential_spawn_point = Vec3::new(
            rng.gen_range(SPAWN_RANGE_X),
            0.,
            rng.gen_range(SPAWN_RANGE_Y),
        );

        let distance = spaceship_transform
            .translation()
            .distance(potential_spawn_point);

        if distance > spaceship_collider.radius + (ASTEROID_RADIUS * 3.) {
            break potential_spawn_point;
        }
    };

    let mut random_unit_vector = || {
        Vec3::new(
            distribution.sample(&mut rng),
            0.,
            distribution.sample(&mut rng),
        )
    };

    let velocity = random_unit_vector() * VELOCITY_SCALAR;

    info!("Spawning asteroid velocity: {velocity:?}");

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(Vec3::zeroed()),
            model: SceneBundle {
                scene: assets.asteroids.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
            collider: Collider::new(ASTEROID_RADIUS),
        },
        Asteroid,
        DespawnAtEndgame,
        Name::new("asteroid"),
    ));
}

pub fn spawn_initial_asteroids(
    mut commands: Commands,
    assets: Res<SceneAssets>,
    spaceship_query: Query<(&GlobalTransform, &Collider), With<Spaceship>>,
) {
    let Ok(spaceship) = spaceship_query.get_single() else {
        error!("Unable to spawn initial asteroids");
        return;
    };

    for _ in 0..5 {
        spawn_asteroid(&mut commands, &assets, spaceship);
    }
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
        .register_type::<Asteroid>()
        .add_systems(
            Update,
            (spawn_asteroid_on_interval, rotate_asteroids)
                .in_set(InGameSet::EntityUpdates)
                .run_if(in_state(AppState::InGame)),
        );
    }
}
