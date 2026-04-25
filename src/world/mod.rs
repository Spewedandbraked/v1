use macroquad::color::colors;
use macroquad::prelude::*;
pub mod systems;

#[derive(Debug, Clone)]
pub struct Platform {
    pub transform: crate::common::Transform,
    pub collider: crate::common::Collider,
    pub color: Color,
}

impl Platform {
    pub fn new(position: Vec3, size: Vec3, color: Color) -> Self {
        Self {
            transform: crate::common::Transform::new(position),
            collider: crate::common::Collider::aabb(size * 0.5),
            color,
        }
    }

    pub fn render(&self) {
        let half = match &self.collider {
            crate::common::Collider::AABB(aabb) => aabb.half_extents,
            _ => Vec3::ONE * 0.5,
        };
        draw_cube(self.transform.position, half * 2.0, None, self.color);
        draw_cube_wires(self.transform.position, half * 2.0, colors::BLACK);
    }
}

#[derive(Clone)]
pub struct Decoration {
    pub position: Vec3,
    pub decoration_type: DecorationType,
}

#[derive(Clone)]
pub enum DecorationType {
    FloatingSphere { radius: f32, color_offset: f32 },
}

#[derive(Debug, Clone)]
pub struct Interactable {
    pub position: Vec3,
    pub size: Vec3,
    pub color: Color,
    pub is_grabbed: bool,
    pub velocity: Vec3,
    pub is_physics_active: bool,
}

impl Interactable {
    pub fn new(position: Vec3, size: Vec3, color: Color) -> Self {
        Self {
            position,
            size,
            color,
            is_grabbed: false,
            velocity: Vec3::ZERO,
            is_physics_active: false,
        }
    }

    pub fn render(&self) {
        if !self.is_grabbed {
            draw_cube(self.position, self.size, None, self.color);
            draw_cube_wires(self.position, self.size, Color::from_rgba(0, 0, 0, 100));
            
            if !self.is_physics_active {
                draw_sphere(
                    self.position + Vec3::new(0.0, self.size.y * 0.5 + 0.5, 0.0),
                    0.15,
                    None,
                    Color::from_rgba(255, 255, 255, 200),
                );
            }
        }
    }
    
    pub fn get_collider(&self) -> crate::common::Collider {
        crate::common::Collider::aabb(self.size * 0.5)
    }
}

