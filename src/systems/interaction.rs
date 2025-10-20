use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

// ===== QUERY TYPE ALIASES =====

type EatingPreyQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Transform, &'static mut Energy, &'static Genome),
    (With<Prey>, Without<Plant>, Without<Predator>),
>;

type EatingPredatorQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Transform, &'static mut Energy, &'static Genome),
    (With<Predator>, Without<Prey>, Without<Plant>),
>;

type EatingPlantQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static Transform, &'static Energy),
    (With<Plant>, Without<Prey>, Without<Predator>),
>;

// ===== INTERACTION SYSTEMS =====

pub fn eating_system(
    mut commands: Commands,
    mut prey: EatingPreyQuery,
    mut predators: EatingPredatorQuery,
    plants: EatingPlantQuery,
    prey_entities: Query<Entity, With<Prey>>,
    config: Res<SimulationConfig>,
) {
    // Prey eating plants
    for (prey_transform, mut prey_energy, _genome) in prey.iter_mut() {
        for (plant_entity, plant_transform, plant_energy) in plants.iter() {
            let distance = prey_transform
                .translation
                .distance(plant_transform.translation);
            if distance < 15.0 && plant_energy.0 > 20.0 {
                prey_energy.0 += config.prey_energy_from_plant;
                commands.entity(plant_entity).despawn();
                break;
            }
        }
    }

    // Predators eating prey
    for (predator_transform, mut predator_energy, _genome) in predators.iter_mut() {
        for prey_entity in prey_entities.iter() {
            if let Ok((prey_transform, _, _)) = prey.get(prey_entity) {
                let distance = predator_transform
                    .translation
                    .distance(prey_transform.translation);
                if distance < 20.0 {
                    predator_energy.0 += config.predator_energy_from_prey;
                    commands.entity(prey_entity).despawn();
                    break;
                }
            }
        }
    }
}
