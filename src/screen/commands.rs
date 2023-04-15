use crate::common::TimestampSeconds;
use crate::screen::gui;
use crate::screen::gui::move_button;
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

#[derive(PartialEq, Debug)]
pub enum Movement {
    None,
    Positive,
    Negative,
}

pub fn get_commands() -> Commands {
    let (forward_movement, left_movement) = get_forward_and_left_movement();
    Commands {
        should_quit: is_key_pressed(KeyCode::Escape),
        forward_movement,
        left_movement,
        jump: get_jump(),
        ts_now: now(),
        pissing: get_pissing(),
    }
}

fn get_jump() -> bool {
    is_key_pressed(KeyCode::Space) || gui::jump_button::is_clicked()
}
fn get_pissing() -> bool {
    is_key_down(KeyCode::Enter) || gui::piss_button::is_clicked()
}

fn get_forward_and_left_movement() -> (Movement, Movement) {
    let forward_movement = get_forward_movement();
    let left_movement = get_left_movement();
    if forward_movement != Movement::None || left_movement != Movement::None {
        return (forward_movement, left_movement);
    }
    if let Some(pos) = move_button::get_movement() {
        let forward = eight_direction_movement(pos.y, pos.x);
        let left = eight_direction_movement(pos.x, pos.y);
        (forward, left)
    } else {
        (Movement::None, Movement::None)
    }
}

fn eight_direction_movement(main_direction: f32, other_direction: f32) -> Movement {
    if main_direction.abs() > other_direction.abs() {
        Movement::from_value(main_direction)
    } else {
        if 2.0 * main_direction.abs() > other_direction.abs() {
            Movement::from_value(main_direction)
        } else {
            Movement::None
        }
    }
}

fn get_left_movement() -> Movement {
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

impl Movement {
    pub fn from_value(value: f32) -> Movement {
        if value == 0.0 {
            Movement::None
        } else if value > 0.0 {
            Movement::Positive
        } else {
            Movement::Negative
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eight_direction_main() {
        let moving = eight_direction_movement(0.5, 0.2);
        assert_eq!(moving, Movement::Positive)
    }

    #[test]
    fn test_eight_direction_secondary() {
        let moving = eight_direction_movement(0.3, 0.5);
        assert_eq!(moving, Movement::Positive)
    }
    #[test]
    fn test_eight_direction_none() {
        let moving = eight_direction_movement(0.2, 0.5);
        assert_eq!(moving, Movement::None)
    }
}
