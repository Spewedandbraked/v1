use macroquad::prelude::*;
use macroquad::color::colors;
use crate::entities::Player;
use crate::systems::MovementSystem;
use crate::input::{InputState, InputConfig, Action};

const FONT_SIZE_TITLE: f32 = 40.0;
const FONT_SIZE_CONTROL: f32 = 25.0;
const FONT_SIZE_HINT: f32 = 20.0;
const FONT_SIZE_DEBUG: f32 = 20.0;

pub struct GameUI {
    pub show_menu: bool,
    pub rebinding_action: Option<Action>,
    pub show_debug: bool,
}

impl GameUI {
    /// Создаёт UI с выключенными меню, ребиндом и дебаг-режимом.
    pub fn new() -> Self {
        Self {
            show_menu: false,
            rebinding_action: None,
            show_debug: false,
        }
    }

    /// Переключает экран меню и синхронизирует состояние курсора.
    pub fn toggle_menu(&mut self, input: &mut InputState) {
        self.show_menu = !self.show_menu;
        if self.show_menu {
            input.cursor_captured = false;
            set_cursor_grab(false);
            show_mouse(true);
        } else {
            input.cursor_captured = true;
            set_cursor_grab(true);
            show_mouse(false);
            self.rebinding_action = None;
        }
    }

    /// Включает или выключает показ отладочной информации.
    pub fn toggle_debug(&mut self) {
        self.show_debug = !self.show_debug;
    }

    /// Ожидает нажатие клавиши для переназначения выбранного действия.
    pub fn update_rebinding(&mut self, config: &mut InputConfig) {
        if let Some(action) = self.rebinding_action {
            if let Some(key) = get_last_key_pressed() {
                if key == KeyCode::Escape {
                    self.rebinding_action = None;
                    return;
                }
                config.set_key(action, key);
                self.rebinding_action = None;
            }
        }
    }

    /// Обрабатывает клики по пунктам меню (ребинд и сброс биндов).
    pub fn handle_menu_click(&mut self, config: &mut InputConfig) {
        if !self.show_menu || self.rebinding_action.is_some() {
            return;
        }

        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }

