use macroquad::prelude::*;
use crate::common::Transform;
use crate::input::InputState;

#[derive(Debug, Clone)]
pub struct CameraComponent {
    pub fov: f32,
    pub sensitivity: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub invert_x: bool,
    pub invert_y: bool,
}

impl Default for CameraComponent {
    fn default() -> Self {
        Self {
            fov: 90.0_f32.to_radians(),
            sensitivity: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            invert_x: true,
            invert_y: false,
        }
    }
}

impl CameraComponent {
    pub fn clamp_pitch(&mut self) {
        let limit = 89.0_f32.to_radians();
        self.pitch = self.pitch.clamp(-limit, limit);
    }
}

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

        transform.rotation = Quat::from_rotation_y(camera.yaw) * Quat::from_rotation_x(camera.pitch);
    }
}