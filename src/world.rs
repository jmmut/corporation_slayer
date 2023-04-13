pub mod obstacles;

use std::collections::VecDeque;
use crate::common::TimestampSeconds;
use crate::screen::commands::{Commands, Movement};
use crate::world::obstacles::{generate_obstacles, Obstacles};
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
    pub obstacles: Obstacles,
    pub previous_frame_ts: TimestampSeconds,
    pub now_ts: TimestampSeconds,
    pub colliding: bool,
    pub pissing: bool,
    pub health: f32,
    pub piss: f32,
    pub level: i32,
    pub game_start: TimestampSeconds,
    pub game_end: Option<TimestampSeconds>,
    pub piss_particles: VecDeque<Particle>,
}

pub struct Particle {
    pub position: Vec3,
    started: TimestampSeconds,
}

impl World {
    pub fn new(level: i32) -> Self {
        let mut world = Self {
            health: 1.0,
            piss: 0.3,
            player_pos: Vec3::new(0.0, 0.0, 0.0),
            jump_started: 0.0,
            obstacles: Vec::new(),
            previous_frame_ts: now(),
            now_ts: now(),
            colliding: false,
            pissing: false,
            level,
            game_start: now(),
            game_end: None,
            piss_particles: VecDeque::new(),
        };
        world.regenerate();
        world
    }

    pub fn update(&mut self, commands: Commands) {
        if self.health > 0.0 {
            self.update_time(&commands);
            self.update_player_position(&commands);
            self.update_jumped(&commands);
            self.update_collision();
            self.update_health(&commands);
            self.update_pissing(&commands);
        }
    }

    pub fn regenerate(&mut self) {
        self.obstacles = generate_obstacles(self.level, get_random_seed());
        self.player_pos = Vec3::new(0.0, 0.0, 0.0);
        self.jump_started = 0.0;
        self.piss_particles = VecDeque::new();
    }

    fn update_player_position(&mut self, commands: &Commands) {
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
        self.player_pos.x = (self.player_pos.x + dx).max(0.0);
        self.player_pos.z = (self.player_pos.z + dz).clamp(-TUNNEL_HALF_WIDTH, TUNNEL_HALF_WIDTH);
    }

    fn update_jumped(&mut self, commands: &Commands) {
        let jump_time = commands.ts_now - self.jump_started;
        let jumping = jump_time < JUMP_DURATION;
        if commands.jump && !jumping {
            self.jump_started = self.now_ts;
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
            if collides(self.player_pos, obstacle.get_pos(self.previous_frame_ts)) {
                self.colliding = true;
                return;
            }
        }
        self.colliding = false;
    }

    fn update_health(&mut self, commands: &Commands) {
        let pain_speed = 1.0;
        if self.colliding {
            let dt = commands.ts_now - self.previous_frame_ts;
            self.health -= (dt * pain_speed) as f32;
            self.health = self.health.clamp(0.0, 1.0);
            if self.health == 0.0 {
                self.game_end = Some(commands.ts_now);
            }
        }
    }

    fn update_pissing(&mut self, commands: &Commands) {
        // is the player pissing?
        self.pissing = if self.piss > 0.0 {
            commands.pissing
        } else {
            false
        };

        // reduce piss bar
        if self.pissing {
            self.piss = 0.0_f32.max(self.piss - 0.01);
        }

        // add piss particle
        if self.pissing {
            self.piss_particles.push_back(Particle {
                position: self.player_pos,
                started: self.now_ts,
            });
        }

        // move piss particles
        let mut particles_to_remove = 0;
        for particle in &mut self.piss_particles {
            let jump_time = commands.ts_now - particle.started;
            let jumping = jump_time < JUMP_DURATION;
            if jumping {
                let height:f64 = 1.5;
                let offset = JUMP_DURATION * 0.5;
                // let jump_speed = (height/offset).sqrt();
                let jump_speed = 1.0;
                let height_coef = height / (offset * offset);
                let x = jump_time * jump_speed - offset;
                let y = height_coef * x * x;
                particle.position.y = (height - y) as f32;
                particle.position.x += (jump_time * jump_speed) as f32;
            } else {
                particles_to_remove += 1;
            }
        }

        // remove piss particles when they touch the ground
        for _ in 0..particles_to_remove {
            self.piss_particles.pop_front();
        }
    }

    fn update_time(&mut self, commands: &Commands) {
        self.previous_frame_ts = self.now_ts;
        self.now_ts = commands.ts_now;
    }
}

fn get_random_seed() -> u64 {
    unsafe { now().floor().to_int_unchecked() }
}

fn collides(player_pos: Vec3, obstacle_pos: Vec3) -> bool {
    let obstacle_radius = 0.4;
    let player_radius = 0.5;
    let dpos = player_pos - obstacle_pos;
    let squared_distance = dpos.dot(dpos);
    let radius = obstacle_radius + player_radius;
    let squared_min_distance = radius * radius;
    squared_distance < squared_min_distance
}
