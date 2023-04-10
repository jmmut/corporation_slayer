use crate::commands::{Commands, SideMovement};
use macroquad::prelude::*;

const SPEED: f32 = 0.001;
const TUNNEL_HALF_WIDTH: f32 = 1.5;

pub struct World {
    pub player_pos: Vec3,
}

impl World {
    pub fn update(&mut self, commands: Commands) {
        match commands.side_movement {
            SideMovement::None => {}
            SideMovement::Right => {
                self.player_pos.z = TUNNEL_HALF_WIDTH.min(self.player_pos.z + SPEED);
            }
            SideMovement::Left => {
                self.player_pos.z = (-TUNNEL_HALF_WIDTH).max(self.player_pos.z - SPEED);
            }
        }
    }
}
