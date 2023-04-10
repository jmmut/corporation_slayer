mod commands;
mod world;

use crate::commands::get_commands;
use crate::world::World;
use macroquad::prelude::*;

const DEFAULT_WINDOW_TITLE: &'static str = "Corporation slayer";
const DEFAULT_WINDOW_WIDTH: i32 = 480;
const DEFAULT_WINDOW_HEIGHT: i32 = 640;

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World {
        player_pos: Vec3::new(0.0, 1.0, 0.0),
        jump_started: 0.0,
    };

    set_camera(&Camera3D {
        position: vec3(-7., 3., 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 3., 0.),
        ..Default::default()
    });
    loop {
        let commands = get_commands();
        if commands.should_quit {
            break;
        }
        world.update(commands);

        clear_background(LIGHTGRAY);
        draw_grid(20, 1., BLACK, GRAY);
        draw_cube(world.player_pos, Vec3::new(1.0, 2.0, 1.0), None, BLUE);
        draw_cube(
            Vec3::new(0.0, 0.5, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            None,
            GREEN,
        );
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
