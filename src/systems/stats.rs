use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

// ===== STATS SYSTEMS =====

pub fn update_population_stats(
    plants: Query<(), With<Plant>>,
    prey: Query<(), With<Prey>>,
    predators: Query<(), With<Predator>>,
    scavengers: Query<(), With<Scavenger>>,
    mut stats: ResMut<PopulationStats>,
) {
    stats.plants = plants.iter().count();
    stats.prey = prey.iter().count();
    stats.predators = predators.iter().count();
    stats.scavengers = scavengers.iter().count();
}

pub fn record_history_system(
    mut history: ResMut<SimulationHistory>,
    stats: Res<PopulationStats>,
    plants: Query<(&Energy, &Age), With<Plant>>,
    prey: Query<(&Energy, &Age, &Genome), With<Prey>>,
    predators: Query<(&Energy, &Age, &Genome), With<Predator>>,
    time: Res<Time>,
) {
    history.time_since_last_record += time.delta_secs();

    if history.time_since_last_record >= history.record_interval {
        history.time_since_last_record = 0.0;

        // Calculate total energy
        let mut total_energy = 0.0;
        for (energy, _) in plants.iter() {
            total_energy += energy.0;
        }
        for (energy, _, _) in prey.iter() {
            total_energy += energy.0;
        }
        for (energy, _, _) in predators.iter() {
            total_energy += energy.0;
        }

        // Calculate average ages
        let avg_plant_age = if stats.plants > 0 {
            plants.iter().map(|(_, age)| age.0).sum::<f32>() / stats.plants as f32
        } else {
            0.0
        };

        let avg_prey_age = if stats.prey > 0 {
            prey.iter().map(|(_, age, _)| age.0).sum::<f32>() / stats.prey as f32
        } else {
            0.0
        };

        let avg_predator_age = if stats.predators > 0 {
            predators.iter().map(|(_, age, _)| age.0).sum::<f32>() / stats.predators as f32
        } else {
            0.0
        };

        // Calculate average speeds
        let avg_prey_speed = if stats.prey > 0 {
            prey.iter().map(|(_, _, genome)| genome.speed).sum::<f32>() / stats.prey as f32
        } else {
            0.0
        };

        let avg_predator_speed = if stats.predators > 0 {
            predators
                .iter()
                .map(|(_, _, genome)| genome.speed)
                .sum::<f32>()
                / stats.predators as f32
        } else {
            0.0
        };

        let elapsed_time = history.snapshots.len() as f32 * history.record_interval;

        history.snapshots.push(SimulationSnapshot {
            _time: elapsed_time,
            plant_count: stats.plants,
            prey_count: stats.prey,
            predator_count: stats.predators,
            total_energy,
            avg_plant_age,
            avg_prey_age,
            avg_predator_age,
            avg_prey_speed,
            avg_predator_speed,
        });
    }
}
