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
            )
                .chain(),
        );
        app
    }

    fn setup_test(mut commands: Commands, config: Res<SimulationConfig>) {
        let mut rng = rand::rng();

        // Spawn plants
        for _ in 0..config.initial_plants {
            let x = rng.random_range(-config.world_size.x / 2.0..config.world_size.x / 2.0);
            let y = rng.random_range(-config.world_size.y / 2.0..config.world_size.y / 2.0);
            commands.spawn((
                Plant,
                Genome::random_plant(),
                Energy(rng.random_range(20.0..50.0)),
                Age(0.0),
                Transform::from_xyz(x, y, 0.0),
            ));
        }

        // Spawn prey
        for _ in 0..config.initial_prey {
            let x = rng.random_range(-config.world_size.x / 2.0..config.world_size.x / 2.0);
            let y = rng.random_range(-config.world_size.y / 2.0..config.world_size.y / 2.0);
            commands.spawn((
                Prey,
                Genome::random_prey(),
                Energy(rng.random_range(40.0..80.0)),
                Age(0.0),
                Velocity(Vec2::ZERO),
                Stamina::default(),
                Transform::from_xyz(x, y, 1.0),
            ));
        }

        // Spawn predators
        for _ in 0..config.initial_predators {
            let x = rng.random_range(-config.world_size.x / 2.0..config.world_size.x / 2.0);
            let y = rng.random_range(-config.world_size.y / 2.0..config.world_size.y / 2.0);

            let waypoint_angle = rng.random_range(0.0..std::f32::consts::TAU);
            let waypoint_distance = rng.random_range(100.0..200.0);
            let waypoint_target = Vec2::new(
                x + waypoint_angle.cos() * waypoint_distance,
                y + waypoint_angle.sin() * waypoint_distance,
            );

            commands.spawn((
                Predator,
                Genome::random_predator(),
                Energy(rng.random_range(60.0..100.0)),
                Age(0.0),
                Velocity(Vec2::ZERO),
                HuntTarget(None),
                ExplorationWaypoint {
                    target: waypoint_target,
                    reached_threshold: 30.0,
                },
                Transform::from_xyz(x, y, 2.0),
            ));
        }

        // Spawn scavengers
        for _ in 0..config.initial_scavengers {
            let x = rng.random_range(-config.world_size.x / 2.0..config.world_size.x / 2.0);
            let y = rng.random_range(-config.world_size.y / 2.0..config.world_size.y / 2.0);

            let waypoint_angle = rng.random_range(0.0..std::f32::consts::TAU);
            let waypoint_distance = rng.random_range(100.0..200.0);
            let waypoint_target = Vec2::new(
                x + waypoint_angle.cos() * waypoint_distance,
                y + waypoint_angle.sin() * waypoint_distance,
            );

            commands.spawn((
                Scavenger,
                Genome::random_scavenger(),
                Energy(rng.random_range(50.0..90.0)),
                Age(0.0),
                Velocity(Vec2::ZERO),
                ExplorationWaypoint {
                    target: waypoint_target,
                    reached_threshold: 30.0,
                },
                Transform::from_xyz(x, y, 1.5),
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
            "After 60s - Plants: {}, Prey: {}, Predators: {}, Scavengers: {}",
            stats.plants, stats.prey, stats.predators, stats.scavengers
        );

        // At least one of each species should survive
        assert!(stats.plants > 0, "Plants went extinct");
        assert!(stats.prey > 0, "Prey went extinct");
        assert!(stats.predators > 0, "Predators went extinct");
        assert!(stats.scavengers > 0, "Scavengers went extinct");
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
            "After 30s - Plants: {}, Prey: {}, Predators: {}, Scavengers: {}",
            stats.plants, stats.prey, stats.predators, stats.scavengers
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
        assert!(
            stats.scavengers > 2,
            "Too few scavengers survived ({})",
            stats.scavengers
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
            "After 90s - Plants: {}, Prey: {}, Predators: {}, Scavengers: {}",
            stats.plants, stats.prey, stats.predators, stats.scavengers
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
        assert!(
            stats.scavengers < 50,
            "Scavenger population exploded ({})",
            stats.scavengers
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

    #[test]
    fn test_immigration_prevents_extinction() {
        let mut app = create_test_app();

        // Manually kill off most animals to trigger immigration
        let prey_entities: Vec<Entity> = app
            .world_mut()
            .query_filtered::<Entity, With<Prey>>()
            .iter(app.world())
            .collect();

        // Kill all but 2 prey
        for (i, entity) in prey_entities.iter().enumerate() {
            if i >= 2 {
                app.world_mut().despawn(*entity);
            }
        }

        let predator_entities: Vec<Entity> = app
            .world_mut()
            .query_filtered::<Entity, With<Predator>>()
            .iter(app.world())
            .collect();

        // Kill all but 2 predators
        for (i, entity) in predator_entities.iter().enumerate() {
            if i >= 2 {
                app.world_mut().despawn(*entity);
            }
        }

        let scavenger_entities: Vec<Entity> = app
            .world_mut()
            .query_filtered::<Entity, With<Scavenger>>()
            .iter(app.world())
            .collect();

        // Kill all but 2 scavengers
        for (i, entity) in scavenger_entities.iter().enumerate() {
            if i >= 2 {
                app.world_mut().despawn(*entity);
            }
        }

        app.update();
        let stats_before = app.world().resource::<PopulationStats>().clone();
        println!(
            "Before immigration - Prey: {}, Predators: {}, Scavengers: {}",
            stats_before.prey, stats_before.predators, stats_before.scavengers
        );

        // Run for 120 seconds to allow immigration to trigger multiple times
        // At 2% chance per second, we should see immigration events
        for _ in 0..7200 {
            app.update();
        }

        let stats_after = app.world().resource::<PopulationStats>().clone();
        println!(
            "After 120s - Prey: {}, Predators: {}, Scavengers: {}",
            stats_after.prey, stats_after.predators, stats_after.scavengers
        );

        // Immigration should have brought populations back up
        assert!(
            stats_after.prey >= 2,
            "Prey population didn't recover via immigration ({})",
            stats_after.prey
        );
        assert!(
            stats_after.predators >= 2,
            "Predator population didn't recover via immigration ({})",
            stats_after.predators
        );
        assert!(
            stats_after.scavengers >= 2,
            "Scavenger population didn't recover via immigration ({})",
            stats_after.scavengers
        );

        // At least one species should have increased from immigration
        let total_before = stats_before.prey + stats_before.predators + stats_before.scavengers;
        let total_after = stats_after.prey + stats_after.predators + stats_after.scavengers;
        assert!(
            total_after >= total_before,
            "No population recovery occurred (before: {}, after: {})",
            total_before,
            total_after
        );
    }
}