pub struct World {
    pub platforms: Vec<Platform>,
    decorations: Vec<Decoration>,
    pub interactables: Vec<Interactable>,
    pub grid_visible: bool,
    time: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            platforms: Self::create_platforms(),
            decorations: Self::create_decorations(),
            interactables: Self::create_interactables(),
            grid_visible: true,
            time: 0.0,
        }
    }

    fn create_platforms() -> Vec<Platform> {
        vec![
            Platform::new(
                Vec3::new(0.0, -0.5, 0.0),
                Vec3::new(20.0, 1.0, 20.0),
                Color::from_rgba(60, 60, 80, 255),
            ),
            Platform::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(8.0, 0.5, 8.0),
                Color::from_rgba(80, 80, 100, 255),
            ),
            Platform::new(
                Vec3::new(3.0, 1.0, 4.0),
                Vec3::new(2.0, 0.5, 2.0),
                Color::from_rgba(255, 150, 100, 255),
            ),
            Platform::new(
                Vec3::new(4.0, 2.0, 5.0),
                Vec3::new(2.0, 0.5, 2.0),
                Color::from_rgba(255, 150, 100, 255),
            ),
            Platform::new(
                Vec3::new(5.0, 3.0, 6.0),
                Vec3::new(2.0, 0.5, 2.0),
                Color::from_rgba(255, 150, 100, 255),
            ),
            Platform::new(
                Vec3::new(-4.0, 1.5, -3.0),
                Vec3::new(1.0, 3.0, 1.0),
                Color::from_rgba(150, 100, 255, 255),
            ),
            Platform::new(
                Vec3::new(4.0, 1.5, -3.0),
                Vec3::new(1.0, 3.0, 1.0),
                Color::from_rgba(150, 100, 255, 255),
            ),
            Platform::new(
                Vec3::new(0.0, 3.0, -3.0),
                Vec3::new(5.0, 0.5, 1.5),
                Color::from_rgba(100, 255, 150, 255),
            ),
            Platform::new(
                Vec3::new(-5.0, 2.0, 5.0),
                Vec3::new(2.0, 4.0, 2.0),
                Color::from_rgba(255, 100, 255, 255),
            ),
            Platform::new(
                Vec3::new(-5.0, 5.0, 5.0),
                Vec3::new(3.0, 0.5, 3.0),
                Color::from_rgba(255, 100, 255, 255),
            ),
        ]
    }

    fn create_interactables() -> Vec<Interactable> {
        vec![
            Interactable::new(
                Vec3::new(2.0, 1.0, 7.0),
                Vec3::new(1.0, 1.0, 1.0),
                Color::from_rgba(255, 50, 50, 255),
            ),
            Interactable::new(
                Vec3::new(-3.0, 1.0, 8.0),
                Vec3::new(0.8, 0.8, 0.8),
                Color::from_rgba(255, 100, 50, 255),
            ),
            Interactable::new(
                Vec3::new(0.0, 1.0, 10.0),
                Vec3::new(0.6, 0.6, 0.6),
                Color::from_rgba(50, 255, 100, 255),
            ),
        ]
    }

    fn create_decorations() -> Vec<Decoration> {
        vec![
            Decoration {
                position: Vec3::new(8.0, 5.0, 8.0),
                decoration_type: DecorationType::FloatingSphere {
                    radius: 0.3,
                    color_offset: 0.0,
                },
            },
            Decoration {
                position: Vec3::new(-8.0, 4.0, 6.0),
                decoration_type: DecorationType::FloatingSphere {
                    radius: 0.3,
                    color_offset: 1.0,
                },
            },
            Decoration {
                position: Vec3::new(6.0, 6.0, -8.0),
                decoration_type: DecorationType::FloatingSphere {
                    radius: 0.3,
                    color_offset: 2.0,
                },
            },
            Decoration {
                position: Vec3::new(-6.0, 7.0, -6.0),
                decoration_type: DecorationType::FloatingSphere {
                    radius: 0.3,
                    color_offset: 3.0,
                },
            },
        ]
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
    }

    pub fn render(&self) {
        if self.grid_visible {
            self.render_grid();
        }
        self.render_decorations();
        for platform in &self.platforms {
            platform.render();
        }
        for interactable in &self.interactables {
            interactable.render();
        }
    }

    fn render_grid(&self) {
        let grid_size = 20;
        let spacing = 1.0;
        for i in -grid_size..=grid_size {
            let i = i as f32;
            draw_line_3d(
                Vec3::new(i * spacing, 0.01, -grid_size as f32 * spacing),
                Vec3::new(i * spacing, 0.01, grid_size as f32 * spacing),
                Color::from_rgba(100, 100, 100, 100),
            );
            draw_line_3d(
                Vec3::new(-grid_size as f32 * spacing, 0.01, i * spacing),
                Vec3::new(grid_size as f32 * spacing, 0.01, i * spacing),
                Color::from_rgba(100, 100, 100, 100),
            );
        }
    }

    fn render_decorations(&self) {
        for (i, decoration) in self.decorations.iter().enumerate() {
            let DecorationType::FloatingSphere {
                radius,
                color_offset,
            } = decoration.decoration_type;
            let offset = (self.time + i as f32) * 0.5 + color_offset;
            let color = Color::from_rgba(
                128 + (offset.sin() * 64.0) as u8,
                128 + (offset.cos() * 64.0) as u8,
                200,
                150,
            );
            draw_sphere(decoration.position, radius, None, color);
        }
        for i in 0..5 {
            let angle = (self.time * 0.2 + i as f32 * 1.2) % (std::f32::consts::PI * 2.0);
            let x = angle.sin() * 10.0;
            let z = angle.cos() * 10.0;
            draw_line_3d(
                Vec3::new(x, 10.0, z),
                Vec3::new(x, -1.0, z),
                Color::from_rgba(255, 255, 200, 30),
            );
        }
    }

    pub fn toggle_grid(&mut self) {
        self.grid_visible = !self.grid_visible;
    }

    pub fn get_background_color(&self) -> Color {
        Color::from_rgba(
            30 + (self.time.sin() * 5.0) as u8,
            30 + (self.time.cos() * 5.0) as u8,
            50 + (self.time.sin() * 0.5 * 10.0) as u8,
            255,
        )
    }
}