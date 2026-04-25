use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct AABBCollider {
    pub half_extents: Vec3,
}

impl AABBCollider {
    /// Создаёт AABB-коллайдер с полуразмерами по осям.
    pub fn new(half_extents: Vec3) -> Self {
        Self { half_extents }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SphereCollider {
    pub radius: f32,
}

impl SphereCollider {
    /// Создаёт сферический коллайдер с указанным радиусом.
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
    /// Удобный конструктор enum-коллайдера типа AABB.
    pub fn aabb(half_extents: Vec3) -> Self {
        Collider::AABB(AABBCollider::new(half_extents))
    }

    /// Удобный конструктор enum-коллайдера типа Sphere.
    pub fn sphere(radius: f32) -> Self {
        Collider::Sphere(SphereCollider::new(radius))
    }
}