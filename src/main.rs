use bevy::prelude::*;

mod components;
mod resources;
mod systems;
mod utils;

#[cfg(test)]
mod tests;

use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Predators and Prey - Ecology Simulator".into(),
                resolution: (1280, 720).into(),
                canvas: Some("#game-container".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<SimulationConfig>()
        .init_resource::<PopulationStats>()
        .init_resource::<SunlightLevel>()
        .init_resource::<ConsoleOutput>()
        .insert_resource(SimulationHistory {
            snapshots: Vec::new(),
            record_interval: 1.0,
            time_since_last_record: 0.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, camera_controls_system)
        .add_systems(
            Update,
            (
                sunlight_cycle_system,
                plant_growth_system,
                plant_respawn_system,
                immigration_system,
                prey_movement_system,
                predator_hunting_system,
                scavenger_movement_system,
                eating_system,
                energy_consumption_system,
                age_system,
                reproduction_system,
                death_system,
                corpse_decay_system,
                update_population_stats,
                record_history_system,
                console_output_system,
                visual_polish_system,
                ui_system,
                draw_graphs_system,
            )
                .chain(),
        )
        .run();
}
