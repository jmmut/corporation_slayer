use crate::screen::draw::FONT_SIZE;
use macroquad::prelude::{
    draw_rectangle, draw_text, measure_text, mouse_position_local, touches_local, Color, BLACK,
};
use macroquad::prelude::{is_mouse_button_down, screen_height, screen_width, MouseButton, Vec2};

const PRESSED_BUTTON_COLOR: Color = Color::new(0.1, 0.1, 0.1, 0.3);
const BUTTON_COLOR: Color = Color::new(0.2, 0.2, 0.2, 0.2);

pub fn draw_gui() {
    jump_button::draw();
    piss_button::draw();
}

pub mod jump_button {
    use super::*;

    static mut LAST_IS_CLICKED: bool = false;

    pub fn is_clicked() -> bool {
        let clicked = clicked_or_touched(inside_button);
        unsafe { LAST_IS_CLICKED = clicked };
        return clicked;
    }

    fn inside_button(position: Vec2) -> bool {
        position.x > 0.0 && position.y > 0.0 && position.y < 0.5
    }

    pub fn draw() {
        draw_button("JUMP", Vec2::new(0.5, 0.5), unsafe { LAST_IS_CLICKED });
    }
}

pub mod piss_button {
    use super::*;

    static mut LAST_IS_CLICKED: bool = false;

    pub fn is_clicked() -> bool {
        let clicked = clicked_or_touched(inside_button);
        unsafe { LAST_IS_CLICKED = clicked };
        return clicked;
    }

    fn inside_button(position: Vec2) -> bool {
        position.x > 0.0 && position.y > 0.5
    }

    pub fn draw() {
        draw_button("PISS", Vec2::new(0.5, 0.75), unsafe { LAST_IS_CLICKED });
    }
}

/// inside_button() will receive screen coordinates in the range of {x: [-1, 1], y: [-1, 1]}
fn clicked_or_touched<F: Fn(Vec2) -> bool>(inside_button: F) -> bool {
    if is_mouse_button_down(MouseButton::Left) && inside_button(mouse_position_local()) {
        return true;
    }
    for touch in touches_local() {
        if inside_button(touch.position) {
            return true;
        }
    }
    return false;
}

/// position is in the range of {x: [0, 1], y: [0, 1]}
pub fn draw_button(text: &str, position: Vec2, button_is_clicked: bool) {
    let screen_h = screen_height();
    let screen_w = screen_width();
    let padding = (0.05 * screen_w).min(0.05 * screen_h);
    let x = position.x * screen_w + padding;
    let y = position.y * screen_h + padding;
    let w = 0.4 * screen_w;
    let h = 0.15 * screen_h;
    let color = if button_is_clicked {
        PRESSED_BUTTON_COLOR
    } else {
        BUTTON_COLOR
    };
    draw_rectangle(x, y, w, h, color);
    let size = measure_text(text, None, FONT_SIZE as u16, 1.0);
    draw_text(
        text,
        x + (w - size.width) * 0.5,
        y + h * 0.5,
        FONT_SIZE,
        BLACK,
    );
}
