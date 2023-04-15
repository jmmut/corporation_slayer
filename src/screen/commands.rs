use crate::common::TimestampSeconds;
use crate::screen::gui;
use macroquad::miniquad::date::now;
use macroquad::prelude::*;

pub struct Commands {
    pub should_quit: bool,
    pub forward_movement: Movement,
    pub left_movement: Movement,
    pub jump: bool,
    pub ts_now: TimestampSeconds,
    pub pissing: bool,
}

pub enum Movement {
    None,
    Positive,
    Negative,
}

pub fn get_commands() -> Commands {
    Commands {
        should_quit: is_key_pressed(KeyCode::Escape),
        forward_movement: get_forward_movement(),
        left_movement: get_side_movement(),
        jump: get_jump(),
        ts_now: now(),
        pissing: get_pissing(),
    }
}

fn get_jump() -> bool {
    is_key_pressed(KeyCode::Space) || gui::jump_button::is_clicked()
}
fn get_pissing() -> bool {
    is_key_down(KeyCode::Enter)
}

fn get_side_movement() -> Movement {
    get_cancellable_movement(
        is_key_down(KeyCode::Left) || is_key_down(KeyCode::A),
        is_key_down(KeyCode::Right) || is_key_down(KeyCode::D),
    )
}

fn get_forward_movement() -> Movement {
    get_cancellable_movement(
        is_key_down(KeyCode::Up) || is_key_down(KeyCode::W),
        is_key_down(KeyCode::Down) || is_key_down(KeyCode::S),
    )
}

fn get_cancellable_movement(positive: bool, negative: bool) -> Movement {
    if positive && negative {
        Movement::None
    } else if positive {
        Movement::Positive
    } else if negative {
        Movement::Negative
    } else {
        Movement::None
    }
}
