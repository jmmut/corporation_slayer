use crate::common::TimestampSeconds;
use crate::screen::commands::{Commands, Movement};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use std::f32::consts::SQRT_2;

const SPEED: f32 = 10.0;
const TUNNEL_HALF_WIDTH: f32 = 1.5;
const JUMP_DURATION: f64 = 0.3;
pub const PLAYER_HEIGHT: f32 = 1.75;

pub struct World {
    pub player_pos: Vec3,
    pub jump_started: TimestampSeconds,
    pub obstacles: Vec<Vec3>,
    pub previous_frame_ts: TimestampSeconds,
    pub colliding: bool,
    pub health: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            health: 1.0,
            player_pos: Vec3::new(0.0, 0.0, 0.0),
            jump_started: 0.0,
            obstacles: generate_obstacles(),
            previous_frame_ts: now(),
            colliding: false,
        }
    }

    pub fn update(&mut self, commands: Commands) {
        if self.health > 0.0 {
            self.update_side_movement(&commands);
            self.update_jumped(&commands);
            self.update_collision();
            self.update_health(&commands);
            self.update_time(&commands);
        }
    }

    fn update_side_movement(&mut self, commands: &Commands) {
        let dt = (commands.ts_now - self.previous_frame_ts) as f32;
        let mut dz = match commands.left_movement {
            Movement::None => 0.0,
            Movement::Positive => -SPEED * dt,
            Movement::Negative => SPEED * dt,
        };
        let mut dx = match commands.forward_movement {
            Movement::None => 0.0,
            Movement::Positive => SPEED * dt,
            Movement::Negative => -SPEED * dt,
        };
        if dx != 0.0 && dz != 0.0 {
            dx = dx / SQRT_2;
            dz = dz / SQRT_2;
        }
        self.player_pos.x = self.player_pos.x + dx;
        self.player_pos.z = (self.player_pos.z + dz).clamp(-TUNNEL_HALF_WIDTH, TUNNEL_HALF_WIDTH);
    }
    fn update_jumped(&mut self, commands: &Commands) {
        let now_ts = commands.ts_now;
        let jump_time = commands.ts_now - self.jump_started;
        let jumping = jump_time < JUMP_DURATION;
        if commands.jump && !jumping {
            self.jump_started = now_ts;
        }
        if jumping {
            let height: f64 = 1.5;
            let offset = JUMP_DURATION * 0.5;
            // let jump_speed = (height/offset).sqrt();
            let jump_speed = 1.0;
            let height_coef = height / (offset * offset);
            let x = jump_time * jump_speed - offset;
            let y = height_coef * x * x;
            self.player_pos.y = (height - y) as f32;
        } else {
            self.player_pos.y = 0.0
        }
    }
    fn update_collision(&mut self) {
        for obstacle in &self.obstacles {
            if collides(self.player_pos, *obstacle) {
                self.colliding = true;
                return;
            }
        }
        self.colliding = false;
    }

    fn update_time(&mut self, commands: &Commands) {
        self.previous_frame_ts = commands.ts_now;
    }
    fn update_health(&mut self, commands: &Commands) {
        let pain_speed = 1.0;
        if self.colliding {
            let dt = commands.ts_now - self.previous_frame_ts;
            self.health -= (dt * pain_speed) as f32;
            self.health = self.health.clamp(0.0, 1.0);
        }
    }
}

pub fn generate_obstacles() -> Vec<Vec3> {
    const NUM_OBSTACLES: usize = 10;
    const LANES: i32 = 4;
    let mut obstacles = Vec::with_capacity(NUM_OBSTACLES as usize);
    let mut depth = 2.0;
    rand::srand(unsafe { now().floor().to_int_unchecked() });
    loop {
        for i_lane in 0..LANES {
            let sample = rand::gen_range(0, 100);
            if sample < 10 {
                obstacles.push(Vec3::new(depth, 0.0, i_lane as f32 - 1.5));
                if obstacles.len() == NUM_OBSTACLES {
                    return obstacles;
                }
            }
        }
        depth += 1.0;
    }
}

fn collides(player_pos: Vec3, obstacle_pos: Vec3) -> bool {
    let obstacle_radius = 0.4;
    let player_radius = 0.5;
    let dx = player_pos.x - obstacle_pos.x;
    let dy = player_pos.y - obstacle_pos.y;
    let dz = player_pos.z - obstacle_pos.z;
    let squared_distance = dx * dx + dy * dy + dz * dz;
    let radius = obstacle_radius + player_radius;
    let squared_min_distance = radius * radius;
    squared_distance < squared_min_distance
}
