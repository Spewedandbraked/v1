use macroquad::prelude::*;

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