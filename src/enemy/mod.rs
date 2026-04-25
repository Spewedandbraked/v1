use macroquad::prelude::*;
use macroquad::color::colors;
use crate::common::Transform;

#[derive(Clone)]
pub struct Enemy {
    pub transform: Transform,
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
    pub color: Color,
    pub size: Vec3,
    pub alive: bool,
}

impl Enemy {
    pub fn new(position: Vec3) -> Self {
        Self {
            transform: Transform::new(position),
            health: 50.0,
            max_health: 50.0,
            speed: 2.0,
            color: Color::from_rgba(255, 50, 50, 255),
            size: Vec3::new(1.5, 2.0, 1.5),
            alive: true,
        }
    }

    pub fn update(&mut self, player_pos: Vec3, delta_time: f32) {
        if !self.alive {
            return;
        }

        let dir = (player_pos - self.transform.position).normalize();
        self.transform.position += dir * self.speed * delta_time;
        
        let angle = dir.z.atan2(dir.x);
        self.transform.rotation = Quat::from_rotation_y(-angle + std::f32::consts::PI);
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health -= amount;
        if self.health <= 0.0 {
            self.health = 0.0;
            self.alive = false;
        }
    }

    pub fn render(&self) {
        if !self.alive {
            return;
        }

        let half = self.size * 0.5;
        let pos = self.transform.position;
        
        draw_cube(pos, self.size, None, self.color);
        draw_cube_wires(pos, self.size, Color::from_rgba(0, 0, 0, 150));
        
        let eye_offset = Vec3::new(0.3, 0.3, half.z + 0.01);
        draw_sphere(pos + eye_offset, 0.15, None, colors::WHITE);
        draw_sphere(pos + Vec3::new(-eye_offset.x, eye_offset.y, eye_offset.z), 0.15, None, colors::WHITE);
        draw_sphere(pos + eye_offset, 0.08, None, colors::BLACK);
        draw_sphere(pos + Vec3::new(-eye_offset.x, eye_offset.y, eye_offset.z), 0.08, None, colors::BLACK);
        
        let health_bar_width = self.size.x;
        let health_bar_height = 0.1;
        let health_bar_y = pos.y + half.y + 0.3;
        
        draw_cube(
            Vec3::new(pos.x, health_bar_y, pos.z),
            Vec3::new(health_bar_width, health_bar_height, 0.05),
            None,
            Color::from_rgba(50, 50, 50, 255),
        );
        
        let health_ratio = self.health / self.max_health;
        let health_color = if health_ratio > 0.5 {
            Color::from_rgba(50, 200, 50, 255)
        } else if health_ratio > 0.25 {
            Color::from_rgba(255, 200, 50, 255)
        } else {
            Color::from_rgba(255, 50, 50, 255)
        };
        
        draw_cube(
            Vec3::new(pos.x - health_bar_width * 0.5 * (1.0 - health_ratio), health_bar_y, pos.z),
            Vec3::new(health_bar_width * health_ratio, health_bar_height, 0.06),
            None,
            health_color,
        );
    }
}