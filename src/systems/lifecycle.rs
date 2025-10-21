use bevy::prelude::*;
use rand::Rng;

use crate::components::*;

// ===== QUERY TYPE ALIASES =====

type EnergyConsumptionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Energy, &'static Genome, &'static Velocity),
    Or<(With<Plant>, With<Prey>, With<Predator>, With<Scavenger>)>,
>;

type DeathSystemQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static Energy, &'static Age),
    Or<(With<Plant>, With<Prey>, With<Predator>, With<Scavenger>)>,
>;

type AgeSystemQuery<'w, 's> =
    Query<'w, 's, &'static mut Age, Or<(With<Plant>, With<Prey>, With<Predator>, With<Scavenger>)>>;

// ===== LIFECYCLE SYSTEMS =====

pub fn energy_consumption_system(mut organisms: EnergyConsumptionQuery, time: Res<Time>) {
    for (mut energy, genome, velocity) in organisms.iter_mut() {
        let base_cost = genome.metabolism * genome.size * time.delta_secs();
        let movement_cost = velocity.0.length() * 0.01 * time.delta_secs();
        energy.0 -= base_cost + movement_cost;
    }
}

pub fn age_system(mut organisms: AgeSystemQuery, time: Res<Time>) {
    for mut age in organisms.iter_mut() {
        age.0 += time.delta_secs();
    }
}

pub fn reproduction_system(
    mut commands: Commands,
    plants: Query<(Entity, &Transform, &Energy, &Genome), With<Plant>>,
    prey: Query<(Entity, &Transform, &Energy, &Genome), With<Prey>>,
    predators: Query<(Entity, &Transform, &Energy, &Genome), With<Predator>>,
    scavengers: Query<(Entity, &Transform, &Energy, &Genome), With<Scavenger>>,
) {
    let mut rng = rand::rng();

    // Count populations for density-dependent reproduction
    let prey_count = prey.iter().count();
    let predator_count = predators.iter().count();
    let scavenger_count = scavengers.iter().count();

    // Plant reproduction
    for (entity, transform, energy, genome) in plants.iter() {
        if energy.0 > genome.reproduction_threshold && rng.random_bool(0.01) {
            let offset = Vec2::new(rng.random_range(-30.0..30.0), rng.random_range(-30.0..30.0));
            commands.spawn((
                Plant,
                genome.clone(),
                Energy(energy.0 * 0.5),
                Age(0.0),
                Transform::from_xyz(
                    transform.translation.x + offset.x,
                    transform.translation.y + offset.y,
                    0.0,
                ),
                Sprite {
                    color: Color::srgb(0.2, 0.8, 0.2),
                    custom_size: Some(Vec2::splat(8.0)),
                    ..default()
                },
            ));

            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.insert(Energy(energy.0 * 0.5));
            }
        }
    }

    // Prey reproduction with density-dependent rates
    // Base rate: 0.5%, doubled to 1% when population < 10
    let prey_reproduction_rate = if prey_count < 10 { 0.01 } else { 0.005 };

    for (entity, transform, energy, genome) in prey.iter() {
        if energy.0 > genome.reproduction_threshold && rng.random_bool(prey_reproduction_rate) {
            let offset = Vec2::new(rng.random_range(-20.0..20.0), rng.random_range(-20.0..20.0));
            commands.spawn((
                Prey,
                genome.clone(),
                Energy(energy.0 * 0.5),
                Age(0.0),
                Velocity(Vec2::ZERO),
                Stamina::default(),
                Transform::from_xyz(
                    transform.translation.x + offset.x,
                    transform.translation.y + offset.y,
                    1.0,
                ),
                Sprite {
                    color: Color::srgb(0.3, 0.3, 0.9),
                    custom_size: Some(Vec2::splat(12.0)),
                    ..default()
                },
            ));

            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.insert(Energy(energy.0 * 0.5));
            }
        }
    }

    // Predator reproduction with density-dependent rates
    // Base rate: 0.3%, doubled to 0.6% when population < 10
    let predator_reproduction_rate = if predator_count < 10 { 0.006 } else { 0.003 };

    for (entity, transform, energy, genome) in predators.iter() {
        if energy.0 > genome.reproduction_threshold && rng.random_bool(predator_reproduction_rate) {
            let offset = Vec2::new(rng.random_range(-20.0..20.0), rng.random_range(-20.0..20.0));
            let spawn_pos = Vec2::new(
                transform.translation.x + offset.x,
                transform.translation.y + offset.y,
            );

            // Generate initial exploration waypoint for offspring
            let waypoint_angle = rng.random_range(0.0..std::f32::consts::TAU);
            let waypoint_distance = rng.random_range(100.0..200.0);
            let waypoint_target = Vec2::new(
                spawn_pos.x + waypoint_angle.cos() * waypoint_distance,
                spawn_pos.y + waypoint_angle.sin() * waypoint_distance,
            );

            commands.spawn((
                Predator,
                genome.clone(),
                Energy(energy.0 * 0.5),
                Age(0.0),
                Velocity(Vec2::ZERO),
                HuntTarget(None),
                ExplorationWaypoint {
                    target: waypoint_target,
                    reached_threshold: 30.0,
                },
                Transform::from_xyz(spawn_pos.x, spawn_pos.y, 2.0),
                Sprite {
                    color: Color::srgb(0.9, 0.2, 0.2),
                    custom_size: Some(Vec2::splat(16.0)),
                    ..default()
                },
            ));

            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.insert(Energy(energy.0 * 0.5));
            }
        }
    }

    // Scavenger reproduction with density-dependent rates
    // Base rate: 0.4%, doubled to 0.8% when population < 10
    let scavenger_reproduction_rate = if scavenger_count < 10 { 0.008 } else { 0.004 };

    for (entity, transform, energy, genome) in scavengers.iter() {
        if energy.0 > genome.reproduction_threshold && rng.random_bool(scavenger_reproduction_rate)
        {
            let offset = Vec2::new(rng.random_range(-20.0..20.0), rng.random_range(-20.0..20.0));
            let spawn_pos = Vec2::new(
                transform.translation.x + offset.x,
                transform.translation.y + offset.y,
            );

            // Generate initial exploration waypoint for offspring
            let waypoint_angle = rng.random_range(0.0..std::f32::consts::TAU);
            let waypoint_distance = rng.random_range(100.0..200.0);
            let waypoint_target = Vec2::new(
                spawn_pos.x + waypoint_angle.cos() * waypoint_distance,
                spawn_pos.y + waypoint_angle.sin() * waypoint_distance,
            );

            commands.spawn((
                Scavenger,
                genome.clone(),
                Energy(energy.0 * 0.5),
                Age(0.0),
                Velocity(Vec2::ZERO),
                ExplorationWaypoint {
                    target: waypoint_target,
                    reached_threshold: 30.0,
                },
                Transform::from_xyz(spawn_pos.x, spawn_pos.y, 1.5),
                Sprite {
                    color: Color::srgb(0.7, 0.5, 0.2),
                    custom_size: Some(Vec2::splat(14.0)),
                    ..default()
                },
            ));

            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.insert(Energy(energy.0 * 0.5));
            }
        }
    }
}

