use crate::screen::draw::FONT_SIZE;
use macroquad::prelude::{
    draw_rectangle, draw_text, measure_text, mouse_position_local, touches_local, Color, BLACK,
};
use macroquad::prelude::{is_mouse_button_down, screen_height, screen_width, MouseButton, Vec2};

const PRESSED_BUTTON_COLOR: Color = Color::new(0.1, 0.1, 0.1, 0.3);
const BUTTON_COLOR: Color = Color::new(0.2, 0.2, 0.2, 0.2);

pub fn draw_gui() {
    jump_button::draw();
}

pub mod jump_button {
    use super::*;

    static mut LAST_IS_CLICKED: bool = false;

    pub fn is_clicked() -> bool {
        if is_mouse_button_down(MouseButton::Left) && inside_button(mouse_position_local()) {
            unsafe { LAST_IS_CLICKED = true };
            return true;
        }
        for touch in touches_local() {
            if inside_button(touch.position) {
                unsafe { LAST_IS_CLICKED = false };
                return true;
            }
        }
        unsafe { LAST_IS_CLICKED = false };
        return false;
    }

    fn inside_button(position: Vec2) -> bool {
        position.x > 0.0 && position.y > 0.0 && position.y < 0.5
    }

    pub fn draw() {
        let screen_h = screen_height();
        let screen_w = screen_width();
        let padding = (0.05 * screen_w).min(0.05 * screen_h);
        let x = 0.5 * screen_w + padding;
        let y = 0.5 * screen_h + padding;
        let w = 0.4 * screen_w;
        let h = 0.15 * screen_h;
        let color = if unsafe { LAST_IS_CLICKED } {
            PRESSED_BUTTON_COLOR
        } else {
            BUTTON_COLOR
        };
        draw_rectangle(x, y, w, h, color);
        let text = "JUMP";
        let size = measure_text(text, None, FONT_SIZE as u16, 1.0);
        draw_text(
            text,
            x + (w - size.width) * 0.5,
            y + h * 0.5,
            FONT_SIZE,
            BLACK,
        );
    }
}
