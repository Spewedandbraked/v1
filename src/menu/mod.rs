use macroquad::prelude::*;
use macroquad::color::colors;
use crate::input::{InputState, InputConfig, Action};
use crate::player::Player;

const FONT_SIZE_TITLE: f32 = 40.0;
const FONT_SIZE_SECTION: f32 = 30.0;
const FONT_SIZE_CONTROL: f32 = 25.0;
const FONT_SIZE_HINT: f32 = 20.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuSection {
    Controls,
    Graphics,
    Audio,
}

impl MenuSection {
    pub fn all() -> &'static [MenuSection] {
        &[MenuSection::Controls, MenuSection::Graphics, MenuSection::Audio]
    }

    pub fn display_name(self) -> &'static str {
        match self {
            MenuSection::Controls => "Controls",
            MenuSection::Graphics => "Graphics",
            MenuSection::Audio => "Audio",
        }
    }
}

pub struct GameUI {
    pub show_menu: bool,
    pub rebinding_action: Option<Action>,
    pub show_debug: bool,
    pub current_section: MenuSection,
    slider_dragging: bool,
}

impl GameUI {
    pub fn new() -> Self {
        Self {
            show_menu: false,
            rebinding_action: None,
            show_debug: false,
            current_section: MenuSection::Controls,
            slider_dragging: false,
        }
    }

    pub fn toggle_menu(&mut self, input: &mut InputState) {
        self.show_menu = !self.show_menu;
        if self.show_menu {
            input.cursor_captured = false;
            set_cursor_grab(false);
            show_mouse(true);
            self.current_section = MenuSection::Controls;
        } else {
            input.cursor_captured = true;
            set_cursor_grab(true);
            show_mouse(false);
            self.rebinding_action = None;
            self.slider_dragging = false;
        }
    }

    pub fn toggle_debug(&mut self) {
        self.show_debug = !self.show_debug;
    }

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

    pub fn handle_menu_click(&mut self, config: &mut InputConfig, player: &mut Player) {
        if !self.show_menu || self.rebinding_action.is_some() {
            return;
        }

        let mouse_pos = mouse_position();
        let center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);

        if is_mouse_button_pressed(MouseButton::Left) {
            let section_y = center.y - 220.0;
            let section_width = 120.0;
            let section_spacing = 20.0;
            let total_width = MenuSection::all().len() as f32 * section_width
                + (MenuSection::all().len() as f32 - 1.0) * section_spacing;
            let start_x = center.x - total_width * 0.5;

            for (i, section) in MenuSection::all().iter().enumerate() {
                let x = start_x + i as f32 * (section_width + section_spacing);
                if mouse_pos.0 >= x && mouse_pos.0 <= x + section_width
                    && mouse_pos.1 >= section_y && mouse_pos.1 <= section_y + 40.0
                {
                    self.current_section = *section;
                    break;
                }
            }

            match self.current_section {
                MenuSection::Controls => {
                    let mut y = center.y - 120.0;
                    for action in Action::all() {
                        let text = format!("{}: ...", action.display_name());
                        let size = measure_text(&text, None, FONT_SIZE_CONTROL as u16, 1.0);
                        let text_x = center.x - size.width * 0.5;
                        if mouse_pos.0 >= text_x && mouse_pos.0 <= text_x + size.width
                            && mouse_pos.1 >= y - size.height && mouse_pos.1 <= y + size.height
                        {
                            self.rebinding_action = Some(*action);
                            break;
                        }
                        y += 35.0;
                    }

                    let reset_text = "Reset to Defaults";
                    let reset_size = measure_text(reset_text, None, FONT_SIZE_CONTROL as u16, 1.0);
                    let reset_x = center.x - reset_size.width * 0.5;
                    let reset_y = center.y + 180.0;
                    if mouse_pos.0 >= reset_x && mouse_pos.0 <= reset_x + reset_size.width
                        && mouse_pos.1 >= reset_y - reset_size.height
                        && mouse_pos.1 <= reset_y + reset_size.height
                    {
                        config.reset_to_defaults();
                    }
                }
                MenuSection::Graphics => {
                    let slider_y = center.y;
                    let slider_width = 300.0;
                    let slider_x = center.x - slider_width * 0.5;
                    let handle_radius = 12.0;
                    let sensitivity = player.camera.sensitivity;
                    let normalized = (sensitivity - 0.1) / 1.9;
                    let handle_x = slider_x + normalized * slider_width;
                    let dist = ((mouse_pos.0 - handle_x).powi(2) + (mouse_pos.1 - slider_y).powi(2)).sqrt();
                    if dist <= handle_radius {
                        self.slider_dragging = true;
                    }
                }
                MenuSection::Audio => {}
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.slider_dragging = false;
        }

        if self.slider_dragging {
            let slider_width = 300.0;
            let slider_x = center.x - slider_width * 0.5;
            let mouse_x = mouse_pos.0.clamp(slider_x, slider_x + slider_width);
            let normalized = (mouse_x - slider_x) / slider_width;
            player.camera.sensitivity = 0.1 + normalized * 1.9;
        }
    }

