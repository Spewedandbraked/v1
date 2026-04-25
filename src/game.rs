use macroquad::prelude::*;
use crate::player::{Player, movement::MovementSystem, camera::CameraSystem};
use crate::world::{World, systems::CollisionSystem};
use crate::input::{InputState, InputConfig, Action};
use crate::menu::GameUI;

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

    pub fn update(&mut self, delta_time: f32) {
        self.world.update(delta_time);

        if is_key_pressed(KeyCode::F3) {
            self.ui.toggle_debug();
        }

        if is_key_pressed(KeyCode::Escape) && !self.ui.is_rebinding() {
            self.ui.toggle_menu(&mut self.input);
        }

        if self.ui.is_rebinding() {
            self.ui.update_rebinding(&mut self.config);
        } else {
            self.input.update(&self.config);

            if self.ui.show_menu {
                self.ui.handle_menu_click(&mut self.config, &mut self.player);
            } else {
                self.update_gameplay(delta_time);
            }
        }
    }

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
        if self.config.is_action_just_pressed(Action::Jump) {
            self.movement_system.jump();
        }

        self.movement_system.update(&mut self.player.transform, &self.input, delta_time);

        self.movement_system.is_grounded = self.collision_system.check_grounded(
            &self.player.transform,
            &self.player.collider,
            &self.world.platforms.iter().map(|p| (p.transform, p.collider)).collect::<Vec<_>>(),
        );

        let platforms_data: Vec<_> = self.world.platforms.iter().map(|p| (p.transform, p.collider)).collect();
        self.collision_system.resolve_collision(&mut self.player.transform, &self.player.collider, &platforms_data);

        self.camera_system.update(&mut self.player.transform, &mut self.player.camera, &self.input);
    }

    pub fn render(&self) {
        clear_background(self.world.get_background_color());

        let camera_transform = self.player.get_camera_transform();
        set_camera(&Camera3D {
            position: camera_transform.position,
            up: Vec3::Y,
            target: camera_transform.position + camera_transform.forward(),
            fovy: self.player.camera.fov,
            ..Default::default()
        });

        self.world.render();
        set_default_camera();

        crate::player::ui::render_debug_info(&self.player, &self.movement_system, self.ui.show_debug);
        crate::player::ui::render_crosshair(self.ui.show_menu);

        if self.ui.show_menu {
            self.ui.render_menu(&self.player, &self.config);
        }
    }
}