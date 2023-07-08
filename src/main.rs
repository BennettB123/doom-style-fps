use macroquad::prelude::*;

mod screen;

const GAME_NAME: &str = "Doom Style FPS";

#[macroquad::main(get_window_configuration())]
async fn main() {
    let map = map_builder::load_map_1();
    let player = Player::new(map.player_start);
    let mut game_state = GameState { map, player };

    // Enter main game loop
    while !exit_button_pressed() {
        capture_user_input(&mut game_state);

        screen::clear_screen();
        screen::draw_screen(&game_state);

        next_frame().await
    }
}

pub fn get_window_configuration() -> Conf {
    Conf {
        window_title: GAME_NAME.to_owned(),
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
    const DISTANCE_INCREMENT: f32 = 0.1;

    fn distance_to_wall(&self, start_location: &Location, direction: f32) -> f32 {
        let mut distance = 0.0;
        let mut curr_location = *start_location;

        loop {
            if self.is_wall(&curr_location) {
                break;
            }

            distance += Map::DISTANCE_INCREMENT;
            curr_location.x += Map::DISTANCE_INCREMENT * direction.to_radians().cos();
            curr_location.y += Map::DISTANCE_INCREMENT * direction.to_radians().sin();
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
