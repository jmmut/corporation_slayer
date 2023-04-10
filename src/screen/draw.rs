use crate::world::{PLAYER_HEIGHT, World};
use macroquad::prelude::*;

pub fn draw(world: &World) {
    set_camera(&Camera3D {
        position: vec3(-5.0  + world.player_pos.x, 3.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0. + world.player_pos.x, 0.0, 0.0),
        ..Default::default()
    });
    clear_background(LIGHTGRAY);
    draw_grid(20, 1., BLACK, GRAY);
    draw_player(world);
    draw_obstacles(&world.obstacles);
}

fn draw_player(world: &World) {
    let color = if world.colliding { RED } else {BLUE};
    draw_cube_from_floor(world.player_pos, Vec3::new(1.0, PLAYER_HEIGHT, 1.0), None, color);
}

pub fn draw_cube_from_floor(
    floor_position: Vec3,
    size: Vec3,
    texture: impl Into<Option<Texture2D>>,
    color: Color,
) {
    let offset = Vec3::new(0.0, size.y / 2.0, 0.0);
    draw_cube(floor_position + offset, size, texture, color);
}

pub fn draw_obstacles(obstacles: &Vec<Vec3>) {
    let size = Vec3::new(0.8, 0.5, 0.8);
    for obstacle in obstacles {
        draw_cube_from_floor(*obstacle, size, None, ORANGE);
    }
}
