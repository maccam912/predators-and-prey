use bevy::prelude::*;

// ===== RESOURCES =====

#[derive(Resource)]
pub struct SimulationConfig {
    pub world_size: Vec2,
    pub initial_plants: usize,
    pub initial_prey: usize,
    pub initial_predators: usize,
    pub initial_scavengers: usize,
    pub plant_energy_from_sun: f32,
    pub prey_energy_from_plant: f32,
    pub predator_energy_from_prey: f32,
    pub scavenger_energy_from_corpse: f32,
    pub plant_respawn_rate: f32,
    pub max_plants: usize,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            world_size: Vec2::new(4800.0, 3200.0),
            initial_plants: 600,
            initial_prey: 160,
            initial_predators: 32,
            initial_scavengers: 48,
            plant_energy_from_sun: 0.5,
            prey_energy_from_plant: 30.0,
            predator_energy_from_prey: 50.0,
            scavenger_energy_from_corpse: 35.0,
            plant_respawn_rate: 2.0,
            max_plants: 1200,
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct PopulationStats {
    pub plants: usize,
    pub prey: usize,
    pub predators: usize,
    pub scavengers: usize,
}

#[derive(Resource)]
pub struct SunlightLevel {
    pub intensity: f32,
    pub cycle_time: f32,
}

impl Default for SunlightLevel {
    fn default() -> Self {
        Self {
            intensity: 1.0,
            cycle_time: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct SimulationSnapshot {
    pub _time: f32,
    pub plant_count: usize,
    pub prey_count: usize,
    pub predator_count: usize,
    pub scavenger_count: usize,
    pub total_energy: f32,
    pub avg_plant_age: f32,
    pub avg_prey_age: f32,
    pub avg_predator_age: f32,
    pub avg_prey_speed: f32,
    pub avg_predator_speed: f32,
}

impl Default for SimulationSnapshot {
    fn default() -> Self {
        Self {
            _time: 0.0,
            plant_count: 0,
            prey_count: 0,
            predator_count: 0,
            scavenger_count: 0,
            total_energy: 0.0,
            avg_plant_age: 0.0,
            avg_prey_age: 0.0,
            avg_predator_age: 0.0,
            avg_prey_speed: 0.0,
            avg_predator_speed: 0.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct SimulationHistory {
    pub snapshots: Vec<SimulationSnapshot>,
    pub record_interval: f32,
    pub time_since_last_record: f32,
}

#[derive(Resource)]
pub struct ConsoleOutput {
    pub print_interval: f32,
    pub time_since_last_print: f32,
}

impl Default for ConsoleOutput {
    fn default() -> Self {
        Self {
            print_interval: 10.0,
            time_since_last_print: 0.0,
        }
    }
}
