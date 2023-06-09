use crate::common::TimestampSeconds;
use macroquad::prelude::*;

pub struct Obstacle {
    alive: bool,
    position: Position,
}

pub enum Position {
    Static {
        pos: Vec3,
    },
    Moving {
        initial_pos: Vec3,
        salt: f64,
        moving_right: bool,
    },
}

// might want to make a VecDeque to queue obstacles dynamically
pub type Obstacles = Vec<Obstacle>;

impl Obstacle {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            alive: true,
            position: Position::Static {
                pos: Vec3::new(x, y, z),
            },
        }
    }
    pub fn new_moving(x: f32, y: f32, z: f32, moving_right: bool) -> Self {
        Self {
            alive: true,
            position: Position::Moving {
                initial_pos: Vec3::new(x, y, z),
                salt: x as f64 % 100.0 * y as f64 % 100.0 + x as f64 + y as f64 + z as f64,
                moving_right,
            },
        }
    }
    pub fn get_pos(&self, ts: TimestampSeconds) -> Vec3 {
        match &self.position {
            Position::Static { pos } => *pos,
            Position::Moving {
                initial_pos,
                salt,
                moving_right,
            } => {
                let path_length = 6.0;
                let obstacle_speed = 3.0;
                let offset = ((ts * obstacle_speed + *salt) % path_length) as f32;
                let z = if *moving_right {
                    initial_pos.z + offset
                } else {
                    initial_pos.z + path_length as f32 - offset
                };
                Vec3::new(initial_pos.x, initial_pos.y, z)
            }
        }
    }
    pub fn get_color(&self) -> Color {
        // match self {
        //     Obstacle::Static { .. } => ORANGE,
        // Obstacle::Moving { moving_right, .. } => if *moving_right {ORANGE} else {PURPLE},
        // }
        if self.alive {
            ORANGE
        } else {
            BROWN
        }
    }
    pub fn kill(&mut self, ts: TimestampSeconds) {
        self.alive = false;
        if let Position::Moving { .. } = self.position {
            self.position = Position::Static {
                pos: self.get_pos(ts),
            }
        }
    }
    pub fn is_alive(&self) -> bool {
        self.alive
    }
}

pub fn generate_obstacles(level: i32, seed: u64) -> Vec<Obstacle> {
    let num_obstacles = 15 + level;
    const LANES: i32 = 4;
    let mut obstacles = Vec::with_capacity(num_obstacles as usize);
    let mut depth = 3.0;
    rand::srand(seed);
    loop {
        let moving_obstacle_chance = (0 + level * 2).min(90);
        if percentage_chance(moving_obstacle_chance) {
            obstacles.push(Obstacle::new_moving(depth, 0.0, -3.0, coin_flip()));
            if obstacles.len() == num_obstacles as usize {
                return obstacles;
            }
        } else {
            for i_lane in 0..LANES {
                let static_obstacle_chance = 80.min(15 + level);
                if percentage_chance(static_obstacle_chance) {
                    obstacles.push(Obstacle::new(depth, 0.0, i_lane as f32 - 1.5));
                    if obstacles.len() == num_obstacles as usize {
                        return obstacles;
                    }
                }
            }
        }
        depth += 1.0;
    }
}

fn percentage_chance(percentage_for_positive_case: i32) -> bool {
    rand::gen_range(0, 99) < percentage_for_positive_case
}

fn coin_flip() -> bool {
    percentage_chance(50)
}
