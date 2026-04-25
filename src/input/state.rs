use macroquad::prelude::*;
use crate::input::config::{Action, InputConfig};

#[derive(Debug, Default)]
pub struct InputState {
    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,
    pub sprint: bool,
    pub jump: bool,
    pub mouse_delta: Vec2,
    pub cursor_captured: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            cursor_captured: true,
            ..Default::default()
        }
    }

    pub fn update(&mut self, config: &InputConfig) {
        self.move_forward = config.is_action_pressed(Action::MoveForward);
        self.move_backward = config.is_action_pressed(Action::MoveBackward);
        self.move_left = config.is_action_pressed(Action::MoveLeft);
        self.move_right = config.is_action_pressed(Action::MoveRight);
        self.move_up = config.is_action_pressed(Action::MoveUp);
        self.move_down = config.is_action_pressed(Action::MoveDown);
        self.sprint = config.is_action_pressed(Action::Sprint);
        self.jump = config.is_action_just_pressed(Action::Jump);

        self.mouse_delta = mouse_delta_position();

        set_cursor_grab(self.cursor_captured);
        show_mouse(!self.cursor_captured);
    }
}