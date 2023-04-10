use macroquad::prelude::*;
use crate::world::World;

pub fn draw_obstacles(obstacles: &Vec<Vec3>) {
    let size = Vec3::new(1.0, 1.0, 1.0);
    for obstacle in obstacles {
        draw_cube(*obstacle, size, None, RED);
        draw_cube_wires(*obstacle, size, BLACK);
    }
}

pub fn draw(world: &World) {
    clear_background(LIGHTGRAY);
    draw_grid(20, 1., BLACK, GRAY);
    draw_cube(world.player_pos, Vec3::new(1.0, 2.0, 1.0), None, BLUE);
    draw_cube(
        Vec3::new(0.0, 0.5, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        None,
        GREEN,
    );
    draw_obstacles(&world.obstacles);
}
