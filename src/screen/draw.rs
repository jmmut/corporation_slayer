use std::collections::VecDeque;
use crate::common::TimestampSeconds;
use crate::screen::models::{Model, Models};
use crate::world::obstacles::Obstacles;
use crate::world::{World, PLAYER_HEIGHT};
use macroquad::models::Vertex;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::{Button, Label, Window};

const FONT_SIZE: f32 = 16.0;
const PISS_SPEED: f32 = 10.0;
const PISS_YELLOW: Color = Color::new(0.9, 0.9, 0.0, 1.0);
const PLAYER_COLOR: Color = Color::new(0.00, 0.47, 0.95, 0.3);

pub struct Drawer {
    piss_particles: VecDeque<Vec3>,
}

impl Drawer {
    pub fn new() -> Self {
        Self { piss_particles: VecDeque::new() }
    }

    pub fn draw(&mut self, world: &mut World, models: &Models) {
        set_camera(&Camera3D {
            position: vec3(-3.0 + world.player_pos.x, 4.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(2.0 + world.player_pos.x, 0.0, 0.0),
            ..Default::default()
        });
        clear_background(GRAY);
        // draw_grid(20, 1., BLACK, GRAY);
        draw_walls(world);
        draw_obstacles(&world.obstacles, world.previous_frame_ts);
        self.draw_piss_spray(world);
        draw_player(world, &models.player);
        draw_hud(world);
    }

    pub fn draw_piss_spray(&mut self, world: &World) {
        if world.pissing {
            self.piss_particles.push_back(world.player_pos);
        }
        let dt = world.now_ts - world.previous_frame_ts;
        for particle in &mut self.piss_particles {
            particle.x += PISS_SPEED * dt as f32;
            draw_cube_from_floor(*particle, Vec3::new(0.1, 0.1, 0.1), None, PISS_YELLOW)
        }
    }
}


fn draw_walls(world: &World) {
    let starting_wall = 5.0;
    let end_x = world
        .obstacles
        .last()
        .unwrap()
        .get_pos(world.previous_frame_ts)
        .x
        + 10.0
        + starting_wall;
    let v0 = Vec3::new(-5.0, 6.0, -2.0);
    let v0v1 = Vec3::new(0.0, -6.0, 0.0);
    let v0v3 = Vec3::new(end_x, 0.0, 0.0);
    let corners = compute_plane_corners(v0, v0v1, v0v3);
    draw_mesh(&Mesh {
        texture: None,
        vertices: point_to_vertex_no_texture(corners, LIGHTGRAY),
        indices: vec![0, 1, 2, 0, 2, 3],
    });
    let v0 = Vec3::new(-5.0, 6.0, 2.0);
    let v0v1 = Vec3::new(0.0, -6.0, 0.0);
    let v0v3 = Vec3::new(end_x, 0.0, 0.0);
    let corners = compute_plane_corners(v0, v0v1, v0v3);
    draw_mesh(&Mesh {
        texture: None,
        vertices: point_to_vertex_no_texture(corners, LIGHTGRAY),
        indices: vec![0, 1, 2, 0, 2, 3],
    });
}

fn compute_plane_corners(v0: Vec3, v0v1: Vec3, v0v3: Vec3) -> Vec<Vec3> {
    vec![v0, v0 + v0v1, v0 + v0v1 + v0v3, v0 + v0v3]
}

fn point_to_vertex_no_texture(points: Vec<Vec3>, color: Color) -> Vec<Vertex> {
    points
        .iter()
        .map(|p| Vertex {
            position: *p,
            uv: Vec2::new(0.0, 0.0),
            color,
        })
        .collect()
}

fn draw_player(world: &World, _model: &Model) {
    let color = if world.colliding { RED } else { PLAYER_COLOR };
    draw_cube_from_floor(
        world.player_pos,
        Vec3::new(1.0, PLAYER_HEIGHT, 1.0),
        None,
        color,
    );
    // push_camera_state();
    // set_camera(&Camera3D {
    //     position: vec3(-3.0 + world.player_pos.x, 4.0, 0.0) - world.player_pos,
    //     up: vec3(0.0, 1.0, 0.0),
    //     target: vec3(2.0 + world.player_pos.x, 0.0, 0.0) - world.player_pos,
    //     ..Default::default()
    // });
    // draw_mesh(model);
    // pop_camera_state();
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

pub fn draw_obstacles(obstacles: &Obstacles, ts: TimestampSeconds) {
    let size = Vec3::new(0.8, 0.5, 0.8);
    for obstacle in obstacles {
        draw_cube_from_floor(obstacle.get_pos(ts), size, None, obstacle.get_color());
    }
}

fn draw_hud(world: &mut World) {
    set_default_camera();
    draw_health(world);
    draw_piss(world);
    draw_level_finished(world);
    draw_game_over(world);
}

fn draw_health(world: &World) {
    let full_width = screen_width();
    let width = 0.4 * full_width;
    let padding = 0.05 * full_width;
    let padding_text = 0.035 * full_width;
    draw_rectangle(
        padding,
        padding,
        width * world.health,
        padding * 2.0,
        DARKGREEN,
    );
    draw_rectangle_lines(padding, padding, width, padding * 2.0, 4.0, BLACK);
    draw_text("HEALTH", padding, padding_text, FONT_SIZE, BLACK);
}

fn draw_piss(world: &World) {
    let full_width = screen_width();
    let width = 0.4 * full_width;
    let padding = 0.05 * full_width;
    draw_rectangle(padding + 0.5*full_width, padding, width, padding * 2.0,
        Color::new(0.4, 0.75, 1.0, 0.3),
    );
    draw_rectangle(
        full_width - padding - width * world.piss,
        padding,
        width * world.piss,
        padding * 2.0,
        PISS_YELLOW,
    );
    draw_rectangle_lines(padding + 0.5*full_width, padding, width, padding * 2.0, 4.0, BLACK);
    let text = "PISS";
    let text_length = measure_text(text, None, FONT_SIZE as u16, 1.0);
    let padding_text = 0.035 * full_width;
    draw_text(text, full_width - padding - text_length.width, padding_text, FONT_SIZE, BLACK);
}

fn draw_level_finished(world: &mut World) {
    let end_x = world
        .obstacles
        .last()
        .unwrap()
        .get_pos(world.previous_frame_ts)
        .x
        + 10.0;
    if world.player_pos.x > end_x {
        let w = screen_width();
        let h = screen_height();
        Window::new(1, Vec2::new(w / 4.0, h / 4.0), Vec2::new(w / 2.0, h / 4.0))
            .label(&format!("Level {} completed!", world.level + 1))
            .ui(&mut root_ui(), |ui| {
                if Button::new(format!("Continue to level {}", world.level + 2).as_str()).ui(ui)
                    || is_key_down(KeyCode::Space)
                    || is_key_down(KeyCode::Enter)
                    || is_key_down(KeyCode::KpEnter)
                {
                    world.level += 1;
                    world.piss = 1.0_f32.min(world.piss + 0.1);
                    world.regenerate();
                }
            });
    }
}

fn draw_game_over(world: &mut World) {
    if let Some(end) = world.game_end {
        let w = screen_width();
        let h = screen_height();
        Window::new(1, Vec2::new(w / 4.0, h / 4.0), Vec2::new(w / 2.0, h / 4.0))
            .label("Game Over")
            .ui(&mut root_ui(), |ui| {
                Label::new(format!("Reached level {}", world.level + 1)).ui(ui);
                Label::new(format!("(and {:.2} meters)", world.player_pos.x)).ui(ui);
                Label::new(format!("in {:.3} seconds", end - world.game_start)).ui(ui);
                if Button::new("Restart").ui(ui)
                    || is_key_down(KeyCode::Space)
                    || is_key_down(KeyCode::Enter)
                    || is_key_down(KeyCode::KpEnter)
                {
                    *world = World::new(0);
                }
            });
    }
}
