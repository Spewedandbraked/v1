use macroquad::prelude::*;
use crate::player::{Player, GrabbedObject};
use crate::world::World;
use crate::world::systems::CollisionSystem;
use crate::input::{InputState, InputConfig, Action};
use crate::menu::GameUI;

pub struct Game {
    player: Player,
    world: World,
    ui: GameUI,
    movement_system: crate::player::movement::MovementSystem,
    collision_system: CollisionSystem,
    camera_system: crate::player::camera::CameraSystem,
    input: InputState,
    config: InputConfig,
    bob_offset: Vec3,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            world: World::new(),
            ui: GameUI::new(),
            movement_system: crate::player::movement::MovementSystem::new(),
            collision_system: CollisionSystem::new(),
            camera_system: crate::player::camera::CameraSystem::new(),
            input: InputState::new(),
            config: InputConfig::load(),
            bob_offset: Vec3::ZERO,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.world.update(delta_time);

        if is_key_pressed(KeyCode::F3) { self.ui.toggle_debug(); }
        if is_key_pressed(KeyCode::Escape) && !self.ui.is_rebinding() { self.ui.toggle_menu(&mut self.input); }

        if self.ui.is_rebinding() {
            self.ui.update_rebinding(&mut self.config);
        } else {
            self.input.update(&self.config);
            if self.ui.show_menu { self.ui.handle_menu_click(&mut self.config, &mut self.player); }
            else { self.update_gameplay(delta_time); }
        }
    }

    fn update_gameplay(&mut self, delta_time: f32) {
        let dt = delta_time.min(0.1);

        self.player.stats.update_respawn(dt);

        if self.player.stats.is_dead() {
            self.input.sprint = false;
            return;
        }

        if self.config.is_action_just_pressed(Action::ToggleGrid) { self.world.toggle_grid(); }
        if self.config.is_action_just_pressed(Action::InvertX) { self.player.camera.invert_x = !self.player.camera.invert_x; }
        if self.config.is_action_just_pressed(Action::InvertY) { self.player.camera.invert_y = !self.player.camera.invert_y; }
        if self.config.is_action_just_pressed(Action::Jump) { self.movement_system.jump(); }

        // ЛКМ — левая рука
        if is_mouse_button_down(MouseButton::Left) && self.player.grabbed_left.is_some() {
            self.player.is_charging_left = true;
            self.player.left_charge = (self.player.left_charge + dt * 2.0).min(1.0);
        }
        if is_mouse_button_released(MouseButton::Left) {
            if self.player.is_charging_left {
                let c = self.player.left_charge;
                self.throw_object(true, c);
                self.player.left_charge = 0.0;
                self.player.is_charging_left = false;
            } else if self.player.grabbed_left.is_none() {
                self.try_pickup(true);
            }
        }

        // ПКМ — правая рука
        if is_mouse_button_down(MouseButton::Right) && self.player.grabbed_right.is_some() {
            self.player.is_charging_right = true;
            self.player.right_charge = (self.player.right_charge + dt * 2.0).min(1.0);
        }
        if is_mouse_button_released(MouseButton::Right) {
            if self.player.is_charging_right {
                let c = self.player.right_charge;
                self.throw_object(false, c);
                self.player.right_charge = 0.0;
                self.player.is_charging_right = false;
            } else if self.player.grabbed_right.is_none() {
                self.try_pickup(false);
            }
        }

        // Враги
        let ppos = self.player.transform.position;
        for enemy in self.world.enemies.iter_mut() { enemy.update(ppos, dt); }

        // Враги атакуют игрока
        for enemy in &self.world.enemies {
            if enemy.alive {
                let dist = (self.player.transform.position - enemy.transform.position).length();
                if dist < 2.0 { self.player.stats.take_damage(15.0 * dt); }
            }
        }

        // Стамина
        let wants = self.input.sprint;
        let can = wants && self.player.stats.can_sprint();
        self.player.stats.update(wants, dt);
        let moving = self.input.move_forward || self.input.move_backward || self.input.move_left || self.input.move_right;
        self.bob_offset = self.camera_system.calculate_bob_offset(&mut self.player.camera, moving, moving && can, dt);
        if !can { self.input.sprint = false; }

        // Физика
        let gravity = -20.0;
        let ground_y = 0.0;
        let platforms: Vec<(crate::common::Transform, crate::common::Collider)> = self.world.platforms.iter().map(|p| (p.transform, p.collider)).collect();

        for int in self.world.interactables.iter_mut() {
            if !int.is_physics_active || int.is_grabbed { continue; }
            int.velocity.y += gravity * dt;
            int.position.y += int.velocity.y * dt;
            let collider = int.get_collider();
            let transform = crate::common::Transform::new(int.position);
            let mut collided = false;
            for (pt, pc) in &platforms {
                if let Some(pen) = self.collision_system.check_collision_direct(&transform, &collider, pt, pc) {
                    if pen.y.abs() > 0.001 {
                        int.position.y += pen.y;
                        if int.velocity.y < 0.0 && pen.y > 0.0 { int.velocity.y = 0.0; collided = true; }
                        else if int.velocity.y > 0.0 && pen.y < 0.0 { int.velocity.y = 0.0; }
                    }
                }
            }
            if int.position.y <= ground_y + int.size.y * 0.5 { int.position.y = ground_y + int.size.y * 0.5; int.velocity.y = 0.0; collided = true; }
            int.position.x += int.velocity.x * dt;
            int.position.z += int.velocity.z * dt;
            let transform = crate::common::Transform::new(int.position);
            for (pt, pc) in &platforms {
                if let Some(pen) = self.collision_system.check_collision_direct(&transform, &collider, pt, pc) {
                    int.position.x += pen.x; int.position.z += pen.z;
                    if pen.x.abs() > 0.001 { int.velocity.x = 0.0; }
                    if pen.z.abs() > 0.001 { int.velocity.z = 0.0; }
                }
            }
            if collided {
                int.velocity.x *= 0.8; int.velocity.z *= 0.8;
                if int.velocity.length() < 0.1 { int.velocity = Vec3::ZERO; int.is_physics_active = false; }
            }
            if int.velocity.length() > 3.0 {
                let pos = int.position;
                for (i, enemy) in self.world.enemies.iter_mut().enumerate() {
                    if !enemy.alive { continue; }
                    if int.hit_enemies.contains(&i) { continue; }
                    let dist = (pos - enemy.transform.position).length();
                    if dist < 1.5 {
                        enemy.take_damage(int.velocity.length() * 1.2);
                        int.hit_enemies.push(i);
                        let dir = (pos - enemy.transform.position).normalize();
                        int.velocity = dir * 8.0 + Vec3::new(0.0, 3.0, 0.0);
                    }
                }
            }
        }

        // Движение игрока
        self.movement_system.update(&mut self.player.transform, &self.input, dt);
        let grounded = self.collision_system.check_grounded(&self.player.transform, &self.player.collider, &platforms);
        self.movement_system.is_grounded = grounded;
        self.collision_system.resolve_collision(&mut self.player.transform, &self.player.collider, &platforms);
        self.camera_system.update(&mut self.player.transform, &mut self.player.camera, &self.input);
    }

    fn try_pickup(&mut self, is_left: bool) {
        let grabbed = if is_left { &self.player.grabbed_left } else { &self.player.grabbed_right };
        if grabbed.is_some() { return; }
        let eye = self.player.get_eye_position();
        let fwd = self.player.transform.forward();
        if let Some((idx, _)) = self.collision_system.raycast_interactable(eye, fwd, 5.0, &self.world.interactables) {
            let already = self.player.grabbed_left.as_ref().map(|g| g.world_index == idx).unwrap_or(false)
                || self.player.grabbed_right.as_ref().map(|g| g.world_index == idx).unwrap_or(false);
            if already { return; }
            let obj = &mut self.world.interactables[idx];
            obj.is_grabbed = true;
            obj.is_physics_active = false;
            obj.velocity = Vec3::ZERO;
            obj.hit_enemies.clear();
            let grab = GrabbedObject { size: obj.size, color: obj.color, world_index: idx };
            if is_left { self.player.grabbed_left = Some(grab); }
            else { self.player.grabbed_right = Some(grab); }
        }
    }

    fn throw_object(&mut self, is_left: bool, charge: f32) {
        let grabbed = if is_left { &self.player.grabbed_left } else { &self.player.grabbed_right };
        if let Some(obj) = grabbed {
            let idx = obj.world_index;
            let force = 5.0 + 20.0 * charge;
            let fwd = self.player.transform.forward();
            let eye = self.player.get_eye_position();
            if idx < self.world.interactables.len() {
                let int = &mut self.world.interactables[idx];
                int.position = eye + fwd * 1.5;
                int.velocity = fwd * force + Vec3::new(0.0, 3.0 * charge, 0.0);
                int.is_physics_active = true;
                int.is_grabbed = false;
            }
            if is_left { self.player.grabbed_left = None; }
            else { self.player.grabbed_right = None; }
        }
    }

    pub fn render(&self) {
        clear_background(self.world.get_background_color());

        // Экран смерти
        if self.player.stats.is_dead() {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(255, 0, 0, 100));
            let text = format!("YOU DIED\nRespawning in {:.1}...", self.player.stats.respawn_timer);
            let size = measure_text(&text, None, 60, 1.0);
            draw_text(&text, screen_width() * 0.5 - size.width * 0.5, screen_height() * 0.5 - size.height * 0.5, 60.0, Color::from_rgba(255, 255, 255, 255));
            return;
        }

        let eye = self.player.get_camera_transform().position + self.bob_offset;
        let fwd = self.player.get_camera_transform().forward();

        set_camera(&Camera3D { position: eye, up: Vec3::Y, target: eye + fwd, fovy: self.player.camera.fov, ..Default::default() });
        self.world.render();
        set_default_camera();

        set_camera(&Camera3D { position: Vec3::new(0.0, 0.0, -2.0), up: Vec3::Y, target: Vec3::ZERO, fovy: 60.0_f32.to_radians(), ..Default::default() });
        if let Some(ref grabbed) = self.player.grabbed_left {
            let charge = self.player.left_charge;
            let shake = if self.player.is_charging_left {
                let intensity = charge * 0.02;
                let t = (get_time() as f32 * 20.0) as i32 as f32;
                Vec3::new((t * 1.7).sin().signum() * intensity, (t * 2.3).sin().signum() * intensity, (t * 1.9).sin().signum() * intensity * 0.5)
            } else { Vec3::ZERO };
            let pos = Vec3::new(0.55, -0.45, 0.6) + shake;
            draw_cube(pos, grabbed.size * 0.7, None, grabbed.color);
            draw_cube_wires(pos, grabbed.size * 0.7, Color::from_rgba(0, 0, 0, 120));
        }
        if let Some(ref grabbed) = self.player.grabbed_right {
            let charge = self.player.right_charge;
            let shake = if self.player.is_charging_right {
                let intensity = charge * 0.02;
                let t = (get_time() as f32 * 20.0) as i32 as f32;
                Vec3::new((t * 1.9 + 1.0).sin().signum() * intensity, (t * 2.1 + 1.0).sin().signum() * intensity, (t * 1.5 + 1.0).sin().signum() * intensity * 0.5)
            } else { Vec3::ZERO };
            let pos = Vec3::new(-0.55, -0.45, 0.6) + shake;
            draw_cube(pos, grabbed.size * 0.7, None, grabbed.color);
            draw_cube_wires(pos, grabbed.size * 0.7, Color::from_rgba(0, 0, 0, 120));
        }
        set_default_camera();

        crate::player::ui::render_hud(&self.player, self.ui.show_menu);
        crate::player::ui::render_debug_info(&self.player, &self.movement_system, self.ui.show_debug);
        crate::player::ui::render_crosshair(self.ui.show_menu);
        if self.ui.show_menu { self.ui.render_menu(&self.player, &self.config); }
    }
}