use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct AABBCollider {
    pub half_extents: Vec3,
}

impl AABBCollider {
    pub fn new(half_extents: Vec3) -> Self {
        Self { half_extents }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SphereCollider {
    pub radius: f32,
}

impl SphereCollider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Collider {
    AABB(AABBCollider),
    Sphere(SphereCollider),
}

impl Collider {
    pub fn aabb(half_extents: Vec3) -> Self {
        Collider::AABB(AABBCollider::new(half_extents))
    }

    pub fn sphere(radius: f32) -> Self {
        Collider::Sphere(SphereCollider::new(radius))
    }
}