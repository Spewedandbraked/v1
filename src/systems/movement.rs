use macroquad::prelude::*;
use crate::components::Transform;
use crate::input::InputState;

pub struct MovementSystem {
    pub walk_speed: f32,
    pub sprint_speed: f32,
    pub jump_force: f32,
    pub gravity: f32,
    pub velocity: Vec3,
    pub is_grounded: bool,
}

impl Default for MovementSystem {
    fn default() -> Self {
        Self {
            walk_speed: 5.0,
            sprint_speed: 8.0,
            jump_force: 5.0,
            gravity: 15.0,
            velocity: Vec3::ZERO,
            is_grounded: false,
        }
    }
}

impl MovementSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(
        &mut self,
        transform: &mut Transform,
        input: &InputState,
        delta_time: f32,
    ) {
        // Гравитация применяется ВСЕГДА, когда не на земле
        if !self.is_grounded {
            self.velocity.y -= self.gravity * delta_time;
        } else {
            // На земле сбрасываем вертикальную скорость
            self.velocity.y = 0.0;
        }

        // Скорость движения
        let speed = if input.sprint {
            self.sprint_speed
        } else {
            self.walk_speed
        };

        // Вектор движения
        let mut move_dir = Vec3::ZERO;
        
        // Вперед/назад
        if input.move_forward {
            move_dir += transform.forward();
        }
        if input.move_backward {
            move_dir -= transform.forward();
        }
        
        // Вправо/влево
        if input.move_right {
            move_dir -= transform.right();
        }
        if input.move_left {
            move_dir += transform.right();
        }
        
        // Вверх/вниз (для полета, если нужно)
        if input.move_up {
            move_dir += Vec3::Y;
        }
        if input.move_down {
            move_dir -= Vec3::Y;
        }

        // Применяем горизонтальное движение
        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
            
            // Сохраняем вертикальную скорость от гравитации/прыжка
            let vertical_velocity = self.velocity.y;
            
            // Устанавливаем горизонтальную скорость
            self.velocity.x = move_dir.x * speed;
            self.velocity.z = move_dir.z * speed;
            
            // Если используем клавиши вверх/вниз, переопределяем вертикальную скорость
            if input.move_up || input.move_down {
                self.velocity.y = move_dir.y * speed;
            } else {
                // Иначе сохраняем гравитацию/прыжок
                self.velocity.y = vertical_velocity;
            }
        } else {
            // Трение только для горизонтального движения
            self.velocity.x = 0.0;
            self.velocity.z = 0.0;
            
            // Вертикальная скорость от клавиш вверх/вниз
            if input.move_up {
                self.velocity.y = speed;
            } else if input.move_down {
                self.velocity.y = -speed;
            } else if !self.is_grounded {
                // Если не нажаты клавиши вверх/вниз и не на земле,
                // сохраняем вертикальную скорость от гравитации
                // (ничего не делаем, velocity.y уже содержит гравитацию)
            } else {
                // На земле без движения вверх/вниз - сбрасываем
                self.velocity.y = 0.0;
            }
        }

        // Применяем движение
        transform.position += self.velocity * delta_time;
        
        // Небольшая защита от проваливания сквозь землю
        if self.is_grounded && transform.position.y < 0.0 {
            transform.position.y = 0.0;
        }
    }

    pub fn jump(&mut self) {
        if self.is_grounded {
            self.velocity.y = self.jump_force;
            self.is_grounded = false;
        }
    }
}