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

pub fn render_hud(player: &Player, show_menu: bool) {
    if show_menu {
        return;
    }

    let screen_w = screen_width();
    let screen_h = screen_height();
    
    // === Здоровье (слева-внизу) ===
    let hud_x = 20.0;
    let bar_width = 250.0;
    let bar_height = 20.0;
    let bar_y = screen_h - 60.0;
    
    // Подложка
    draw_rectangle(hud_x, bar_y, bar_width, bar_height, Color::from_rgba(0, 0, 0, 180));
    draw_rectangle_lines(hud_x, bar_y, bar_width, bar_height, 2.0, Color::from_rgba(255, 255, 255, 100));
    
    // Полоска здоровья
    let health_ratio = player.stats.health / player.stats.max_health;
    let health_width = bar_width * health_ratio;
    let health_color = if health_ratio > 0.5 {
        Color::from_rgba(50, 200, 50, 255)
    } else if health_ratio > 0.25 {
        Color::from_rgba(255, 200, 50, 255)
    } else {
        Color::from_rgba(255, 50, 50, 255)
    };
    draw_rectangle(hud_x, bar_y, health_width, bar_height, health_color);
    
    // Текст здоровья
    let health_text = format!("HP: {:.0} / {:.0}", player.stats.health, player.stats.max_health);
    let text_size = measure_text(&health_text, None, 24, 1.0);
    draw_text(
        &health_text,
        hud_x + (bar_width - text_size.width) * 0.5,
        bar_y - 5.0,
        24.0,
        colors::WHITE,
    );
    
    // Иконка сердца
    draw_text("❤", hud_x - 5.0, bar_y - 3.0, 28.0, Color::from_rgba(255, 50, 50, 255));
    
    // === Выносливость (под здоровьем) ===
    let stamina_y = bar_y + bar_height + 5.0;
    let stamina_height = 12.0;
    
    // Подложка
    draw_rectangle(hud_x, stamina_y, bar_width, stamina_height, Color::from_rgba(0, 0, 0, 180));
    draw_rectangle_lines(hud_x, stamina_y, bar_width, stamina_height, 1.0, Color::from_rgba(255, 255, 255, 80));
    
    // Полоска выносливости
    let stamina_ratio = player.stats.stamina / player.stats.max_stamina;
    let stamina_width = bar_width * stamina_ratio;
    let stamina_color = if stamina_ratio > 0.5 {
        Color::from_rgba(50, 150, 255, 255)
    } else if stamina_ratio > 0.25 {
        Color::from_rgba(255, 200, 50, 255)
    } else {
        Color::from_rgba(255, 80, 50, 255)
    };
    draw_rectangle(hud_x, stamina_y, stamina_width, stamina_height, stamina_color);
    
    // Текст выносливости
    let stamina_text = format!("ST: {:.0} / {:.0}", player.stats.stamina, player.stats.max_stamina);
    let text_size = measure_text(&stamina_text, None, 16, 1.0);
    draw_text(
        &stamina_text,
        hud_x + (bar_width - text_size.width) * 0.5,
        stamina_y - 3.0,
        16.0,
        colors::WHITE,
    );
    
    // Миникарта (справа-вверху)
    let map_size = 100.0;
    let map_x = screen_w - map_size - 20.0;
    let map_y = 20.0;
    draw_rectangle(map_x, map_y, map_size, map_size, Color::from_rgba(0, 0, 0, 150));
    draw_rectangle_lines(map_x, map_y, map_size, map_size, 2.0, Color::from_rgba(255, 255, 255, 100));
    draw_circle(map_x + map_size * 0.5, map_y + map_size * 0.5, 3.0, colors::WHITE);
}