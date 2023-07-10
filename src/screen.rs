use macroquad::prelude::*;

use crate::GameState;

const BACKGROUND_COLOR: Color = BLACK;
const PLAYER_FOV: f32 = 90.0; // field of view in degrees
const SCREEN_CHUNKS_PER_FOV_DEGREE: f32 = 2.0;
const MAX_VIEW_DISTANCE: f32 = 15.0; // maximum distance at which objects are visible
const MAX_VIEW_DISTANCE_WALL_HEIGHT: f32 = 20.0; // % of screen height that max distance walls appear

const WALL_MAX_ALPHA: u8 = 255;
const WALL_MIN_ALPHA: u8 = 0;
const WALL_R: u8 = 255;
const WALL_G: u8 = 255;
const WALL_B: u8 = 255;

pub fn clear_screen() {
    clear_background(BACKGROUND_COLOR);
}

pub fn draw_screen(state: &GameState) {
    let player = &state.player;
    let map = &state.map;

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
        if let Some(dist_to_wall) =
            map.distance_to_wall(&player.location, view_angle, MAX_VIEW_DISTANCE)
        {
            let chunk_start_x: f32 = chunk_width * curr_chunk as f32;
            draw_wall_chunk(chunk_start_x, chunk_start_x + chunk_width, dist_to_wall);
        }

        curr_chunk_view_angle += chunk_view_angle_increment;
        curr_chunk += 1;
    }
}

// TODO: fix this function
fn draw_wall_chunk(start_x: f32, end_x: f32, dist_to_wall: f32) {
    let x = start_x;
    let w = end_x - start_x;
    let h = 20.0 * (MAX_VIEW_DISTANCE_WALL_HEIGHT * MAX_VIEW_DISTANCE / dist_to_wall); // TODO
    let y = (screen_height() / 2.0) - (h / 2.0);
    let alpha: u8 = map_range(
        dist_to_wall,
        (MAX_VIEW_DISTANCE, 0.0),
        (WALL_MIN_ALPHA as f32, WALL_MAX_ALPHA as f32),
    ) as u8;

    draw_rectangle(x, y, w, h, color_u8!(WALL_R, WALL_G, WALL_B, alpha));
}

fn map_range(input: f32, from_range: (f32, f32), to_range: (f32, f32)) -> f32 {
    to_range.0 + (input - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
