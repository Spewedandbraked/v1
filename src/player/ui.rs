use macroquad::prelude::*;
use macroquad::color::colors;
use crate::player::Player;
use crate::player::movement::MovementSystem;

const FONT_SIZE_DEBUG: f32 = 20.0;

pub fn render_debug_info(player: &Player, movement: &MovementSystem, show_debug: bool) {
    if !show_debug {
        return;
    }

    let mut y = 30.0;
    let line_height = 25.0;

    draw_text(&format!("FPS: {:.0}", get_fps()), 10.0, y, FONT_SIZE_DEBUG, colors::WHITE);
    y += line_height;

    draw_text(
        &format!(
            "Position: {:.2}, {:.2}, {:.2}",
            player.transform.position.x,
            player.transform.position.y,
            player.transform.position.z
        ),
        10.0,
        y,
        FONT_SIZE_DEBUG,
        colors::WHITE,
    );
    y += line_height;

    draw_text(
        &format!(
            "Velocity: {:.2}, {:.2}, {:.2}",
            movement.velocity.x, movement.velocity.y, movement.velocity.z
        ),
        10.0,
        y,
        FONT_SIZE_DEBUG,
        colors::WHITE,
    );
    y += line_height;

    draw_text(
        &format!("Grounded: {}", movement.is_grounded),
        10.0,
        y,
        FONT_SIZE_DEBUG,
        colors::WHITE,
    );
}

pub fn render_crosshair(show_menu: bool) {
    if show_menu {
        return;
    }

    let center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);
    let size = 10.0;
    let thickness = 2.0;

    draw_line(center.x - size, center.y, center.x + size, center.y, thickness, colors::WHITE);
    draw_line(center.x, center.y - size, center.x, center.y + size, thickness, colors::WHITE);
    draw_circle(center.x, center.y, 2.0, colors::WHITE);
}