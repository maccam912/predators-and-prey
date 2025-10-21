use bevy::prelude::*;
use rand::Rng;

use crate::components::*;

// ===== QUERY TYPE ALIASES =====

type EnergyConsumptionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Energy, &'static Genome, &'static Velocity),
    Or<(With<Plant>, With<Prey>, With<Predator>)>,
>;

type DeathSystemQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static Energy, &'static Age),
    Or<(With<Plant>, With<Prey>, With<Predator>)>,
>;

type AgeSystemQuery<'w, 's> =
    Query<'w, 's, &'static mut Age, Or<(With<Plant>, With<Prey>, With<Predator>)>>;

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
) {
    let mut rng = rand::rng();

    // Count populations for density-dependent reproduction
    let prey_count = prey.iter().count();
    let predator_count = predators.iter().count();

    // Plant reproduction
    for (entity, transform, energy, genome) in plants.iter() {
        if energy.0 > genome.reproduction_threshold && rng.gen_bool(0.01) {
            let offset = Vec2::new(rng.gen_range(-30.0..30.0), rng.gen_range(-30.0..30.0));
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
        if energy.0 > genome.reproduction_threshold && rng.gen_bool(prey_reproduction_rate) {
            let offset = Vec2::new(rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0));
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
        if energy.0 > genome.reproduction_threshold && rng.gen_bool(predator_reproduction_rate) {
            let offset = Vec2::new(rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0));
            commands.spawn((
                Predator,
                genome.clone(),
                Energy(energy.0 * 0.5),
                Age(0.0),
                Velocity(Vec2::ZERO),
                HuntTarget(None),
                Transform::from_xyz(
                    transform.translation.x + offset.x,
                    transform.translation.y + offset.y,
                    2.0,
                ),
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
}

pub fn death_system(mut commands: Commands, organisms: DeathSystemQuery) {
    for (entity, energy, age) in organisms.iter() {
        if energy.0 <= 0.0 || age.0 > 300.0 {
            commands.entity(entity).despawn();
        }
    }
}
