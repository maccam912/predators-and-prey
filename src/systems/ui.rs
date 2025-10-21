use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

// ===== QUERY TYPE ALIASES =====

type VisualPolishPlantQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Sprite, &'static Genome, &'static Energy),
    (With<Plant>, Without<Prey>, Without<Predator>),
>;

type VisualPolishPreyQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Sprite,
        &'static Genome,
        &'static Energy,
        &'static Stamina,
    ),
    (With<Prey>, Without<Plant>, Without<Predator>),
>;

type VisualPolishPredatorQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Sprite, &'static Genome, &'static Energy),
    (With<Predator>, Without<Prey>, Without<Plant>),
>;

// ===== UI SYSTEMS =====

pub fn console_output_system(
    mut console: ResMut<ConsoleOutput>,
    stats: Res<PopulationStats>,
    history: Res<SimulationHistory>,
    sunlight: Res<SunlightLevel>,
    time: Res<Time>,
) {
    console.time_since_last_print += time.delta_secs();

    if console.time_since_last_print >= console.print_interval {
        console.time_since_last_print = 0.0;

        let elapsed = history.snapshots.len() as f32 * history.record_interval;

        println!("\n========== Simulation Stats ({elapsed:.1}s) ==========");
        println!(
            "Population: Plants={}, Prey={}, Predators={}, Scavengers={}",
            stats.plants, stats.prey, stats.predators, stats.scavengers
        );

        if let Some(latest) = history.snapshots.last() {
            println!("Total Energy: {:.1}", latest.total_energy);
            println!(
                "Avg Ages: Plants={:.1}s, Prey={:.1}s, Predators={:.1}s",
                latest.avg_plant_age, latest.avg_prey_age, latest.avg_predator_age
            );
            println!(
                "Avg Speeds: Prey={:.1}, Predators={:.1}",
                latest.avg_prey_speed, latest.avg_predator_speed
            );
        }

        println!("Sunlight: {:.0}%", sunlight.intensity * 100.0);
        println!("==============================================\n");
    }
}

pub fn ui_system(
    stats: Res<PopulationStats>,
    history: Res<SimulationHistory>,
    sunlight: Res<SunlightLevel>,
    mut text: Query<&mut Text>,
) {
    for mut text in text.iter_mut() {
        let mut display = format!(
            "=== ECOSYSTEM SIMULATION ===\n\n\
             POPULATION\n\
             Plants:     {}\n\
             Prey:       {}\n\
             Predators:  {}\n\
             Scavengers: {}\n\n",
            stats.plants, stats.prey, stats.predators, stats.scavengers
        );

        if let Some(latest) = history.snapshots.last() {
            display.push_str(&format!(
                "ENERGY & AGE\n\
                 Total Energy: {:.0}\n\
                 Avg Plant Age: {:.1}s\n\
                 Avg Prey Age: {:.1}s\n\
                 Avg Pred Age: {:.1}s\n\n\
                 TRAITS\n\
                 Prey Speed: {:.1}\n\
                 Pred Speed: {:.1}\n\n",
                latest.total_energy,
                latest.avg_plant_age,
                latest.avg_prey_age,
                latest.avg_predator_age,
                latest.avg_prey_speed,
                latest.avg_predator_speed
            ));
        }

        display.push_str(&format!(
            "ENVIRONMENT\n\
             Sunlight: {:.0}%\n\n\
             Time: {:.0}s",
            sunlight.intensity * 100.0,
            history.snapshots.len() as f32 * history.record_interval
        ));

        **text = display;
    }
}

pub fn visual_polish_system(
    mut plants: VisualPolishPlantQuery,
    mut prey: VisualPolishPreyQuery,
    mut predators: VisualPolishPredatorQuery,
) {
    // Update plant visuals
    for (mut sprite, genome, energy) in plants.iter_mut() {
        let size = 8.0 * genome.size;
        sprite.custom_size = Some(Vec2::splat(size));

        // Darken color based on energy
        let energy_factor = (energy.0 / 150.0).clamp(0.3, 1.0);
        sprite.color = Color::srgb(
            0.2 * energy_factor,
            0.8 * energy_factor,
            0.2 * energy_factor,
        );
    }

    // Update prey visuals
    for (mut sprite, genome, energy, stamina) in prey.iter_mut() {
        let size = 12.0 * genome.size;
        sprite.custom_size = Some(Vec2::splat(size));

        // Color based on stamina and energy
        let stamina_factor = (stamina.current / stamina.max).clamp(0.3, 1.0);
        let energy_factor = (energy.0 / 100.0).clamp(0.3, 1.0);
        sprite.color = Color::srgb(
            0.3 * energy_factor,
            0.3 * energy_factor,
            0.9 * stamina_factor,
        );
    }

    // Update predator visuals
    for (mut sprite, genome, energy) in predators.iter_mut() {
        let size = 16.0 * genome.size;
        sprite.custom_size = Some(Vec2::splat(size));

        // Color based on energy
        let energy_factor = (energy.0 / 140.0).clamp(0.3, 1.0);
        sprite.color = Color::srgb(
            0.9 * energy_factor,
            0.2 * energy_factor,
            0.2 * energy_factor,
        );
    }
}

