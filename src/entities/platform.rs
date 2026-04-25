use macroquad::prelude::*;
use macroquad::color::colors;
use crate::components::{Transform, Collider};

#[derive(Debug, Clone)]
pub struct Platform {
    pub transform: Transform,
    pub collider: Collider,
    pub color: Color,
}

impl Platform {
    /// Создаёт платформу с заданной позицией, размером и цветом.
    pub fn new(position: Vec3, size: Vec3, color: Color) -> Self {
        Self {
            transform: Transform::new(position),
            collider: Collider::aabb(size * 0.5),
            color,
        }
    }

    /// Отрисовывает платформу как заполненный куб с контуром.
    pub fn render(&self) {
        let half = match &self.collider {
            Collider::AABB(aabb) => aabb.half_extents,
            _ => Vec3::ONE * 0.5,
        };

        draw_cube(self.transform.position, half * 2.0, None, self.color);
        draw_cube_wires(self.transform.position, half * 2.0, colors::BLACK);
    }
}