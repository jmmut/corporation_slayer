mod common;
mod screen;
mod world;

use crate::screen::commands::get_commands;
use crate::world::{generate_obstacles, World};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use screen::draw;

const DEFAULT_WINDOW_TITLE: &'static str = "Corporation slayer";
const DEFAULT_WINDOW_WIDTH: i32 = 480;
const DEFAULT_WINDOW_HEIGHT: i32 = 640;

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = initialize();
    loop {
        let commands = get_commands();
        if commands.should_quit {
            break;
        }
        world.update(commands);
        draw::draw(&world);
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}

fn initialize() -> World {
    let world = World {
        player_pos: Vec3::new(0.0, 0.0, 0.0),
        jump_started: 0.0,
        obstacles: generate_obstacles(),
        previous_frame_ts: now(),
        colliding: false,
    };

    world
}
