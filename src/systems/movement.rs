use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::resources::*;
use crate::utils::*;

// ===== HELPER FUNCTIONS =====

/// Calculate speed multiplier based on age
/// - Age 0-240: Normal speed (1.0)
/// - Age 240-270: Gradual slowdown (1.0 -> 0.2)
/// - Age 270-300: Very slow (0.2 -> 0.0)
fn age_speed_multiplier(age: f32) -> f32 {
    const SLOWDOWN_START: f32 = 240.0;
    const VERY_SLOW_START: f32 = 270.0;
    const MAX_AGE: f32 = 300.0;

    if age < SLOWDOWN_START {
        1.0
    } else if age < VERY_SLOW_START {
        // Linear interpolation from 1.0 to 0.2
        1.0 - ((age - SLOWDOWN_START) / (VERY_SLOW_START - SLOWDOWN_START)) * 0.8
    } else if age < MAX_AGE {
        // Linear interpolation from 0.2 to 0.0
        0.2 - ((age - VERY_SLOW_START) / (MAX_AGE - VERY_SLOW_START)) * 0.2
    } else {
        0.0
    }
}

// ===== QUERY TYPE ALIASES =====

type PreyMovementQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static mut Transform,
        &'static mut Velocity,
        &'static mut Stamina,
        &'static Genome,
        &'static Energy,
        &'static Age,
    ),
    (With<Prey>, Without<Corpse>),
>;

type PredatorHuntingQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static mut Transform,
        &'static mut Velocity,
        &'static mut HuntTarget,
        &'static Genome,
        &'static Age,
    ),
    (With<Predator>, Without<Corpse>),
>;

type PreyTargetQuery<'w, 's> =
    Query<'w, 's, (Entity, &'static Transform), (With<Prey>, Without<Predator>)>;

// ===== MOVEMENT SYSTEMS =====

pub fn prey_movement_system(
    mut prey: PreyMovementQuery,
    plants: Query<&Transform, (With<Plant>, Without<Prey>)>,
    predators: Query<&Transform, (With<Predator>, Without<Prey>)>,
    config: Res<SimulationConfig>,
    time: Res<Time>,
) {
    let mut rng = rand::rng();

    // Collect all prey data for flocking calculations
    let prey_data: Vec<(Entity, Vec2, Vec2)> = prey
        .iter()
        .map(|(e, t, v, _, _, _, _)| (e, t.translation.xy(), v.0))
        .collect();

    for (entity, mut transform, mut velocity, mut stamina, genome, energy, age) in prey.iter_mut() {
        let mut desired_direction = Vec2::ZERO;
        let mut is_fleeing = false;
        let mut threat_level: f32 = 0.0;

        // Flee from predators (highest priority)
        for predator_transform in predators.iter() {
            let to_predator = predator_transform.translation.xy() - transform.translation.xy();
            let distance = to_predator.length();
            if distance < genome.vision_range * 1.5 {
                let flee_strength = (genome.vision_range * 1.5 - distance) / genome.vision_range;
                desired_direction -= to_predator.normalize() * flee_strength * 2.0;
                is_fleeing = true;
                threat_level = threat_level.max(flee_strength);
            }
        }

        // Flocking behavior (boids algorithm)
        if !is_fleeing || threat_level < 0.7 {
            let flocking_radius = 60.0;
            let mut separation = Vec2::ZERO;
            let mut alignment = Vec2::ZERO;
            let mut cohesion = Vec2::ZERO;
            let mut neighbor_count = 0;

            for (other_entity, other_pos, other_vel) in &prey_data {
                if *other_entity == entity {
                    continue;
                }

                let to_other = *other_pos - transform.translation.xy();
                let distance = to_other.length();

                if distance < flocking_radius && distance > 0.1 {
                    neighbor_count += 1;

                    // Separation: avoid crowding
                    if distance < 30.0 {
                        separation -= to_other.normalize() * (30.0 - distance) / 30.0;
                    }

                    // Alignment: match velocity
                    alignment += *other_vel;

                    // Cohesion: move toward center of group
                    cohesion += to_other;
                }
            }

            if neighbor_count > 0 {
                let neighbor_count_f = neighbor_count as f32;
                separation /= neighbor_count_f;
                alignment = (alignment / neighbor_count_f).normalize_or_zero();
                cohesion = (cohesion / neighbor_count_f).normalize_or_zero();

                // Weight flocking behaviors - less when fleeing
                let flocking_weight = if is_fleeing { 0.2 } else { 0.6 };
                desired_direction += separation * 1.5 * flocking_weight;
                desired_direction += alignment * 0.5 * flocking_weight;
                desired_direction += cohesion * 0.8 * flocking_weight;
            }
        }

        // Determine speed multiplier based on stamina and threat
        let mut speed_multiplier = 1.0;
        let can_sprint = stamina.current > 10.0 && energy.0 > 20.0;

        if is_fleeing && can_sprint && threat_level > 0.5 {
            // Sprint when threatened and have stamina
            speed_multiplier = 2.5;
            stamina.current -= 30.0 * time.delta_secs(); // Drain stamina quickly
        } else {
            // Regenerate stamina when not sprinting
            stamina.current =
                (stamina.current + stamina.regen_rate * time.delta_secs()).min(stamina.max);
        }

        // Move towards nearest plant if hungry and not fleeing strongly
        if desired_direction.length() < 0.5
            && threat_level < 0.3
            && let Some(nearest_plant) = plants
                .iter()
                .min_by_key(|p| (p.translation.xy() - transform.translation.xy()).length() as i32)
        {
            let to_plant = nearest_plant.translation.xy() - transform.translation.xy();
            if to_plant.length() < genome.vision_range {
                desired_direction += to_plant.normalize() * 0.5;
            }
        }

        // Random wander if no strong stimulus
        if desired_direction.length() < 0.1 {
            desired_direction = Vec2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0));
        }

        // Apply age-based speed reduction
        let age_multiplier = age_speed_multiplier(age.0);
        let target_speed = genome.speed * speed_multiplier * age_multiplier;
        velocity.0 = velocity
            .0
            .lerp(desired_direction.normalize() * target_speed, 0.1);
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();

        // Wrap around world
        wrap_position(&mut transform.translation, &config.world_size);
    }
}

