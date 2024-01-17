use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::prelude::*;

use crate::{
    application::AppState,
    asset_loader::SceneAssets,
    collision_detection::Collider,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    schedule::InGameSet,
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);

const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIUS: f32 = 5.0;

const MISSILE_RADIUS: f32 = 1.0;
const MISSILE_SPEED: f32 = 10.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct Missile;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

#[derive(Component, Debug)]
pub struct AlreadyFired;

#[derive(Component, Debug)]
pub struct Health(u32);

impl Add<u32> for Health {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<u32> for Health {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}

impl Sub<u32> for Health {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<u32> for Health {
    fn sub_assign(&mut self, rhs: u32) {
        self.0 -= rhs;
    }
}

impl PartialEq<u32> for Health {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u32> for Health {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(other))
    }
}

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
    ));
}

fn spaceship_shield_controls(
    mut commands: Commands,
    query: Query<Entity, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let Ok(spaceship) = query.get_single() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Tab) {
        commands.entity(spaceship).insert(SpaceshipShield);
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
        Health(1),
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
