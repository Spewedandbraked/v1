use macroquad::prelude::*;
use crate::player::{Player, movement::MovementSystem, camera::CameraSystem, GrabbedObject};
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
    bob_offset: Vec3,
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
            bob_offset: Vec3::ZERO,
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
        let dt = delta_time.min(0.1);

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

        // Зарядка левой руки (ЛКМ)
        if is_mouse_button_down(MouseButton::Left) && self.player.grabbed_left.is_some() {
            self.player.is_charging_left = true;
            self.player.left_charge = (self.player.left_charge + dt * 2.0).min(1.0);
        }
        if is_mouse_button_released(MouseButton::Left) {
            if self.player.is_charging_left {
                let charge = self.player.left_charge;
                self.throw_object(true, charge);
                self.player.left_charge = 0.0;
                self.player.is_charging_left = false;
            } else if self.player.grabbed_left.is_none() {
                self.handle_interact(true);
            }
        }
        
        // Зарядка правой руки (ПКМ)
        if is_mouse_button_down(MouseButton::Right) && self.player.grabbed_right.is_some() {
            self.player.is_charging_right = true;
            self.player.right_charge = (self.player.right_charge + dt * 2.0).min(1.0);
        }
        if is_mouse_button_released(MouseButton::Right) {
            if self.player.is_charging_right {
                let charge = self.player.right_charge;
                self.throw_object(false, charge);
                self.player.right_charge = 0.0;
                self.player.is_charging_right = false;
            } else if self.player.grabbed_right.is_none() {
                self.handle_interact(false);
            }
        }

        let wants_to_sprint = self.input.sprint;
        let can_sprint = wants_to_sprint && self.player.stats.can_sprint();
        self.player.stats.update(wants_to_sprint, dt);

        let is_moving = self.input.move_forward || self.input.move_backward 
            || self.input.move_left || self.input.move_right;
        let is_sprinting = is_moving && can_sprint;
        self.bob_offset = self.camera_system.calculate_bob_offset(
            &mut self.player.camera, is_moving, is_sprinting, dt,
        );

        if !can_sprint {
            self.input.sprint = false;
        }

        let gravity = -20.0;
        let ground_y = 0.0;
        let platforms_data: Vec<(crate::common::Transform, crate::common::Collider)> = 
            self.world.platforms.iter().map(|p| (p.transform, p.collider)).collect();

        for interactable in self.world.interactables.iter_mut() {
            if !interactable.is_physics_active || interactable.is_grabbed {
                continue;
            }

            interactable.velocity.y += gravity * dt;
            interactable.position.y += interactable.velocity.y * dt;
            
            let collider = interactable.get_collider();
            let transform = crate::common::Transform::new(interactable.position);
            let mut collided = false;
            
            for (platform_transform, platform_collider) in &platforms_data {
                if let Some(penetration) = self.collision_system.check_collision_direct(
                    &transform, &collider,
                    platform_transform, platform_collider,
                ) {
                    if penetration.y.abs() > 0.001 {
                        interactable.position.y += penetration.y;
                        if interactable.velocity.y < 0.0 && penetration.y > 0.0 {
                            interactable.velocity.y = 0.0;
                            collided = true;
                        } else if interactable.velocity.y > 0.0 && penetration.y < 0.0 {
                            interactable.velocity.y = 0.0;
                        }
                    }
                }
            }
            
            let half_height = interactable.size.y * 0.5;
            if interactable.position.y <= ground_y + half_height {
                interactable.position.y = ground_y + half_height;
                interactable.velocity.y = 0.0;
                collided = true;
            }
            
            interactable.position.x += interactable.velocity.x * dt;
            interactable.position.z += interactable.velocity.z * dt;
            
            let transform = crate::common::Transform::new(interactable.position);
            for (platform_transform, platform_collider) in &platforms_data {
                if let Some(penetration) = self.collision_system.check_collision_direct(
                    &transform, &collider,
                    platform_transform, platform_collider,
                ) {
                    interactable.position.x += penetration.x;
                    interactable.position.z += penetration.z;
                    if penetration.x.abs() > 0.001 {
                        interactable.velocity.x = 0.0;
                    }
                    if penetration.z.abs() > 0.001 {
                        interactable.velocity.z = 0.0;
                    }
                }
            }

            if collided {
                interactable.velocity.x *= 0.8;
                interactable.velocity.z *= 0.8;
                
                if interactable.velocity.length() < 0.1 {
                    interactable.velocity = Vec3::ZERO;
                    interactable.is_physics_active = false;
                }
            }
        }

        self.movement_system.update(&mut self.player.transform, &self.input, dt);

        self.movement_system.is_grounded = self.collision_system.check_grounded(
            &self.player.transform,
            &self.player.collider,
            &self.world.platforms.iter().map(|p| (p.transform, p.collider)).collect::<Vec<_>>(),
        );

        let platforms_data: Vec<_> = self.world.platforms.iter().map(|p| (p.transform, p.collider)).collect();
        self.collision_system.resolve_collision(&mut self.player.transform, &self.player.collider, &platforms_data);

        self.camera_system.update(&mut self.player.transform, &mut self.player.camera, &self.input);
    }

    fn throw_object(&mut self, is_left: bool, charge: f32) {
        let min_force = 5.0;
        let max_force = 25.0;
        let throw_force = min_force + (max_force - min_force) * charge;
        let forward = self.player.transform.forward();
        let eye_pos = self.player.get_eye_position();
        
        let grabbed = if is_left { &self.player.grabbed_left } else { &self.player.grabbed_right };
        if let Some(obj) = grabbed {
            let idx = obj.world_index;
            if idx < self.world.interactables.len() {
                let interactable = &mut self.world.interactables[idx];
                interactable.position = eye_pos + forward * 1.5;
                interactable.velocity = forward * throw_force + Vec3::new(0.0, 3.0 * charge, 0.0);
                interactable.is_physics_active = true;
                interactable.is_grabbed = false;
            }
        }
        
        if is_left {
            self.player.grabbed_left = None;
        } else {
            self.player.grabbed_right = None;
        }
    }

    fn handle_interact(&mut self, is_left: bool) {
        let grabbed = if is_left { &self.player.grabbed_left } else { &self.player.grabbed_right };
        
        if grabbed.is_some() {
            return;
        }
        
        let eye_pos = self.player.get_eye_position();
        let forward = self.player.transform.forward();

        if let Some((idx, _)) = self.collision_system.raycast_interactable(
            eye_pos,
            forward,
            5.0,
            &self.world.interactables,
        ) {
            let already_grabbed = self.player.grabbed_left.as_ref().map(|g| g.world_index == idx).unwrap_or(false)
                || self.player.grabbed_right.as_ref().map(|g| g.world_index == idx).unwrap_or(false);
            
            if already_grabbed {
                return;
            }
            
            let interactable = &mut self.world.interactables[idx];
            interactable.is_grabbed = true;
            interactable.is_physics_active = false;
            interactable.velocity = Vec3::ZERO;

            let obj = GrabbedObject {
                size: interactable.size,
                color: interactable.color,
                world_index: idx,
            };
            
            if is_left {
                self.player.grabbed_left = Some(obj);
            } else {
                self.player.grabbed_right = Some(obj);
            }
        }
    }

    pub fn render(&self) {
        clear_background(self.world.get_background_color());

        let camera_transform = self.player.get_camera_transform();
        let eye_pos = camera_transform.position + self.bob_offset;
        let forward = camera_transform.forward();
        
        set_camera(&Camera3D {
            position: eye_pos,
            up: Vec3::Y,
            target: eye_pos + forward,
            fovy: self.player.camera.fov,
            ..Default::default()
        });

        self.world.render();
        set_default_camera();
        
        let hand_camera_pos = Vec3::new(0.0, 0.0, -2.0);
        set_camera(&Camera3D {
            position: hand_camera_pos,
            up: Vec3::Y,
            target: Vec3::ZERO,
            fovy: 60.0_f32.to_radians(),
            ..Default::default()
        });
        
        // Левая рука с дёрганым дрожанием
        if let Some(ref grabbed) = self.player.grabbed_left {
            let charge = self.player.left_charge;
            let shake = if self.player.is_charging_left {
                let intensity = charge * 0.02;
                let t = (get_time() as f32 * 20.0) as i32 as f32;
                Vec3::new(
                    (t * 1.7).sin().signum() * intensity,
                    (t * 2.3).sin().signum() * intensity,
                    (t * 1.9).sin().signum() * intensity * 0.5,
                )
            } else {
                Vec3::ZERO
            };
            let pos = Vec3::new(0.55, -0.45, 0.6) + shake;
            draw_cube(pos, grabbed.size * 0.7, None, grabbed.color);
            draw_cube_wires(pos, grabbed.size * 0.7, Color::from_rgba(0, 0, 0, 120));
        }
        
        // Правая рука с дёрганым дрожанием
        if let Some(ref grabbed) = self.player.grabbed_right {
            let charge = self.player.right_charge;
            let shake = if self.player.is_charging_right {
                let intensity = charge * 0.02;
                let t = (get_time() as f32 * 20.0) as i32 as f32;
                Vec3::new(
                    (t * 1.9 + 1.0).sin().signum() * intensity,
                    (t * 2.1 + 1.0).sin().signum() * intensity,
                    (t * 1.5 + 1.0).sin().signum() * intensity * 0.5,
                )
            } else {
                Vec3::ZERO
            };
            let pos = Vec3::new(-0.55, -0.45, 0.6) + shake;
            draw_cube(pos, grabbed.size * 0.7, None, grabbed.color);
            draw_cube_wires(pos, grabbed.size * 0.7, Color::from_rgba(0, 0, 0, 120));
        }
        
        set_default_camera();

        crate::player::ui::render_hud(&self.player, self.ui.show_menu);
        crate::player::ui::render_debug_info(&self.player, &self.movement_system, self.ui.show_debug);
        crate::player::ui::render_crosshair(self.ui.show_menu);

        if self.ui.show_menu {
            self.ui.render_menu(&self.player, &self.config);
        }
    }
}