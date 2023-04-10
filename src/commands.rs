use macroquad::prelude::*;

pub struct Commands {
    pub should_quit: bool,
    pub side_movement: SideMovement,
    pub jump: bool,
}

pub enum SideMovement {
    None,
    Right,
    Left,
}

pub fn get_commands() -> Commands {
    Commands {
        should_quit: is_key_pressed(KeyCode::Escape),
        side_movement: get_side_movement(),
        jump: get_jump(),
    }
}

fn get_jump() -> bool {
    is_key_down(KeyCode::Space) || is_key_down(KeyCode::Up)
}

fn get_side_movement() -> SideMovement {
    let left = is_key_down(KeyCode::Left);
    let right = is_key_down(KeyCode::Right);
    if left && right {
        SideMovement::None
    } else if left {
        SideMovement::Left
    } else if right {
        SideMovement::Right
    } else {
        SideMovement::None
    }
}
