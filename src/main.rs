mod common;
mod screen;
mod world;

use clap::Parser;
use git_version::git_version;
use crate::screen::commands::get_commands;
use crate::world::World;
use macroquad::prelude::*;
use screen::draw;
use crate::screen::models::load_models;

const GIT_VERSION: &str = git_version!(args = ["--tags", "--dirty"]);
const DEFAULT_WINDOW_TITLE: &'static str = "Corporation slayer";
const DEFAULT_WINDOW_WIDTH: i32 = 480;
const DEFAULT_WINDOW_HEIGHT: i32 = 640;

#[macroquad::main(window_conf)]
async fn main() {
    let args = CliArgs::parse();
    let models = load_models();
    let mut world = World::new(args.level);
    loop {
        let commands = get_commands();
        if commands.should_quit {
            break;
        }
        world.update(commands);
        draw::draw(&mut world, &models);
        next_frame().await
    }
}

#[derive(Parser, Debug)]
#[clap(version = GIT_VERSION)]
struct CliArgs {
    #[clap(long, help = "Starting level.", default_value = "0")]
    level: i32,
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}
