use bevy::prelude::*;
use rand::Rng;

// ===== COMPONENTS =====

#[derive(Component, Clone)]
pub struct Genome {
    pub speed: f32,
    pub size: f32,
    pub metabolism: f32,
    pub reproduction_threshold: f32,
    pub vision_range: f32,
}

impl Genome {
    pub fn random_plant() -> Self {
        let mut rng = rand::rng();
        Self {
            speed: 0.0,
            size: rng.gen_range(0.5..1.5),
            metabolism: rng.gen_range(0.3..0.7),
            reproduction_threshold: rng.gen_range(80.0..120.0),
            vision_range: 0.0,
        }
    }

    pub fn random_prey() -> Self {
        let mut rng = rand::rng();
        Self {
            speed: rng.gen_range(50.0..150.0),
            size: rng.gen_range(1.0..2.0),
            metabolism: rng.gen_range(0.5..1.5),
            reproduction_threshold: rng.gen_range(60.0..100.0),
            vision_range: rng.gen_range(80.0..120.0),
        }
    }

    pub fn random_predator() -> Self {
        let mut rng = rand::rng();
        Self {
            speed: rng.gen_range(80.0..180.0),
            size: rng.gen_range(1.5..3.0),
            metabolism: rng.gen_range(1.0..2.0),
            reproduction_threshold: rng.gen_range(80.0..140.0),
            vision_range: rng.gen_range(100.0..180.0),
        }
    }
}

#[derive(Component)]
pub struct Plant;

#[derive(Component)]
pub struct Prey;

#[derive(Component)]
pub struct Predator;

#[derive(Component)]
pub struct Energy(pub f32);

#[derive(Component)]
pub struct Age(pub f32);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct HuntTarget(pub Option<Entity>);

#[derive(Component)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,
}

impl Default for Stamina {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
            regen_rate: 10.0, // per second
        }
    }
}
