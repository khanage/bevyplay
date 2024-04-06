use bevy::prelude::*;
use bevy_health_bar3d::configuration::{BarHeight, BarSettings, Percentage};
use std::time::Duration;

use crate::end_game::DespawnAtEndgame;

use super::Spaceship;

const SHIELD_TIME: u64 = 1200;

#[derive(Component, Debug, Reflect, Resource)]
pub struct SpaceshipShield {
    timer: Timer,
}

impl Percentage for SpaceshipShield {
    fn value(&self) -> f32 {
        1. - self.timer.fraction()
    }
}

#[derive(Component, Debug, Reflect, Resource)]
pub struct ShieldDisplay;

pub fn enable_shields(
    mut commands: Commands,
    query: Query<Entity, (With<Spaceship>, Without<SpaceshipShield>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyF) {
        return;
    }

    let Ok(spaceship) = query.get_single() else {
        return;
    };

    let mesh = meshes.add(Sphere {
        radius: 6.1,
        ..default()
    });

    let material = materials.add(StandardMaterial {
        base_color: Color::BLUE.with_a(0.5),
        alpha_mode: AlphaMode::Premultiplied,
        ..default()
    });

    commands
        .entity(spaceship)
        .insert(SpaceshipShield {
            timer: Timer::new(Duration::from_millis(SHIELD_TIME), TimerMode::Once),
        })
        .insert(BarSettings::<SpaceshipShield> {
            offset: 15.,
            height: BarHeight::Static(1.),
            width: 10.,
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                PbrBundle {
                    mesh,
                    material,
                    ..default()
                },
                ShieldDisplay,
                DespawnAtEndgame,
            ));
        });
}

pub fn disable_shields(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SpaceshipShield)>,
    spaceship_shield_query: Query<Entity, With<ShieldDisplay>>,
    time: Res<Time>,
) {
    let Ok((spaceship, mut shield)) = query.get_single_mut() else {
        return;
    };

    shield.timer.tick(time.delta());

    if !shield.timer.finished() {
        return;
    }

    let Ok(shield_display) = spaceship_shield_query.get_single() else {
        error!("Couldn't find a shield display");
        return;
    };

    commands.entity(spaceship).remove::<SpaceshipShield>();
    commands.entity(shield_display).despawn_recursive();
}
