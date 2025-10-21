use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

// ===== QUERY TYPE ALIASES =====

type EatingPreyQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Transform, &'static mut Energy, &'static Genome),
    (
        With<Prey>,
        Without<Plant>,
        Without<Predator>,
        Without<Scavenger>,
    ),
>;

type EatingPredatorQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Transform, &'static mut Energy, &'static Genome),
    (
        With<Predator>,
        Without<Prey>,
        Without<Plant>,
        Without<Scavenger>,
    ),
>;

type EatingPlantQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static Transform, &'static Energy),
    (
        With<Plant>,
        Without<Prey>,
        Without<Predator>,
        Without<Scavenger>,
    ),
>;

type ScavengerEatingQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Transform, &'static mut Energy),
    (
        With<Scavenger>,
        Without<Corpse>,
        Without<Prey>,
        Without<Predator>,
        Without<Plant>,
    ),
>;

type CorpseQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static Transform, &'static Energy),
    (
        With<Corpse>,
        Without<Predator>,
        Without<Prey>,
        Without<Scavenger>,
        Without<Plant>,
    ),
>;

// ===== INTERACTION SYSTEMS =====

#[allow(clippy::too_many_arguments)]
pub fn eating_system(
    mut commands: Commands,
    mut prey: EatingPreyQuery,
    mut predators: EatingPredatorQuery,
    mut scavengers: ScavengerEatingQuery,
    plants: EatingPlantQuery,
    prey_entities: Query<Entity, With<Prey>>,
    corpses: CorpseQuery,
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

    // Predators eating prey (living)
    for (predator_transform, mut predator_energy, _genome) in predators.iter_mut() {
        let mut ate_something = false;

        // First try to eat living prey
        for prey_entity in prey_entities.iter() {
            if let Ok((prey_transform, _, _)) = prey.get(prey_entity) {
                let distance = predator_transform
                    .translation
                    .distance(prey_transform.translation);
                if distance < 20.0 {
                    predator_energy.0 += config.predator_energy_from_prey;
                    commands.entity(prey_entity).despawn();
                    ate_something = true;
                    break;
                }
            }
        }

        // If no living prey found, try to scavenge corpses
        if !ate_something {
            for (corpse_entity, corpse_transform, corpse_energy) in corpses.iter() {
                let distance = predator_transform
                    .translation
                    .distance(corpse_transform.translation);
                if distance < 20.0 && corpse_energy.0 > 10.0 {
                    // Get less energy from corpses than fresh prey
                    predator_energy.0 += config.predator_energy_from_prey * 0.7;
                    commands.entity(corpse_entity).despawn();
                    break;
                }
            }
        }
    }

    // Scavengers eating corpses (corpses query excludes scavenger corpses)
    for (scavenger_transform, mut scavenger_energy) in scavengers.iter_mut() {
        for (corpse_entity, corpse_transform, corpse_energy) in corpses.iter() {
            let to_corpse = crate::utils::wrapped_direction(
                scavenger_transform.translation.xy(),
                corpse_transform.translation.xy(),
                &config.world_size,
            );
            let distance = to_corpse.length();
            if distance < 15.0 && corpse_energy.0 > 10.0 {
                scavenger_energy.0 += config.scavenger_energy_from_corpse;
                commands.entity(corpse_entity).despawn();
                break;
            }
        }
    }
}
