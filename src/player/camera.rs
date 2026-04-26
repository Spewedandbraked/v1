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
    pub bob_timer: f32,
    pub bob_amount: f32,
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
            bob_timer: 0.0,
            bob_amount: 0.0,
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

    pub fn calculate_bob_offset(
        &self,
        camera: &mut CameraComponent,
        is_moving: bool,
        is_sprinting: bool,
        delta_time: f32,
    ) -> Vec3 {
        let bob_speed = if is_sprinting { 14.0 } else { 10.0 };
        let target_amount = if is_sprinting { 0.12 } else { 0.07 };
        
        if is_moving {
            camera.bob_timer += delta_time * bob_speed;
            // Резкое нарастание
            camera.bob_amount = camera.bob_amount + (target_amount - camera.bob_amount) * 15.0 * delta_time;
        } else {
            camera.bob_timer = 0.0;
            // Резкое затухание
            camera.bob_amount = camera.bob_amount + (0.0 - camera.bob_amount) * 15.0 * delta_time;
        }
        
        // Дёрганое движение: используем резкие пики
        let phase = camera.bob_timer;
        let bob_x = phase.sin() * camera.bob_amount;
        // Вертикальное: резкие провалы вниз с быстрым подъёмом
        let raw_y = (phase * 2.0).sin();
        let bob_y = if raw_y < 0.0 {
            raw_y.abs().powf(0.7) * camera.bob_amount * 1.5
        } else {
            raw_y.powf(0.5) * camera.bob_amount * 1.2
        };
        
        Vec3::new(bob_x, -bob_y, 0.0)
    }
}