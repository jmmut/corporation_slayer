use macroquad::miniquad::date::now;
use crate::screen::commands::{Commands, SideMovement};
use macroquad::prelude::*;
use crate::common::TimestampSeconds;

const SPEED: f32 = 10.0;
const TUNNEL_HALF_WIDTH: f32 = 1.5;
const JUMP_DURATION: f64 = 1.0;
const PLAYER_HALF_HEIGHT: f64 = 1.0;

pub struct World {
    pub player_pos: Vec3,
    pub jump_started: TimestampSeconds,
    pub obstacles: Vec<Vec3>,
    pub previous_frame_ts: TimestampSeconds,
}

impl World {
    pub fn update(&mut self, commands: Commands) {
        self.update_side_movement(&commands);
        self.update_jumped(&commands);
        self.update_time(&commands);
    }

    fn update_side_movement(&mut self, commands: &Commands) {
        let dt= (commands.ts_now - self.previous_frame_ts) as f32;
        match commands.side_movement {
            SideMovement::None => {}
            SideMovement::Right => {
                self.player_pos.z = TUNNEL_HALF_WIDTH.min(self.player_pos.z + SPEED*dt);
            }
            SideMovement::Left => {
                self.player_pos.z = (-TUNNEL_HALF_WIDTH).max(self.player_pos.z - SPEED*dt);
            }
        }
    }
    fn update_jumped(&mut self, commands: &Commands) {
        let now_ts = commands.ts_now;
        let jump_time = commands.ts_now  - self.jump_started;
        let jumping = jump_time < JUMP_DURATION;
        if commands.jump && !jumping {
            self.jump_started = now_ts;
        }
        if jumping {
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
    fn update_time(&mut self, commands: &Commands) {
        self.previous_frame_ts = commands.ts_now;
    }
}

pub fn generate_obstacles() -> Vec<Vec3> {
    const NUM_OBSTACLES: usize = 10;
    const LANES: i32 = 4;
    let mut obstacles = Vec::with_capacity(NUM_OBSTACLES as usize);
    let mut depth = 0.0;
    rand::srand( unsafe { now().floor().to_int_unchecked()});
    loop {
        for i_lane in 0..LANES {
            let sample = rand::gen_range(0, 100);
            if sample < 10 {
                obstacles.push(Vec3::new(depth, 0.5, i_lane as f32 - 1.5));
                if obstacles.len() == NUM_OBSTACLES {
                    return obstacles;
                }
            }
        }
        depth += 1.0;
    }
}
