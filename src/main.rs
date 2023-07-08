use macroquad::prelude::*;

const SCREEN_CHUNKS_PER_FOV_DEGREE: f32 = 2.0;
const MAX_VIEW_DISTANCE: f32 = 15.0; // maximum distance at which objects are visible
const MAX_VIEW_DISTANCE_WALL_HEIGHT: f32 = 20.0; // % of screen height that max dist walls appear
const DISTANCE_STEP_INCREMENT: f32 = 0.1;

#[macroquad::main(get_window_conf())]
async fn main() {
    let map = map_builder::load_map_1();
    let player = Player::new(map.player_start);
    let mut game_state = GameState { map, player };

    // Enter main game loop
    while !exit_button_pressed() {
        clear_background(BLACK);

        capture_user_input(&mut game_state);
        draw_scene(&game_state);

        next_frame().await
    }
}

fn get_window_conf() -> Conf {
    Conf {
        window_title: "doom-style-fps".to_owned(),
        high_dpi: true,
        fullscreen: false,
        window_width: 2500,
        window_height: 1500,
        ..Conf::default()
    }
}

fn exit_button_pressed() -> bool {
    is_key_down(KeyCode::Escape) || is_key_down(KeyCode::Q)
}

fn draw_scene(state: &GameState) {
    let player = &state.player;
    let map = &state.map;

    // draw map from player's perspective
    let half_fov = player.fov / 2.0;
    let chunk_width = screen_width() / (player.fov * SCREEN_CHUNKS_PER_FOV_DEGREE);
    let chunk_view_angle_increment = 1.0 / SCREEN_CHUNKS_PER_FOV_DEGREE;
    let mut curr_chunk = 1;
    let mut curr_chunk_view_angle = -half_fov;

    loop {
        if curr_chunk_view_angle >= half_fov {
            break;
        }

        let view_angle = player.direction + curr_chunk_view_angle;
        let dist_to_wall = map.distance_to_wall(&player.location, view_angle);
        let chunk_start_x: f32 = chunk_width * curr_chunk as f32;
        draw_wall_chunk(chunk_start_x, chunk_start_x + chunk_width, dist_to_wall);

        curr_chunk_view_angle += chunk_view_angle_increment;
        curr_chunk += 1;
    }
}

fn draw_wall_chunk(start_x: f32, end_x: f32, dist_to_wall: f32) {
    const MAX_ALPHA: u8 = 200;
    const MIN_ALPHA: u8 = 50;

    let x = start_x;
    let w = end_x - start_x;
    let h = 20.0 * (MAX_VIEW_DISTANCE_WALL_HEIGHT * MAX_VIEW_DISTANCE / dist_to_wall); // TODO: fix
    let y = (screen_height() / 2.0) - (h / 2.0);
    let alpha: u8 = map_range(
        dist_to_wall,
        (MAX_VIEW_DISTANCE, 0.0),
        (MIN_ALPHA as f32, MAX_ALPHA as f32),
    ) as u8;

    draw_rectangle(x, y, w, h, color_u8!(255, 255, 255, alpha));
}

