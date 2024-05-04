use bevy::prelude::*;

use crate::{
    application::AppState,
    asteroid::Asteroid,
    camera::CameraBounds,
    schedule::InGameSet,
    spaceship::{AlreadyFired, Missile, Spaceship},
};

const DESPAWN_DISTANCE: f32 = 50.0;

fn despawn_far_away_asteroids(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), With<Asteroid>>,
    camera_bounds: Res<CameraBounds>,
) {
    for (entity, transform) in query.iter() {
        if !camera_bounds
            .window_bounds
            .contains(transform.translation().truncate())
        {
            info!(
                "Despawning asteroid at {:#?}",
                transform.translation().truncate()
            );
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_far_away_missiles(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), With<Missile>>,
    spaceship_query: Query<Entity, With<Spaceship>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation().distance(Vec3::ZERO) > DESPAWN_DISTANCE {
            info!("Despawning missiles");

            commands.entity(entity).despawn_recursive();
            if let Ok(spaceship) = spaceship_query.get_single() {
                commands.entity(spaceship).remove::<AlreadyFired>();
            }
        }
    }
}

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_far_away_asteroids, despawn_far_away_missiles)
                .chain()
                .in_set(InGameSet::DespawnEntities)
                .run_if(in_state(AppState::InGame)),
        );
    }
}