pub fn predator_hunting_system(
    mut predators: PredatorHuntingQuery,
    prey: PreyTargetQuery,
    config: Res<SimulationConfig>,
    time: Res<Time>,
) {
    let mut rng = rand::rng();

    // Collect prey positions and valid prey set
    let prey_entities: std::collections::HashSet<Entity> = prey.iter().map(|(e, _)| e).collect();
    let prey_positions: std::collections::HashMap<Entity, Vec2> =
        prey.iter().map(|(e, t)| (e, t.translation.xy())).collect();

    // Collect predator data for separation calculations
    let predator_data: Vec<(Entity, Vec2, Option<Entity>)> = predators
        .iter()
        .map(|(e, t, _, ht, _, _)| (e, t.translation.xy(), ht.0))
        .collect();

    // Count hunters per prey
    let mut hunters_per_prey: std::collections::HashMap<Entity, usize> =
        std::collections::HashMap::new();
    for (_, _, target) in &predator_data {
        if let Some(t) = target {
            *hunters_per_prey.entry(*t).or_insert(0) += 1;
        }
    }

    // Update predators
    for (_predator_entity, mut transform, mut velocity, mut hunt_target, genome, age) in
        predators.iter_mut()
    {
        // Validate target
        if let Some(target) = hunt_target.0
            && !prey_entities.contains(&target)
        {
            hunt_target.0 = None;
        }

        let mut desired_direction = Vec2::ZERO;

        // Check if current target is valid and not overcrowded
        let mut need_new_target = false;
        if let Some(target) = hunt_target.0 {
            if let Some(target_pos) = prey_positions.get(&target) {
                let distance = transform.translation.xy().distance(*target_pos);
                let hunter_count = hunters_per_prey.get(&target).copied().unwrap_or(0);

                // Switch if too many hunters (max 3) or target too far
                if hunter_count > 3 || distance > genome.vision_range * 2.0 {
                    need_new_target = true;
                }
            } else {
                need_new_target = true;
            }
        } else {
            need_new_target = true;
        }

        // Find new target if needed
        if need_new_target {
            hunt_target.0 = prey_positions
                .iter()
                .filter(|(_, pos)| {
                    let distance = transform.translation.xy().distance(**pos);
                    distance < genome.vision_range
                })
                .min_by_key(|(prey_entity, _)| {
                    let hunter_count = hunters_per_prey.get(prey_entity).copied().unwrap_or(0);
                    hunter_count * 1000 // Prioritize less hunted prey
                })
                .map(|(e, _)| *e);
        }

        // Move toward target
        if let Some(target) = hunt_target.0
            && let Some(target_pos) = prey_positions.get(&target)
        {
            let to_prey = *target_pos - transform.translation.xy();
            desired_direction = to_prey.normalize();
        }

        // Add separation from other predators (avoid crowding)
        let separation_radius = 50.0;
        let mut separation_force = Vec2::ZERO;
        for (_, other_pos, _) in &predator_data {
            let to_other = *other_pos - transform.translation.xy();
            let distance = to_other.length();
            if distance > 0.1 && distance < separation_radius {
                separation_force -=
                    to_other.normalize() * (separation_radius - distance) / separation_radius;
            }
        }
        desired_direction += separation_force * 0.3;

        // Random wander if no target
        if desired_direction.length() < 0.1 {
            desired_direction = Vec2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0));
        }

        // Apply age-based speed reduction
        let age_multiplier = age_speed_multiplier(age.0);
        let target_speed = genome.speed * age_multiplier;
        velocity.0 = velocity
            .0
            .lerp(desired_direction.normalize() * target_speed, 0.1);
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();

        // Wrap around world
        wrap_position(&mut transform.translation, &config.world_size);
    }
}
