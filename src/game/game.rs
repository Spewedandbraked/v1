use crate::entities::Player;
use crate::game::{GameUI, World};
use crate::input::{Action, InputConfig, InputState};
use crate::systems::{CameraSystem, CollisionSystem, MovementSystem};
use macroquad::prelude::*;

pub struct Game {
    player: Player,
    world: World,
    ui: GameUI,
    movement_system: MovementSystem,
    collision_system: CollisionSystem,
    camera_system: CameraSystem,
    input: InputState,
    config: InputConfig,
}

impl Game {
    /// Создаёт игровой контейнер и инициализирует все подсистемы.
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            world: World::new(),
            ui: GameUI::new(),
            movement_system: MovementSystem::new(),
            collision_system: CollisionSystem::new(),
            camera_system: CameraSystem::new(),
            input: InputState::new(),
            config: InputConfig::load(),
        }
    }

    /// Обновляет состояние мира, UI и геймплей в рамках одного кадра.
    pub fn update(&mut self, delta_time: f32) {
        self.world.update(delta_time);

        if is_key_pressed(KeyCode::F3) {
            self.ui.toggle_debug();
        }

        if self.ui.is_rebinding() {
            self.ui.update_rebinding(&mut self.config);
        } else {
            self.input.update(&self.config);

            if self.config.is_action_just_pressed(Action::ToggleMenu) {
                self.ui.toggle_menu(&mut self.input);
            }

            if self.ui.show_menu {
                self.ui.handle_menu_click(&mut self.config);
            } else {
                self.update_gameplay(delta_time);
            }
        }
    }

    /// Обрабатывает игровую логику, когда меню закрыто.
    fn update_gameplay(&mut self, delta_time: f32) {
        if self.config.is_action_just_pressed(Action::ToggleGrid) {
            self.world.toggle_grid();
        }

        if self.config.is_action_just_pressed(Action::InvertX) {
            self.player.camera.invert_x = !self.player.camera.invert_x;
        }

        if self.config.is_action_just_pressed(Action::InvertY) {
            self.player.camera.invert_y = !self.player.camera.invert_y;
        }

        self.handle_sensitivity_adjustment();

        if self.config.is_action_just_pressed(Action::Jump) {
            self.movement_system.jump();
        }

        self.movement_system
            .update(&mut self.player.transform, &self.input, delta_time);

        self.movement_system.is_grounded = self.collision_system.check_grounded(
            &self.player.transform,
            &self.player.collider,
            &self
                .world
                .platforms
                .iter()
                .map(|p| (p.transform, p.collider))
                .collect::<Vec<_>>(),
        );

        self.resolve_collisions();

        self.camera_system.update(
            &mut self.player.transform,
            &mut self.player.camera,
            &self.input,
        );
    }

    /// Меняет чувствительность камеры с помощью колеса мыши.
    fn handle_sensitivity_adjustment(&mut self) {
        let mouse_wheel = mouse_wheel().1;
        if mouse_wheel != 0.0 {
            self.player.camera.sensitivity += mouse_wheel * 0.05;
            self.player.camera.sensitivity = self.player.camera.sensitivity.clamp(0.1, 2.0);
        }
    }

    /// Собирает коллайдеры платформ и разрешает столкновения игрока с ними.
    fn resolve_collisions(&mut self) {
        let platforms_data: Vec<_> = self
            .world
            .platforms
            .iter()
            .map(|p| (p.transform, p.collider))
            .collect();

        self.collision_system.resolve_collision(
            &mut self.player.transform,
            &self.player.collider,
            &platforms_data,
        );
    }

    /// Отрисовывает 3D-сцену и затем поверх неё элементы интерфейса.
    pub fn render(&self) {
        clear_background(self.world.get_background_color());

        let camera_transform = self.player.get_camera_transform();
        set_camera(&Camera3D {
            position: camera_transform.position,
            up: Vec3::Y,
            fovy: self.player.camera.fov,
            target: camera_transform.position + camera_transform.forward(),
            ..Default::default()
        });

        self.world.render();

        set_default_camera();
        self.render_ui();
    }

    /// Рисует HUD, дебаг-информацию и меню управления.
    fn render_ui(&self) {
        self.ui
            .render_debug_info(&self.player, &self.movement_system);
        self.ui.render_crosshair();

        if self.ui.show_menu {
            self.ui.render_menu(&self.player, &self.config);
        }
    }
}
