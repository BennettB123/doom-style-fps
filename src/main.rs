use macroquad::prelude::*;

mod screen;
mod world;

use world::map::Map;
use world::map_builder;
use world::player::{Direction, Player};
use world::zombie::Zombie;

const GAME_NAME: &str = "Doom Style FPS";

#[macroquad::main(get_window_configuration())]
async fn main() {
    // bake sprite assets and textures into binary
    let zombie_png = include_bytes!("../assets/zombie.png");
    let textures = SpriteTextures {
        zombie: Texture2D::from_file_with_format(zombie_png, None),
    };

    let map_info = map_builder::load_map_1();

    let mut game_state = GameState {
        map: map_info.map,
        player: Player::new(map_info.player_start),
        zombies: Zombie::from_locations(map_info.starting_zombies),
    };

    // Enter main game loop
    while !exit_button_pressed() {
        capture_user_input(&mut game_state);

        screen::clear_screen();
        screen::draw_screen(&game_state, &textures);

        next_frame().await
    }
}

pub fn get_window_configuration() -> Conf {
    Conf {
        window_title: GAME_NAME.to_owned(),
        high_dpi: true,
        fullscreen: false,
        window_width: 1920,
        window_height: 1080,
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
        state.player.move_direction(&Direction::Forward, &state.map);
    }
    if is_key_down(KeyCode::A) {
        state.player.move_direction(&Direction::Left, &state.map);
    }
    if is_key_down(KeyCode::S) {
        state.player.move_direction(&Direction::Back, &state.map);
    }
    if is_key_down(KeyCode::D) {
        state.player.move_direction(&Direction::Right, &state.map);
    }
}

pub struct GameState {
    map: Map,
    player: Player,
    zombies: Vec<Zombie>,
}

pub struct SpriteTextures {
    zombie: Texture2D,
}
