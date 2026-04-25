use macroquad::prelude::*;
use crate::common::{Transform, Collider};

pub struct CollisionSystem;

impl CollisionSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve_collision(
        &self,
        player_transform: &mut Transform,
        player_collider: &Collider,
        platforms: &[(Transform, Collider)],
    ) -> bool {
        let mut collided = false;
        let mut correction = Vec3::ZERO;

        for (platform_transform, platform_collider) in platforms {
            if let Some(penetration) = self.check_collision(
                player_transform,
                player_collider,
                platform_transform,
                platform_collider,
            ) {
                collided = true;
                correction += penetration;
            }
        }

        if collided {
            player_transform.position += correction;
        }
        collided
    }

    fn check_collision(
        &self,
        transform_a: &Transform,
        collider_a: &Collider,
        transform_b: &Transform,
        collider_b: &Collider,
    ) -> Option<Vec3> {
        match (collider_a, collider_b) {
            (Collider::Sphere(sphere_a), Collider::Sphere(sphere_b)) => {
                self.sphere_sphere(transform_a.position, sphere_a.radius, transform_b.position, sphere_b.radius)
            }
            (Collider::Sphere(sphere), Collider::AABB(aabb)) => {
                self.sphere_aabb(transform_a.position, sphere.radius, transform_b.position, aabb.half_extents)
            }
            (Collider::AABB(aabb), Collider::Sphere(sphere)) => {
                self.sphere_aabb(transform_b.position, sphere.radius, transform_a.position, aabb.half_extents)
                    .map(|v| -v)
            }
            _ => None,
        }
    }

    fn sphere_sphere(&self, pos_a: Vec3, r_a: f32, pos_b: Vec3, r_b: f32) -> Option<Vec3> {
        let delta = pos_b - pos_a;
        let dist = delta.length();
        let min_dist = r_a + r_b;
        if dist < min_dist && dist > 0.001 {
            Some(-delta.normalize() * (min_dist - dist))
        } else {
            None
        }
    }

    fn sphere_aabb(&self, sphere_pos: Vec3, radius: f32, aabb_pos: Vec3, half: Vec3) -> Option<Vec3> {
        let min = aabb_pos - half;
        let max = aabb_pos + half;
        let closest = Vec3::new(
            sphere_pos.x.clamp(min.x, max.x),
            sphere_pos.y.clamp(min.y, max.y),
            sphere_pos.z.clamp(min.z, max.z),
        );
        let delta = closest - sphere_pos;
        let dist = delta.length();
        if dist < radius && dist > 0.001 {
            Some(-delta.normalize() * (radius - dist))
        } else {
            None
        }
    }

    pub fn check_grounded(
        &self,
        player_transform: &Transform,
        player_collider: &Collider,
        platforms: &[(Transform, Collider)],
    ) -> bool {
        if let Collider::Sphere(sphere) = player_collider {
            let check_pos = player_transform.position - Vec3::new(0.0, sphere.radius + 0.1, 0.0);
            for (platform_transform, platform_collider) in platforms {
                if let Collider::AABB(aabb) = platform_collider {
                    let min = platform_transform.position - aabb.half_extents;
                    let max = platform_transform.position + aabb.half_extents;
                    if check_pos.x >= min.x && check_pos.x <= max.x
                        && check_pos.z >= min.z && check_pos.z <= max.z
                        && check_pos.y >= min.y - 0.1 && check_pos.y <= max.y + 0.1
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
}