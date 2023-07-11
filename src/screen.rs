use macroquad::prelude::*;

use crate::GameState;

const BACKGROUND_COLOR: Color = BLACK;
const PLAYER_FOV: f32 = 90.0; // field of view in degrees
const SCREEN_CHUNKS_PER_FOV_DEGREE: f32 = 2.0;
const MAX_VIEW_DISTANCE: f32 = 20.0; // maximum distance at which objects are visible
const FLOOR_AND_CEILING_CHUNKS: usize = 40;
const ZOOM_FACTOR: f32 = 4.0; // control how "zoomed" the camera is

const WALL_R: u8 = 255;
const WALL_G: u8 = 255;
const WALL_B: u8 = 255;
const FLOOR_R: u8 = 170;
const FLOOR_G: u8 = 125;
const FLOOR_B: u8 = 70;
const CEILING_R: u8 = 150;
const CEILING_G: u8 = 150;
const CEILING_B: u8 = 150;

pub fn clear_screen() {
    clear_background(BACKGROUND_COLOR);
}

pub fn draw_screen(state: &GameState) {
    let player = &state.player;
    let map = &state.map;

    draw_floor();
    draw_ceiling();

    // draw map from player's perspective
    let half_fov = PLAYER_FOV / 2.0;
    let chunk_width = screen_width() / (PLAYER_FOV * SCREEN_CHUNKS_PER_FOV_DEGREE);
    let chunk_view_angle_increment = 1.0 / SCREEN_CHUNKS_PER_FOV_DEGREE;
    let mut curr_chunk = 0;
    let mut curr_chunk_view_angle = -half_fov;

    loop {
        if curr_chunk_view_angle >= half_fov {
            break;
        }

        let view_angle = player.direction + curr_chunk_view_angle;
        let dist_to_wall = map
            .distance_to_wall(&player.location, view_angle, MAX_VIEW_DISTANCE)
            .unwrap_or(MAX_VIEW_DISTANCE);
        let chunk_start_x: f32 = chunk_width * curr_chunk as f32;
        draw_wall_chunk(chunk_start_x, chunk_start_x + chunk_width, dist_to_wall);

        curr_chunk_view_angle += chunk_view_angle_increment;
        curr_chunk += 1;
    }
}

fn draw_floor() {
    let screen_h = screen_height();
    let half_screen_h = screen_h / 2.0;
    let screen_w = screen_width();
    let chunk_height: f32 = (screen_h / 2.0) / FLOOR_AND_CEILING_CHUNKS as f32;

    // draw floor as horizontal lines, increasing in brightness
    for chunk in 0..=FLOOR_AND_CEILING_CHUNKS {
        let r = map_range(
            chunk as f32,
            (0.0, FLOOR_AND_CEILING_CHUNKS as f32),
            (0.0, FLOOR_R as f32),
        ) as u8;
        let g = map_range(
            chunk as f32,
            (0.0, FLOOR_AND_CEILING_CHUNKS as f32),
            (0.0, FLOOR_G as f32),
        ) as u8;
        let b = map_range(
            chunk as f32,
            (0.0, FLOOR_AND_CEILING_CHUNKS as f32),
            (0.0, FLOOR_B as f32),
        ) as u8;

        let y = half_screen_h + chunk_height * chunk as f32;
        draw_rectangle(0.0, y, screen_w, chunk_height, color_u8!(r, g, b, 255));
    }
}

fn draw_ceiling() {
    let screen_h = screen_height();
    let half_screen_h = screen_h / 2.0;
    let screen_w = screen_width();
    let chunk_height: f32 = half_screen_h / FLOOR_AND_CEILING_CHUNKS as f32;

    // draw floor as horizontal lines, increasing in brightness
    for chunk in 0..=FLOOR_AND_CEILING_CHUNKS {
        let r = map_range(
            chunk as f32,
            (0.0, FLOOR_AND_CEILING_CHUNKS as f32),
            (CEILING_R as f32, 0.0),
        ) as u8;
        let g = map_range(
            chunk as f32,
            (0.0, FLOOR_AND_CEILING_CHUNKS as f32),
            (CEILING_G as f32, 0.0),
        ) as u8;
        let b = map_range(
            chunk as f32,
            (0.0, FLOOR_AND_CEILING_CHUNKS as f32),
            (CEILING_B as f32, 0.0),
        ) as u8;

        let y = chunk_height * chunk as f32;
        draw_rectangle(0.0, y, screen_w, chunk_height, color_u8!(r, g, b, 255));
    }
}

fn draw_wall_chunk(start_x: f32, end_x: f32, dist_to_wall: f32) {
    let screen_h = screen_height();
    let x = start_x;
    let w = end_x - start_x;
    let wall_h = ZOOM_FACTOR * (screen_height() / 2.0) / dist_to_wall;
    let r = map_range(dist_to_wall, (MAX_VIEW_DISTANCE, 0.0), (0.0, WALL_R as f32)) as u8;
    let g = map_range(dist_to_wall, (MAX_VIEW_DISTANCE, 0.0), (0.0, WALL_G as f32)) as u8;
    let b = map_range(dist_to_wall, (MAX_VIEW_DISTANCE, 0.0), (0.0, WALL_B as f32)) as u8;

    // draw wall
    draw_rectangle(
        x,
        (screen_h / 2.0) - (wall_h / 2.0),
        w,
        wall_h,
        color_u8!(r, g, b, 255),
    );
}

fn map_range(input: f32, from_range: (f32, f32), to_range: (f32, f32)) -> f32 {
    to_range.0 + (input - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
