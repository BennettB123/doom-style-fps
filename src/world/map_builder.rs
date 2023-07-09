use super::Location;
use super::{map::Map, map::MapPiece};

#[rustfmt::skip]
    const MAP_1: &str = 
"
########################
#                      #
#                      #
#  P    #        #######
#                #######
#                #######
#                      #
#                      #
#                      #
#                      #
###   #######   ########
#                      #
#                      #
######   ########   ####
#        #      #      #
#        #      #      #
#        #      #      #
#        #      #      #
#               #      #
#               #      #
#                      #
#                      #
# # # # # # # # # # #  #
########################";

pub fn load_map_1() -> Map {
    load_map_from_string(MAP_1)
}

fn load_map_from_string(map_str: &str) -> Map {
    let mut grid: Vec<Vec<MapPiece>> = vec![];
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

fn map_char_to_map_entity(c: char) -> MapPiece {
    match c {
        ' ' | 'P' => MapPiece::Nothing,
        '#' => MapPiece::Wall,
        _ => panic!("Unsupported character found in map"),
    }
}
