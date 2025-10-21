use bevy::prelude::*;
use rand::Rng;

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

pub fn plant_respawn_system(
    mut commands: Commands,
    plants: Query<&Plant>,
    sunlight: Res<SunlightLevel>,
    config: Res<SimulationConfig>,
    time: Res<Time>,
) {
    let current_plant_count = plants.iter().count();

    // Only spawn new plants if below the maximum
    if current_plant_count < config.max_plants {
        // Probability of spawning a new plant is based on sunlight intensity
        // Higher sunlight = more plant spawns
        let spawn_chance = config.plant_respawn_rate * sunlight.intensity * time.delta_secs();

        let mut rng = rand::rng();
        if rng.gen_bool(spawn_chance as f64) {
            // Spawn a new plant at a random location
            let x = rng.gen_range(-config.world_size.x / 2.0..config.world_size.x / 2.0);
            let y = rng.gen_range(-config.world_size.y / 2.0..config.world_size.y / 2.0);

            commands.spawn((
                Plant,
                Genome::random_plant(),
                Energy(rng.gen_range(20.0..40.0)),
                Age(0.0),
                Transform::from_xyz(x, y, 0.0),
                Sprite {
                    color: Color::srgb(0.2, 0.8, 0.2),
                    custom_size: Some(Vec2::splat(8.0)),
                    ..default()
                },
            ));
        }
    }
}