fn map_range(input: f32, from_range: (f32, f32), to_range: (f32, f32)) -> f32 {
    to_range.0 + (input - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn capture_user_input(state: &mut GameState) {
    if is_key_down(KeyCode::Right) {
        state.player.look_right();
    }
    if is_key_down(KeyCode::Left) {
        state.player.look_left();
    }
    if is_key_down(KeyCode::W) {
        state.player.move_direction(&Direction::Forward);
    }
    if is_key_down(KeyCode::A) {
        state.player.move_direction(&Direction::Left);
    }
    if is_key_down(KeyCode::S) {
        state.player.move_direction(&Direction::Back);
    }
    if is_key_down(KeyCode::D) {
        state.player.move_direction(&Direction::Right);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

impl Location {
    pub fn new(x: f32, y: f32) -> Self {
        Location { x, y }
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn move_x(&mut self, amt: f32) {
        self.x += amt;
    }

    pub fn move_y(&mut self, amt: f32) {
        self.y += amt;
    }
}

enum Direction {
    Forward,
    Back,
    Left,
    Right,
}

#[derive(Debug)]
struct Player {
    location: Location,
    direction: f32, // direction player is facing in degrees (0 - 360)
    fov: f32,       // field of view in degrees
}

impl Player {
    const MOVEMENT_SPEED: f32 = 0.1; // speed per second. TODO
    const ROTATION_SPEED: f32 = 90.0; // angle rotation per second. TODO

    fn new(location: Location) -> Self {
        Player {
            location,
            direction: 0.0,
            fov: 90.0,
        }
    }

    fn look_right(&mut self) {
        self.direction += 2.0;
        if self.direction >= 360.0 {
            self.direction -= 360.0;
        }
    }

    fn look_left(&mut self) {
        self.direction -= 2.0;
        if self.direction <= 0.0 {
            self.direction += 360.0;
        }
    }

    fn move_direction(&mut self, dir: &Direction) {
        let mut move_angle = self.direction;
        match dir {
            Direction::Forward => (),
            Direction::Back => move_angle += 180.0,
            Direction::Left => move_angle += 270.0,
            Direction::Right => move_angle += 90.0,
        }

        self.location
            .move_x(Player::MOVEMENT_SPEED * move_angle.to_radians().cos());
        self.location
            .move_y(Player::MOVEMENT_SPEED * move_angle.to_radians().sin());
    }
}

#[derive(Debug)]
enum MapEntity {
    Nothing,
    Wall,
}

#[derive(Debug)]
pub struct Map {
    grid: Vec<Vec<MapEntity>>,
    player_start: Location,
}

impl Map {
    fn distance_to_wall(&self, start_location: &Location, direction: f32) -> f32 {
        let mut distance = 0.0;
        let mut curr_location = *start_location;

        loop {
            if self.is_wall(&curr_location) {
                break;
            }

            distance += DISTANCE_STEP_INCREMENT;
            curr_location.x += DISTANCE_STEP_INCREMENT * direction.to_radians().cos();
            curr_location.y += DISTANCE_STEP_INCREMENT * direction.to_radians().sin();
        }
        distance
    }

    fn is_wall(&self, location: &Location) -> bool {
        match self.grid[location.x as usize][location.y as usize] {
            MapEntity::Wall => true,
            _ => false,
        }
    }
}

pub struct GameState {
    map: Map,
    player: Player,
}

mod map_builder {
    use crate::{Location, Map, MapEntity};

    #[rustfmt::skip]
    const MAP_1: &str = 
"
########################
#                      #
# P                    #
#                      #
#                      #
#                      #
#                      #
#                      #
#                      #
#         ####         #
#         #  #         #
#         #  #         #
#         ####         #
#                      #
#                      #
#                      #
#                      #
#                      #
#                      #
#                      #
#                      #
########################";

    pub fn load_map_1() -> Map {
        load_map_from_string(MAP_1)
    }

    fn load_map_from_string(map_str: &str) -> Map {
        let mut grid: Vec<Vec<MapEntity>> = vec![];
        let mut player: Location = Location::new(0.0, 0.0);

        for (y, line) in map_str.lines().enumerate() {
            let mut grid_line = vec![];
            for (x, c) in line.chars().enumerate() {
                grid_line.push(map_char_to_map_entity(c));
                if c == 'P' {
                    player.set_x(x as f32);
                    player.set_y(y as f32);
                }
            }
            grid.push(grid_line);
        }

        if player.x == 0.0 && player.y == 0.0 {
            panic!("Attempted to load a map without a player's starting location");
        }

        Map {
            grid,
            player_start: player,
        }
    }

    fn map_char_to_map_entity(c: char) -> MapEntity {
        match c {
            ' ' | 'P' => MapEntity::Nothing,
            '#' => MapEntity::Wall,
            _ => panic!("Unsupported character found in map"),
        }
    }
}
