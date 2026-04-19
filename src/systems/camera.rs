use macroquad::prelude::*;
use crate::components::{Transform, CameraComponent};
use crate::input::InputState;

pub struct CameraSystem;

impl CameraSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn update(
        &self,
        transform: &mut Transform,
        camera: &mut CameraComponent,
        input: &InputState,
    ) {
        if !input.cursor_captured {
            return;
        }

        let yaw_delta = input.mouse_delta.x * camera.sensitivity;
        if camera.invert_x {
            camera.yaw += yaw_delta;
        } else {
            camera.yaw -= yaw_delta;
        }
        
        let pitch_delta = input.mouse_delta.y * camera.sensitivity;
        if camera.invert_y {
            camera.pitch += pitch_delta;
        } else {
            camera.pitch -= pitch_delta;
        }
        
        camera.clamp_pitch();

        transform.rotation = Quat::from_rotation_y(camera.yaw) * 
                            Quat::from_rotation_x(camera.pitch);
    }
}