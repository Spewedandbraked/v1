use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Default for Transform {
    /// Возвращает трансформ в начале координат без поворота.
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
        }
    }
}

impl Transform {
    /// Создаёт трансформ с заданной позицией и нулевым поворотом.
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    /// Возвращает локальный вектор "вперёд" с учётом текущего поворота.
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::Z
    }

    /// Возвращает локальный вектор "вправо" с учётом текущего поворота.
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }
}