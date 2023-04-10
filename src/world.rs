use crate::commands::{Commands, SideMovement};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;

type TimestampMs = f64;
const SPEED: f32 = 0.001;
const TUNNEL_HALF_WIDTH: f32 = 1.5;
const JUMP_DURATION: f64 = 1.0;
const PLAYER_HALF_HEIGHT: f64 = 1.0;

pub struct World {
    pub player_pos: Vec3,
    pub jump_started: TimestampMs,
}

impl World {
    pub fn update(&mut self, commands: Commands) {
        self.update_side_movement(&commands);
        self.update_jumped(&commands);
    }

    fn update_side_movement(&mut self, commands: &Commands) {
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
    fn update_jumped(&mut self, commands: &Commands) {
        let now_ts = now();
        if commands.jump {
            self.jump_started = now_ts;
        }
        let jump_time = now_ts - self.jump_started;
        if jump_time < JUMP_DURATION {
            let height: f64 = 1.0;
            let offset = JUMP_DURATION * 0.5;
            // let jump_speed = (height/offset).sqrt();
            let jump_speed = 1.0;
            let height_coef = height / (offset * offset);
            let x = jump_time * jump_speed - offset;
            let y = height_coef * x * x;
            self.player_pos.y = (PLAYER_HALF_HEIGHT + height - y) as f32;
        } else {
            self.player_pos.y = PLAYER_HALF_HEIGHT as f32;
        }
    }
}
