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
