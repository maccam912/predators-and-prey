use bevy::prelude::*;

// ===== UTILITY FUNCTIONS =====

pub fn wrap_position(position: &mut Vec3, world_size: &Vec2) {
    if position.x > world_size.x / 2.0 {
        position.x = -world_size.x / 2.0;
    } else if position.x < -world_size.x / 2.0 {
        position.x = world_size.x / 2.0;
    }

    if position.y > world_size.y / 2.0 {
        position.y = -world_size.y / 2.0;
    } else if position.y < -world_size.y / 2.0 {
        position.y = world_size.y / 2.0;
    }
}

/// Calculate the shortest vector from `from` to `to` on a torus world
/// This accounts for wrapping at world boundaries
pub fn wrapped_direction(from: Vec2, to: Vec2, world_size: &Vec2) -> Vec2 {
    let mut delta = to - from;

    // Check if wrapping around horizontally is shorter
    if delta.x > world_size.x / 2.0 {
        delta.x -= world_size.x;
    } else if delta.x < -world_size.x / 2.0 {
        delta.x += world_size.x;
    }

    // Check if wrapping around vertically is shorter
    if delta.y > world_size.y / 2.0 {
        delta.y -= world_size.y;
    } else if delta.y < -world_size.y / 2.0 {
        delta.y += world_size.y;
    }

    delta
}

/// Calculate the shortest distance between two points on a torus world
pub fn wrapped_distance(from: Vec2, to: Vec2, world_size: &Vec2) -> f32 {
    wrapped_direction(from, to, world_size).length()
}
