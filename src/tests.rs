#[cfg(test)]
mod tests {
    use bevy::app::ScheduleRunnerPlugin;
    use bevy::prelude::*;
    use rand::Rng;
    use std::time::Duration;

    use crate::components::*;
    use crate::resources::*;
    use crate::systems::*;

    fn create_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / 60.0),
        )))
        .init_resource::<SimulationConfig>()
        .init_resource::<PopulationStats>()
        .init_resource::<SunlightLevel>()
        .insert_resource(SimulationHistory {
            snapshots: Vec::new(),
            record_interval: 1.0,
            time_since_last_record: 0.0,
        })
        .add_systems(Startup, setup_test)
        .add_systems(
            Update,
            (
                sunlight_cycle_system,
                plant_growth_system,
                prey_movement_system,
                predator_hunting_system,
                eating_system,
                energy_consumption_system,
                age_system,
                reproduction_system,
                death_system,
                update_population_stats,
                record_history_system,
            )
                .chain(),
        );
        app
    }

    fn setup_test(mut commands: Commands, config: Res<SimulationConfig>) {
        let mut rng = rand::rng();

        // Spawn plants
        for _ in 0..config.initial_plants {
            let x = rng.gen_range(-config.world_size.x / 2.0..config.world_size.x / 2.0);
            let y = rng.gen_range(-config.world_size.y / 2.0..config.world_size.y / 2.0);
            commands.spawn((
                Plant,
                Genome::random_plant(),
                Energy(rng.gen_range(20.0..50.0)),
                Age(0.0),
                Transform::from_xyz(x, y, 0.0),
            ));
        }

        // Spawn prey
        for _ in 0..config.initial_prey {
            let x = rng.gen_range(-config.world_size.x / 2.0..config.world_size.x / 2.0);
            let y = rng.gen_range(-config.world_size.y / 2.0..config.world_size.y / 2.0);
            commands.spawn((
                Prey,
                Genome::random_prey(),
                Energy(rng.gen_range(40.0..80.0)),
                Age(0.0),
                Velocity(Vec2::ZERO),
                Stamina::default(),
                Transform::from_xyz(x, y, 1.0),
            ));
        }

        // Spawn predators
        for _ in 0..config.initial_predators {
            let x = rng.gen_range(-config.world_size.x / 2.0..config.world_size.x / 2.0);
            let y = rng.gen_range(-config.world_size.y / 2.0..config.world_size.y / 2.0);
            commands.spawn((
                Predator,
                Genome::random_predator(),
                Energy(rng.gen_range(60.0..100.0)),
                Age(0.0),
                Velocity(Vec2::ZERO),
                HuntTarget(None),
                Transform::from_xyz(x, y, 2.0),
            ));
        }
    }

    #[test]
    fn test_simulation_survives_60_seconds() {
        let mut app = create_test_app();

        // Run for 60 simulated seconds (3600 frames at 60 FPS)
        for _ in 0..3600 {
            app.update();
        }

        let stats = app.world().resource::<PopulationStats>();
        println!(
            "After 60s - Plants: {}, Prey: {}, Predators: {}",
            stats.plants, stats.prey, stats.predators
        );

        // At least one of each species should survive
        assert!(stats.plants > 0, "Plants went extinct");
        assert!(stats.prey > 0, "Prey went extinct");
        assert!(stats.predators > 0, "Predators went extinct");
    }

    #[test]
    fn test_no_early_extinction() {
        let mut app = create_test_app();

        // Run for 30 simulated seconds
        for _ in 0..1800 {
            app.update();
        }

        let stats = app.world().resource::<PopulationStats>();
        println!(
            "After 30s - Plants: {}, Prey: {}, Predators: {}",
            stats.plants, stats.prey, stats.predators
        );

        // All species should still be alive after 30 seconds
        assert!(
            stats.plants > 10,
            "Too few plants survived ({})",
            stats.plants
        );
        assert!(stats.prey > 5, "Too few prey survived ({})", stats.prey);
        assert!(
            stats.predators > 2,
            "Too few predators survived ({})",
            stats.predators
        );
    }

    #[test]
    fn test_no_population_explosion() {
        let mut app = create_test_app();

        // Run for 90 simulated seconds
        for _ in 0..5400 {
            app.update();
        }

        let stats = app.world().resource::<PopulationStats>();
        println!(
            "After 90s - Plants: {}, Prey: {}, Predators: {}",
            stats.plants, stats.prey, stats.predators
        );

        // Populations shouldn't explode
        assert!(
            stats.plants < 500,
            "Plant population exploded ({})",
            stats.plants
        );
        assert!(
            stats.prey < 200,
            "Prey population exploded ({})",
            stats.prey
        );
        assert!(
            stats.predators < 50,
            "Predator population exploded ({})",
            stats.predators
        );
    }

    #[test]
    fn test_energy_balance() {
        let mut app = create_test_app();

        // Run for 30 seconds
        for _ in 0..1800 {
            app.update();
        }

        // Calculate total energy in the system
        let mut total_energy = 0.0;
        let mut query = app.world_mut().query::<&Energy>();
        for energy in query.iter(app.world()) {
            total_energy += energy.0;
        }

        println!("Total energy after 30s: {:.2}", total_energy);

        // System should have reasonable energy (not zero, not infinite)
        assert!(
            total_energy > 500.0,
            "System energy too low ({:.2})",
            total_energy
        );
        assert!(
            total_energy < 50000.0,
            "System energy too high ({:.2})",
            total_energy
        );
    }
}
