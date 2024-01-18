pub mod health;

use self::health::Health;
use crate::{
    application::AppState,
    asset_loader::SceneAssets,
    collision_detection::Collider,
    end_game::DespawnAtEndgame,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    schedule::InGameSet,
};
use bevy::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);

const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIUS: f32 = 5.0;

const MISSILE_RADIUS: f32 = 1.0;
const MISSILE_SPEED: f32 = 10.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;

pub const STARTING_HEALTH: u32 = 3;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct Missile;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

#[derive(Component, Debug)]
pub struct ShieldDisplay;

#[derive(Component, Debug)]
pub struct AlreadyFired;

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };

    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::S) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::W) {
        movement = SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::D) {
        rotation = -SPACESHIP_ROTATION * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::A) {
        rotation = SPACESHIP_ROTATION * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::Q) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::E) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    transform.rotate_y(rotation);
    transform.rotate_z(roll);

    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<(Entity, &Transform), (With<Spaceship>, Without<AlreadyFired>)>,
    keyboard_input: Res<Input<KeyCode>>,
    assets: Res<SceneAssets>,
) {
    if !keyboard_input.pressed(KeyCode::Space) {
        return;
    }

    let Ok((spaceship_entity, spaceship_transform)) = query.get_single() else {
        return;
    };

    commands
        .get_entity(spaceship_entity)
        .unwrap()
        .insert(AlreadyFired);

    commands.spawn((
        Missile,
        MovingObjectBundle {
            velocity: Velocity::new(-spaceship_transform.forward() * MISSILE_SPEED),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: assets.missiles.clone(),
                transform: Transform::from_translation(
                    spaceship_transform.translation
                        + -spaceship_transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                ),
                ..default()
            },
            collider: Collider::new(MISSILE_RADIUS),
        },
        AudioBundle {
            source: assets.explosion.clone(),
            ..default()
        },
        DespawnAtEndgame,
    ));
}

fn spaceship_shield_controls(
    mut commands: Commands,
    query: Query<Entity, (With<Spaceship>, Without<SpaceshipShield>)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(spaceship) = query.get_single() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Tab) {
        info!("Spawning shield");

        let mesh = meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 6.1,
                subdivisions: 7,
                ..default()
            })
            .unwrap(),
        );

        info!("Mesh created");

        let material = materials.add(StandardMaterial {
            base_color: Color::BLUE.with_a(0.5),
            alpha_mode: AlphaMode::Premultiplied,
            ..default()
        });

        info!("Material created");

        commands
            .entity(spaceship)
            .insert(SpaceshipShield)
            .with_children(|builder| {
                builder.spawn((
                    PbrBundle {
                        mesh,
                        material,
                        ..default()
                    },
                    ShieldDisplay,
                ));
            });
    }
}

fn spawn_spaceship(mut commands: Commands, assets: Res<SceneAssets>) {
    commands.spawn((
        Spaceship,
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
            collider: Collider::new(SPACESHIP_RADIUS),
        },
        Health::default(),
        DespawnAtEndgame,
    ));
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                from: AppState::MainMenu,
                to: AppState::InGame,
            },
            spawn_spaceship,
        )
        .add_systems(
            Update,
            (
                spaceship_weapon_controls,
                spaceship_movement_controls,
                spaceship_shield_controls,
            )
                .chain()
                .in_set(InGameSet::UserInput)
                .run_if(in_state(AppState::InGame)),
        );
    }
}