pub fn death_system(
    mut commands: Commands,
    organisms: DeathSystemQuery,
    prey_query: Query<&Transform, With<Prey>>,
    predator_query: Query<&Transform, With<Predator>>,
    scavenger_query: Query<&Transform, With<Scavenger>>,
) {
    for (entity, energy, age) in organisms.iter() {
        if energy.0 <= 0.0 || age.0 > 300.0 {
            // Convert to corpse instead of despawning immediately
            // Corpses provide food and decay over time
            let corpse_decay_time = 30.0; // 30 seconds before corpse despawns

            // Change sprite color to indicate death
            if prey_query.get(entity).is_ok() {
                commands
                    .entity(entity)
                    .remove::<Prey>()
                    .remove::<Velocity>()
                    .remove::<Stamina>()
                    .insert(Corpse::new(corpse_decay_time))
                    .insert(Sprite {
                        color: Color::srgb(0.5, 0.5, 0.5), // Gray for corpse
                        custom_size: Some(Vec2::splat(12.0)),
                        ..default()
                    });
            } else if predator_query.get(entity).is_ok() {
                commands
                    .entity(entity)
                    .remove::<Predator>()
                    .remove::<Velocity>()
                    .remove::<HuntTarget>()
                    .remove::<ExplorationWaypoint>()
                    .insert(Corpse::new(corpse_decay_time))
                    .insert(Sprite {
                        color: Color::srgb(0.6, 0.3, 0.3), // Dark red for predator corpse
                        custom_size: Some(Vec2::splat(16.0)),
                        ..default()
                    });
            } else if scavenger_query.get(entity).is_ok() {
                commands
                    .entity(entity)
                    .remove::<Scavenger>()
                    .remove::<Velocity>()
                    .remove::<ExplorationWaypoint>()
                    .insert(Corpse::new(corpse_decay_time))
                    .insert(Sprite {
                        color: Color::srgb(0.5, 0.4, 0.2), // Dark brown for scavenger corpse
                        custom_size: Some(Vec2::splat(14.0)),
                        ..default()
                    });
            }
        }
    }
}

pub fn corpse_decay_system(
    mut commands: Commands,
    mut corpses: Query<(Entity, &mut Corpse, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut corpse, mut sprite) in corpses.iter_mut() {
        corpse.decay_timer -= time.delta_secs();

        // Gradually fade out the corpse as it decays
        let decay_progress = corpse.decay_timer / corpse.max_decay_time;
        let alpha = decay_progress.max(0.2); // Keep minimum alpha of 0.2
        sprite.color = sprite.color.with_alpha(alpha);

        // Despawn when fully decayed
        if corpse.decay_timer <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
