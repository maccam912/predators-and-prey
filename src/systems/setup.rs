use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::resources::*;
use crate::systems::input::CameraController;

// ===== SETUP SYSTEM =====

pub fn setup(mut commands: Commands, config: Res<SimulationConfig>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
        CameraController::default(),
    ));

    let mut rng = rand::rng();

    // Spawn plants
    for _ in 0..config.initial_plants {
        let x = rng.random_range(-config.world_size.x / 2.0..config.world_size.x / 2.0);
        let y = rng.random_range(-config.world_size.y / 2.0..config.world_size.y / 2.0);

        commands.spawn((
            Plant,
            Genome::random_plant(),
            Energy(rng.random_range(20.0..50.0)),
            Age(0.0),
            Transform::from_xyz(x, y, 0.0),
            Sprite {
                color: Color::srgb(0.2, 0.8, 0.2),
                custom_size: Some(Vec2::splat(8.0)),
                ..default()
            },
        ));
    }

    // Spawn prey
    for _ in 0..config.initial_prey {
        let x = rng.random_range(-config.world_size.x / 2.0..config.world_size.x / 2.0);
        let y = rng.random_range(-config.world_size.y / 2.0..config.world_size.y / 2.0);

        commands.spawn((
            Prey,
            Genome::random_prey(),
            Energy(rng.random_range(40.0..80.0)),
            Age(0.0),
            Velocity(Vec2::ZERO),
            Stamina::default(),
            Transform::from_xyz(x, y, 1.0),
            Sprite {
                color: Color::srgb(0.3, 0.3, 0.9),
                custom_size: Some(Vec2::splat(12.0)),
                ..default()
            },
        ));
    }

    // Spawn predators
    for _ in 0..config.initial_predators {
        let x = rng.random_range(-config.world_size.x / 2.0..config.world_size.x / 2.0);
        let y = rng.random_range(-config.world_size.y / 2.0..config.world_size.y / 2.0);

        commands.spawn((
            Predator,
            Genome::random_predator(),
            Energy(rng.random_range(60.0..100.0)),
            Age(0.0),
            Velocity(Vec2::ZERO),
            HuntTarget(None),
            Transform::from_xyz(x, y, 2.0),
            Sprite {
                color: Color::srgb(0.9, 0.2, 0.2),
                custom_size: Some(Vec2::splat(16.0)),
                ..default()
            },
        ));
    }

    // Spawn UI text
    commands.spawn((
        Text::new("Population Stats"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}