pub fn draw_graphs_system(mut gizmos: Gizmos, history: Res<SimulationHistory>) {
    if history.snapshots.len() < 2 {
        return;
    }

    // Graph parameters
    let graph_width = 400.0;
    let graph_height = 150.0;
    let graph_x = -550.0; // Left side
    let graph_y = 300.0; // Top

    // Draw background
    let top_left = Vec2::new(graph_x, graph_y);
    let top_right = Vec2::new(graph_x + graph_width, graph_y);
    let bottom_left = Vec2::new(graph_x, graph_y - graph_height);
    let bottom_right = Vec2::new(graph_x + graph_width, graph_y - graph_height);

    gizmos.line_2d(top_left, top_right, Color::srgba(0.3, 0.3, 0.3, 0.8));
    gizmos.line_2d(top_left, bottom_left, Color::srgba(0.3, 0.3, 0.3, 0.8));
    gizmos.line_2d(bottom_left, bottom_right, Color::srgba(0.3, 0.3, 0.3, 0.8));
    gizmos.line_2d(top_right, bottom_right, Color::srgba(0.3, 0.3, 0.3, 0.8));

    // Find max values for scaling
    let max_plants = history
        .snapshots
        .iter()
        .map(|s| s.plant_count)
        .max()
        .unwrap_or(1)
        .max(10);
    let max_prey = history
        .snapshots
        .iter()
        .map(|s| s.prey_count)
        .max()
        .unwrap_or(1)
        .max(10);
    let max_predators = history
        .snapshots
        .iter()
        .map(|s| s.predator_count)
        .max()
        .unwrap_or(1)
        .max(10);
    let max_scavengers = history
        .snapshots
        .iter()
        .map(|s| s.scavenger_count)
        .max()
        .unwrap_or(1)
        .max(10);
    let max_pop = max_plants
        .max(max_prey)
        .max(max_predators)
        .max(max_scavengers) as f32;

    // Draw data points (last 100 snapshots)
    let start_idx = history.snapshots.len().saturating_sub(100);
    let visible_snapshots = &history.snapshots[start_idx..];

    for i in 0..visible_snapshots.len().saturating_sub(1) {
        let x1 = graph_x + (i as f32 / visible_snapshots.len() as f32) * graph_width;
        let x2 = graph_x + ((i + 1) as f32 / visible_snapshots.len() as f32) * graph_width;

        // Plants (green)
        let y1_plants = graph_y - graph_height
            + (visible_snapshots[i].plant_count as f32 / max_pop) * graph_height;
        let y2_plants = graph_y - graph_height
            + (visible_snapshots[i + 1].plant_count as f32 / max_pop) * graph_height;
        gizmos.line_2d(
            Vec2::new(x1, y1_plants),
            Vec2::new(x2, y2_plants),
            Color::srgb(0.2, 0.9, 0.2),
        );

        // Prey (blue)
        let y1_prey = graph_y - graph_height
            + (visible_snapshots[i].prey_count as f32 / max_pop) * graph_height;
        let y2_prey = graph_y - graph_height
            + (visible_snapshots[i + 1].prey_count as f32 / max_pop) * graph_height;
        gizmos.line_2d(
            Vec2::new(x1, y1_prey),
            Vec2::new(x2, y2_prey),
            Color::srgb(0.3, 0.3, 0.9),
        );

        // Predators (red)
        let y1_pred = graph_y - graph_height
            + (visible_snapshots[i].predator_count as f32 / max_pop) * graph_height;
        let y2_pred = graph_y - graph_height
            + (visible_snapshots[i + 1].predator_count as f32 / max_pop) * graph_height;
        gizmos.line_2d(
            Vec2::new(x1, y1_pred),
            Vec2::new(x2, y2_pred),
            Color::srgb(0.9, 0.2, 0.2),
        );

        // Scavengers (brown)
        let y1_scav = graph_y - graph_height
            + (visible_snapshots[i].scavenger_count as f32 / max_pop) * graph_height;
        let y2_scav = graph_y - graph_height
            + (visible_snapshots[i + 1].scavenger_count as f32 / max_pop) * graph_height;
        gizmos.line_2d(
            Vec2::new(x1, y1_scav),
            Vec2::new(x2, y2_scav),
            Color::srgb(0.7, 0.5, 0.2),
        );
    }

    // Energy graph below
    let energy_y = graph_y - graph_height - 50.0;
    let energy_height = 100.0;

    // Draw energy graph background
    let e_top_left = Vec2::new(graph_x, energy_y);
    let e_top_right = Vec2::new(graph_x + graph_width, energy_y);
    let e_bottom_left = Vec2::new(graph_x, energy_y - energy_height);
    let e_bottom_right = Vec2::new(graph_x + graph_width, energy_y - energy_height);

    gizmos.line_2d(e_top_left, e_top_right, Color::srgba(0.3, 0.3, 0.3, 0.8));
    gizmos.line_2d(e_top_left, e_bottom_left, Color::srgba(0.3, 0.3, 0.3, 0.8));
    gizmos.line_2d(
        e_bottom_left,
        e_bottom_right,
        Color::srgba(0.3, 0.3, 0.3, 0.8),
    );
    gizmos.line_2d(
        e_top_right,
        e_bottom_right,
        Color::srgba(0.3, 0.3, 0.3, 0.8),
    );

    let max_energy = history
        .snapshots
        .iter()
        .map(|s| s.total_energy as i32)
        .max()
        .unwrap_or(1000)
        .max(1000) as f32;

    // Draw energy line
    for i in 0..visible_snapshots.len().saturating_sub(1) {
        let x1 = graph_x + (i as f32 / visible_snapshots.len() as f32) * graph_width;
        let x2 = graph_x + ((i + 1) as f32 / visible_snapshots.len() as f32) * graph_width;

        let y1 = energy_y - energy_height
            + (visible_snapshots[i].total_energy / max_energy) * energy_height;
        let y2 = energy_y - energy_height
            + (visible_snapshots[i + 1].total_energy / max_energy) * energy_height;

        gizmos.line_2d(
            Vec2::new(x1, y1),
            Vec2::new(x2, y2),
            Color::srgb(0.9, 0.9, 0.2),
        );
    }
}
