use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::input::touch::{TouchInput, TouchPhase};
use bevy::prelude::*;

// ===== CAMERA CONTROLS =====

/// Component to track camera state for pan and zoom
#[derive(Component)]
pub struct CameraController {
    pub zoom_speed: f32,
    pub pan_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    /// Tracks if we're currently panning with mouse
    pub is_panning: bool,
    /// Tracks initial touch positions for pinch-to-zoom
    pub touch_state: TouchState,
}

#[derive(Default)]
pub struct TouchState {
    /// Positions of active touches
    pub positions: Vec<(u64, Vec2)>,
    /// Initial distance between two touches (for pinch zoom)
    pub initial_distance: Option<f32>,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            zoom_speed: 0.1,
            pan_speed: 1.0,
            min_zoom: 0.1,
            max_zoom: 5.0,
            is_panning: false,
            touch_state: TouchState::default(),
        }
    }
}

/// System to handle camera zoom and pan controls
pub fn camera_controls_system(
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut camera_controller_query: Query<&mut CameraController>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut touch_events: EventReader<TouchInput>,
    windows: Query<&Window>,
) {
    let Ok((mut camera_transform, mut projection)) = camera_query.get_single_mut() else {
        return;
    };

    let Ok(mut controller) = camera_controller_query.get_single_mut() else {
        return;
    };

    let current_scale = projection.scale;

    // === DESKTOP: Mouse Wheel Zoom ===
    for event in mouse_wheel_events.read() {
        let zoom_delta = match event.unit {
            MouseScrollUnit::Line => event.y * 0.1,
            MouseScrollUnit::Pixel => event.y * 0.01,
        };

        let new_scale = (current_scale - zoom_delta * controller.zoom_speed)
            .clamp(controller.min_zoom, controller.max_zoom);
        projection.scale = new_scale;
    }

    // === DESKTOP: Mouse Drag Pan ===
    // Start panning on right mouse button or middle mouse button
    if mouse_button_input.just_pressed(MouseButton::Right)
        || mouse_button_input.just_pressed(MouseButton::Middle)
    {
        controller.is_panning = true;
    }

    if mouse_button_input.just_released(MouseButton::Right)
        || mouse_button_input.just_released(MouseButton::Middle)
    {
        controller.is_panning = false;
    }

    // Pan camera while dragging
    if controller.is_panning {
        for event in mouse_motion_events.read() {
            // Pan relative to current zoom level
            camera_transform.translation.x -= event.delta.x * controller.pan_speed * current_scale;
            camera_transform.translation.y += event.delta.y * controller.pan_speed * current_scale;
        }
    } else {
        // Clear events if not panning
        mouse_motion_events.clear();
    }

    // === MOBILE: Touch Controls ===
    for event in touch_events.read() {
        match event.phase {
            TouchPhase::Started => {
                // Add new touch
                controller
                    .touch_state
                    .positions
                    .push((event.id, event.position));
            }
            TouchPhase::Moved => {
                // Update touch position
                if let Some(touch) = controller
                    .touch_state
                    .positions
                    .iter_mut()
                    .find(|(id, _)| *id == event.id)
                {
                    let old_pos = touch.1;
                    touch.1 = event.position;

                    if controller.touch_state.positions.len() == 1 {
                        // Single touch: Pan
                        let window = windows.single();
                        let window_size = Vec2::new(window.width(), window.height());

                        // Calculate delta in world space
                        let delta = event.position - old_pos;
                        let world_delta = Vec2::new(
                            -delta.x / window_size.x * window_size.x * current_scale,
                            delta.y / window_size.y * window_size.y * current_scale,
                        );

                        camera_transform.translation.x += world_delta.x;
                        camera_transform.translation.y += world_delta.y;
                    } else if controller.touch_state.positions.len() == 2 {
                        // Two touches: Pinch to zoom
                        let touch1 = controller.touch_state.positions[0].1;
                        let touch2 = controller.touch_state.positions[1].1;
                        let current_distance = touch1.distance(touch2);

                        if let Some(initial_distance) = controller.touch_state.initial_distance {
                            let zoom_factor = initial_distance / current_distance;
                            let new_scale = (current_scale * zoom_factor)
                                .clamp(controller.min_zoom, controller.max_zoom);
                            projection.scale = new_scale;
                        }

                        controller.touch_state.initial_distance = Some(current_distance);
                    }
                }
            }
            TouchPhase::Ended | TouchPhase::Canceled => {
                // Remove touch
                controller
                    .touch_state
                    .positions
                    .retain(|(id, _)| *id != event.id);

                // Reset pinch state when touches end
                if controller.touch_state.positions.len() < 2 {
                    controller.touch_state.initial_distance = None;
                }
            }
        }
    }
}
