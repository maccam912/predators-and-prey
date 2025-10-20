use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

// ===== ENVIRONMENT SYSTEMS =====

pub fn sunlight_cycle_system(mut sunlight: ResMut<SunlightLevel>, time: Res<Time>) {
    sunlight.cycle_time += time.delta_secs();
    sunlight.intensity = (sunlight.cycle_time * 0.5).sin() * 0.3 + 0.7;
}

pub fn plant_growth_system(
    mut plants: Query<(&mut Energy, &Genome), With<Plant>>,
    sunlight: Res<SunlightLevel>,
    config: Res<SimulationConfig>,
    time: Res<Time>,
) {
    for (mut energy, genome) in plants.iter_mut() {
        let growth =
            config.plant_energy_from_sun * sunlight.intensity * genome.size * time.delta_secs();
        energy.0 = (energy.0 + growth).min(150.0);
    }
}
