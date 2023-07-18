use super::Location;
use super::{map::Map, map::MapPiece};

pub struct MapInfo {
    pub map: Map,
    pub player_start: Location,
    pub starting_zombies: Vec<Location>,
}

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
#        #  Z   #      #
#        #      #      #
#        #      #      #
#               #      #
#               #      #
#                      #
#                      #
# # # # # # # # # # #  #
########################";

pub fn load_map_1() -> MapInfo {
    load_map_from_string(MAP_1)
}

fn load_map_from_string(map_str: &str) -> MapInfo {
    let mut grid: Vec<Vec<MapPiece>> = vec![];
    let mut player: Location = Location::new(0.0, 0.0);
    let mut zombies: Vec<Location> = vec![];

    for (y, line) in map_str.lines().enumerate() {
        let mut grid_line = vec![];
        for (x, c) in line.chars().enumerate() {
            grid_line.push(map_char_to_map_entity(c));
            if c == 'P' {
                player.x = x as f32;
                player.y = y as f32;
            } else if c == 'Z' {
                zombies.push(Location::new(x as f32, y as f32));
            }
        }
        grid.push(grid_line);
    }

    // panic if there's no starting point for player
    if player.x == 0.0 && player.y == 0.0 {
        panic!("Attempted to load a map without a player's starting location");
    }

    MapInfo {
        map: Map { grid },
        player_start: player,
        starting_zombies: zombies,
    }
}

fn map_char_to_map_entity(c: char) -> MapPiece {
    match c {
        ' ' | 'P' | 'Z' => MapPiece::Nothing,
        '#' => MapPiece::Wall,
        _ => panic!("Unsupported character found in map"),
    }
}
