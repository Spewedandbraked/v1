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

        if self.config.is_action_just_pressed(Action::Interact) {
            if self.player.grabbed_object.is_some() {
                let throw_force = 12.0;
                let forward = self.player.transform.forward();
                let eye_pos = self.player.get_eye_position();
                
                if let Some(interactable) = self.world.interactables.iter_mut().find(|i| i.is_grabbed) {
                    interactable.position = eye_pos + forward * 1.5;
                    interactable.velocity = forward * throw_force + Vec3::new(0.0, 3.0, 0.0);
                    interactable.is_physics_active = true;
                    interactable.is_grabbed = false;
                }
                self.player.grabbed_object = None;
            } else {
                let eye_pos = self.player.get_eye_position();
                let forward = self.player.transform.forward();

                if let Some((idx, _)) = self.collision_system.raycast_interactable(
                    eye_pos,
                    forward,
                    5.0,
                    &self.world.interactables,
                ) {
                    let interactable = &mut self.world.interactables[idx];
                    interactable.is_grabbed = true;
                    interactable.is_physics_active = false;
                    interactable.velocity = Vec3::ZERO;

                    self.player.grabbed_object = Some(GrabbedObject {
                        size: interactable.size,
                        color: interactable.color,
                    });
                }
            }
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
        crate::player::ui::render_grabbed_object(&self.player, self.ui.show_menu);
        set_default_camera();

        crate::player::ui::render_debug_info(&self.player, &self.movement_system, self.ui.show_debug);
        crate::player::ui::render_crosshair(self.ui.show_menu);

        if self.ui.show_menu {
            self.ui.render_menu(&self.player, &self.config);
        }
    }
}