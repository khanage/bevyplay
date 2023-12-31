use bevy::{prelude::*, utils::HashMap};

use crate::{application::AppState, asteroid::Asteroid, schedule::InGameSet, spaceship::Spaceship};

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliditing_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliditing_entities: vec![],
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
        collider.colliditing_entities.clear();

        if let Some(collisions) = colliding_entities.get(&entity) {
            collider.colliditing_entities.extend(collisions);
        }
    }
}

fn handle_collisions<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliditing_entities.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }

            commands.entity(entity).despawn_recursive();
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
            (
                handle_collisions::<Asteroid>,
                handle_collisions::<Spaceship>,
            )
                .in_set(InGameSet::DespawnEntities)
                .run_if(in_state(AppState::InGame)),
        );
    }
}