    pub fn is_rebinding(&self) -> bool {
        self.rebinding_action.is_some()
    }

    pub fn render_menu(&self, player: &Player, config: &InputConfig) {
        let center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(0, 0, 0, 200));

        let title = "SETTINGS";
        let title_size = measure_text(title, None, FONT_SIZE_TITLE as u16, 1.0);
        draw_text(title, center.x - title_size.width * 0.5, center.y - 280.0, FONT_SIZE_TITLE, colors::WHITE);

        let section_y = center.y - 220.0;
        let section_width = 120.0;
        let section_spacing = 20.0;
        let total_width = MenuSection::all().len() as f32 * section_width
            + (MenuSection::all().len() as f32 - 1.0) * section_spacing;
        let start_x = center.x - total_width * 0.5;

        for (i, section) in MenuSection::all().iter().enumerate() {
            let x = start_x + i as f32 * (section_width + section_spacing);
            let is_active = self.current_section == *section;
            let bg = if is_active { Color::from_rgba(80, 80, 120, 255) } else { Color::from_rgba(40, 40, 60, 255) };
            draw_rectangle(x, section_y, section_width, 40.0, bg);
            let text = section.display_name();
            let text_size = measure_text(text, None, FONT_SIZE_SECTION as u16, 1.0);
            draw_text(
                text,
                x + (section_width - text_size.width) * 0.5,
                section_y + 28.0,
                FONT_SIZE_SECTION,
                if is_active { colors::WHITE } else { Color::from_rgba(180, 180, 180, 255) },
            );
        }

        draw_line(start_x, section_y + 45.0, start_x + total_width, section_y + 45.0, 2.0, Color::from_rgba(100, 100, 100, 255));

        match self.current_section {
            MenuSection::Controls => {
                let mut y = center.y - 120.0;
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
                        if self.rebinding_action == Some(*action) { colors::GOLD } else { Color::from_rgba(200, 200, 200, 255) },
                    );
                    y += 35.0;
                }
                let reset_text = "Reset to Defaults";
                let reset_size = measure_text(reset_text, None, FONT_SIZE_CONTROL as u16, 1.0);
                draw_text(reset_text, center.x - reset_size.width * 0.5, center.y + 180.0, FONT_SIZE_CONTROL, colors::WHITE);
            }
            MenuSection::Graphics => {
                let slider_y = center.y;
                let slider_width = 300.0;
                let slider_x = center.x - slider_width * 0.5;
                let slider_height = 6.0;

                draw_rectangle(slider_x, slider_y - slider_height * 0.5, slider_width, slider_height, Color::from_rgba(60, 60, 80, 255));
                let sensitivity = player.camera.sensitivity;
                let normalized = (sensitivity - 0.1) / 1.9;
                let fill_width = normalized * slider_width;
                draw_rectangle(slider_x, slider_y - slider_height * 0.5, fill_width, slider_height, Color::from_rgba(100, 150, 255, 255));

                let handle_radius = 10.0;
                let handle_x = slider_x + normalized * slider_width;
                draw_circle(handle_x, slider_y, handle_radius, colors::WHITE);
                draw_circle_lines(handle_x, slider_y, handle_radius, 2.0, Color::from_rgba(150, 150, 150, 255));

                let sens_label = "Mouse Sensitivity";
                let sens_label_size = measure_text(sens_label, None, FONT_SIZE_CONTROL as u16, 1.0);
                draw_text(sens_label, center.x - sens_label_size.width * 0.5, center.y - 40.0, FONT_SIZE_CONTROL, colors::WHITE);

                let value_text = format!("{:.2}", sensitivity);
                draw_text(&value_text, slider_x + slider_width + 20.0, slider_y + 8.0, FONT_SIZE_CONTROL, colors::WHITE);
            }
            MenuSection::Audio => {
                let msg = "Audio settings coming soon...";
                let msg_size = measure_text(msg, None, FONT_SIZE_CONTROL as u16, 1.0);
                draw_text(msg, center.x - msg_size.width * 0.5, center.y, FONT_SIZE_CONTROL, Color::from_rgba(150, 150, 150, 255));
            }
        }

        let hint = "Press ESC to close";
        let hint_size = measure_text(hint, None, FONT_SIZE_HINT as u16, 1.0);
        draw_text(hint, center.x - hint_size.width * 0.5, screen_height() - 50.0, FONT_SIZE_HINT, Color::from_rgba(150, 150, 150, 255));
    }
}