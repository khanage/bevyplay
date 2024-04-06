pub mod health;
pub mod shield;

use self::{
    health::Health,
    shield::{disable_shields, enable_shields, ShieldDisplay, SpaceshipShield},
};
use crate::{
    application::AppState,
    asset_loader::SceneAssets,
    collision_detection::Collider,
    end_game::DespawnAtEndgame,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    schedule::InGameSet,
};
use bevy::prelude::*;
use bevy_health_bar3d::{
    configuration::{ColorScheme, ForegroundColor, Percentage},
    plugin::HealthBarPlugin,
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);

const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIUS: f32 = 5.0;

const MISSILE_RADIUS: f32 = 1.0;
const MISSILE_SPEED: f32 = 10.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;

pub const STARTING_HEALTH: u32 = 3;

#[derive(Component, Debug, Reflect, Resource)]
pub struct Spaceship;

#[derive(Component, Debug, Reflect, Resource)]
pub struct Missile;

#[derive(Component, Debug)]
pub struct AlreadyFired;

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };

    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -SPACESHIP_ROTATION * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = SPACESHIP_ROTATION * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyQ) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyE) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    transform.rotate_y(rotation);
    transform.rotate_z(roll);

    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<(Entity, &Transform), (With<Spaceship>, Without<AlreadyFired>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    assets: Res<SceneAssets>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
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
        app.add_plugins(HealthBarPlugin::<SpaceshipShield>::default())
            .insert_resource(
                ColorScheme::<SpaceshipShield>::new()
                    .foreground_color(ForegroundColor::Static(Color::BLUE)),
            )
            .add_systems(
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
                    disable_shields,
                    enable_shields,
                )
                    .chain()
                    .in_set(InGameSet::UserInput)
                    .run_if(in_state(AppState::InGame)),
            )
            .register_type::<Missile>()
            .register_type::<Spaceship>()
            .register_type::<ShieldDisplay>()
            .register_type::<SpaceshipShield>();
    }
}
