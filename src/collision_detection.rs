use bevy::{prelude::*, utils::HashMap};

use crate::{
    application::AppState,
    asteroid::Asteroid,
    schedule::InGameSet,
    spaceship::{health::Health, AlreadyFired, Missile, ShieldDisplay, Spaceship, SpaceshipShield},
};

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (a, transform_a, collider_a) in query.iter() {
        for (b, transform_b, collider_b) in query.iter() {
            if a == b {
                continue;
            }

            let distance = transform_a
                .translation()
                .distance(transform_b.translation());

            if distance < collider_a.radius + collider_b.radius {
                colliding_entities.entry(a).or_insert_with(Vec::new).push(b);
            }
        }
    }

    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();

        if let Some(collisions) = colliding_entities.get(&entity) {
            collider.colliding_entities.extend(collisions);
        }
    }
}

fn handle_spaceship_collision(
    mut commands: Commands,
    mut spaceship_query: Query<
        (Entity, &mut Health, &Collider, Option<&SpaceshipShield>),
        With<Spaceship>,
    >,
    mut app_state: ResMut<NextState<AppState>>,
    spaceship_shield_query: Query<Entity, With<ShieldDisplay>>,
    asteroids: Query<Entity, With<Asteroid>>,
) {
    let Ok((spaceship_entity, mut spaceship_health, spaceship_collider, maybe_shield)) =
        spaceship_query.get_single_mut()
    else {
        return;
    };

    for &collided_entity in spaceship_collider.colliding_entities.iter() {
        let Ok(asteroid) = asteroids.get(collided_entity) else {
            continue;
        };

        commands.entity(asteroid).despawn_recursive();

        if maybe_shield.is_some() {
            info!("Despawning shield");

            commands
                .entity(spaceship_entity)
                .remove::<SpaceshipShield>();

            let Ok(shield) = spaceship_shield_query.get_single() else {
                error!("No shield found");
                break;
            };

            commands.entity(shield).despawn_recursive();
        } else {
            *spaceship_health -= 1;

            if *spaceship_health < 1 {
                warn!("Time to die!");
                app_state.set(AppState::EndGame);
            }
        }

        break;
    }
}

fn handle_asteroid_collision(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<Asteroid>>,
    missiles: Query<Entity, With<Missile>>,
    spaceship: Query<Entity, With<AlreadyFired>>,
) {
    for (asteroid_entity, asteroid_collider) in query.iter() {
        for &colliding_entity in asteroid_collider.colliding_entities.iter() {
            let Ok(missile_entity) = missiles.get(colliding_entity) else {
                continue;
            };

            commands.entity(missile_entity).despawn_recursive();
            commands.entity(asteroid_entity).despawn_recursive();

            if let Ok(spaceship_entity) = spaceship.get_single() {
                commands.entity(spaceship_entity).remove::<AlreadyFired>();
            }
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection
                .in_set(InGameSet::CollisionDetection)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (handle_spaceship_collision, handle_asteroid_collision)
                .in_set(InGameSet::DespawnEntities)
                .run_if(in_state(AppState::InGame)),
        );
    }
}