        let mouse_pos = mouse_position();
        let center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);
        let mut y = center.y - 180.0;

        for action in Action::all() {
            let text = format!("{}: ...", action.display_name());
            let size = measure_text(&text, None, FONT_SIZE_CONTROL as u16, 1.0);
            let text_x = center.x - size.width * 0.5;

            if mouse_pos.0 >= text_x
                && mouse_pos.0 <= text_x + size.width
                && mouse_pos.1 >= y - size.height
                && mouse_pos.1 <= y + size.height
            {
                self.rebinding_action = Some(*action);
                break;
            }
            y += 35.0;
        }

        let reset_text = "Reset to Defaults";
        let reset_size = measure_text(reset_text, None, FONT_SIZE_CONTROL as u16, 1.0);
        let reset_x = center.x - reset_size.width * 0.5;
        let reset_y = center.y + 200.0;

        if mouse_pos.0 >= reset_x
            && mouse_pos.0 <= reset_x + reset_size.width
            && mouse_pos.1 >= reset_y - reset_size.height
            && mouse_pos.1 <= reset_y + reset_size.height
        {
            config.reset_to_defaults();
        }
    }

    /// Возвращает признак того, что UI находится в режиме ребинда.
    pub fn is_rebinding(&self) -> bool {
        self.rebinding_action.is_some()
    }

    /// Рисует отладочный оверлей с FPS, позицией и скоростью игрока.
    pub fn render_debug_info(
        &self,
        player: &Player,
        movement: &MovementSystem,
    ) {
        if !self.show_debug {
            return;
        }

        let mut y = 30.0;
        let line_height = 25.0;

        let fps_text = format!("FPS: {:.0}", get_fps());
        draw_text(&fps_text, 10.0, y, FONT_SIZE_DEBUG, colors::WHITE);
        y += line_height;

        let pos_text = format!(
            "Position: {:.2}, {:.2}, {:.2}",
            player.transform.position.x,
            player.transform.position.y,
            player.transform.position.z
        );
        draw_text(&pos_text, 10.0, y, FONT_SIZE_DEBUG, colors::WHITE);
        y += line_height;

        let vel_text = format!(
            "Velocity: {:.2}, {:.2}, {:.2}",
            movement.velocity.x,
            movement.velocity.y,
            movement.velocity.z
        );
        draw_text(&vel_text, 10.0, y, FONT_SIZE_DEBUG, colors::WHITE);
        y += line_height;

        let grounded_text = format!("Grounded: {}", movement.is_grounded);
        draw_text(&grounded_text, 10.0, y, FONT_SIZE_DEBUG, colors::WHITE);
    }

    /// Отрисовывает прицел в центре экрана, когда меню закрыто.
    pub fn render_crosshair(&self) {
        if self.show_menu {
            return;
        }

        let screen_center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);
        let crosshair_size = 10.0;
        let thickness = 2.0;

        draw_line(
            screen_center.x - crosshair_size,
            screen_center.y,
            screen_center.x + crosshair_size,
            screen_center.y,
            thickness,
            colors::WHITE,
        );

        draw_line(
            screen_center.x,
            screen_center.y - crosshair_size,
            screen_center.x,
            screen_center.y + crosshair_size,
            thickness,
            colors::WHITE,
        );

        draw_circle(screen_center.x, screen_center.y, 2.0, colors::WHITE);
    }

    /// Рисует экран меню управления и подсказки по переназначению клавиш.
    pub fn render_menu(&self, player: &Player, config: &InputConfig) {
        let center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);

        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(0, 0, 0, 200));

        let title = "CONTROLS";
        let title_size = measure_text(title, None, FONT_SIZE_TITLE as u16, 1.0);
        draw_text(
            title,
            center.x - title_size.width * 0.5,
            center.y - 250.0,
            FONT_SIZE_TITLE,
            colors::WHITE,
        );

        let mut y = center.y - 180.0;

        for action in Action::all() {
            let key = config.get_key(*action);
            let text = if let Some(rebinding) = self.rebinding_action {
                if rebinding == *action {
                    format!("{}: Press any key...", action.display_name())
                } else {
                    format!("{}: {:?}", action.display_name(), key)
                }
            } else {
                format!("{}: {:?}", action.display_name(), key)
            };

            let size = measure_text(&text, None, FONT_SIZE_CONTROL as u16, 1.0);
            draw_text(
                &text,
                center.x - size.width * 0.5,
                y,
                FONT_SIZE_CONTROL,
                if self.rebinding_action == Some(*action) {
                    colors::GOLD
                } else {
                    Color::from_rgba(200, 200, 200, 255)
                },
            );
            y += 35.0;
        }

        y += 20.0;
        let sens_text = format!("Current Sensitivity: {:.2}", player.camera.sensitivity);
        let sens_size = measure_text(&sens_text, None, FONT_SIZE_CONTROL as u16, 1.0);
        draw_text(
            &sens_text,
            center.x - sens_size.width * 0.5,
            y,
            FONT_SIZE_CONTROL,
            colors::GOLD,
        );

        let reset_text = "Reset to Defaults";
        let reset_size = measure_text(reset_text, None, FONT_SIZE_CONTROL as u16, 1.0);
        let reset_y = center.y + 200.0;
        draw_text(
            reset_text,
            center.x - reset_size.width * 0.5,
            reset_y,
            FONT_SIZE_CONTROL,
            colors::WHITE,
        );

        let hint = "Click on action to rebind | Tab to close";
        let hint_size = measure_text(hint, None, FONT_SIZE_HINT as u16, 1.0);
        draw_text(
            hint,
            center.x - hint_size.width * 0.5,
            screen_height() - 50.0,
            FONT_SIZE_HINT,
            Color::from_rgba(150, 150, 150, 255),
        );
    }
}