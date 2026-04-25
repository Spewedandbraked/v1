pub mod camera;
pub mod movement;
pub mod ui;

use macroquad::prelude::*;
use crate::common::{Transform, Collider};

#[derive(Clone)]
pub struct GrabbedObject {
    pub position: Vec3,
    pub size: Vec3,
    pub color: Color,
    pub original_position: Vec3,
}

pub struct Player {
    pub transform: Transform,
    pub collider: Collider,
    pub camera: camera::CameraComponent,
    pub height: f32,
    pub eye_height: f32,
    pub grabbed_object: Option<GrabbedObject>,
}

impl Default for Player {
    fn default() -> Self {
        let height = 1.8;
        let eye_height = height * 0.9;
        Self {
            transform: Transform::new(Vec3::new(0.0, height, 5.0)),
            collider: Collider::sphere(0.5),
            camera: camera::CameraComponent::default(),
            height,
            eye_height,
            grabbed_object: None,
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_eye_position(&self) -> Vec3 {
        self.transform.position + Vec3::new(0.0, self.eye_height - self.height * 0.5, 0.0)
    }

    pub fn get_camera_transform(&self) -> Transform {
        Transform {
            position: self.get_eye_position(),
            rotation: self.transform.rotation,
        }
    }
}