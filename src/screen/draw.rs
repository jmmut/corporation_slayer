use crate::world::World;
use macroquad::prelude::*;

pub fn draw(world: &World) {
    clear_background(LIGHTGRAY);
    draw_grid(20, 1., BLACK, GRAY);
    draw_cube_from_floor(world.player_pos, Vec3::new(1.0, 2.0, 1.0), None, BLUE);
    draw_obstacles(&world.obstacles);
}

pub fn draw_cube_from_floor(
    floor_position: Vec3,
    size: Vec3,
    texture: impl Into<Option<Texture2D>>,
    color: Color,
) {
    draw_cube(
        floor_position + Vec3::new(0.0, size.y / 2.0, 0.0),
        size,
        texture,
        color,
    );
}

pub fn draw_obstacles(obstacles: &Vec<Vec3>) {
    let size = Vec3::new(1.0, 1.0, 1.0);
    for obstacle in obstacles {
        draw_cube_from_floor(*obstacle, size, None, RED);
    }
}
