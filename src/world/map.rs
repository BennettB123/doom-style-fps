use super::Location;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MapPiece {
    // TODO: better name for this enum
    Nothing,
    Wall,
    OutOfBounds,
}

#[derive(Debug)]
pub struct Map {
    pub grid: Vec<Vec<MapPiece>>,
    pub player_start: Location,
}

impl Map {
    const DISTANCE_INCREMENT: f32 = 0.1;

    pub fn distance_to_wall(&self, start: &Location, direction: f32, max_dist: f32) -> Option<f32> {
        let mut distance = 0.0;
        let mut curr_location = *start;

        loop {
            if distance >= max_dist {
                return None;
            }
            if self.get_piece_at_location(&curr_location) == MapPiece::Wall {
                break;
            }

            distance += Map::DISTANCE_INCREMENT;
            curr_location.x += Map::DISTANCE_INCREMENT * direction.to_radians().cos();
            curr_location.y += Map::DISTANCE_INCREMENT * direction.to_radians().sin();
        }
        Some(distance)
    }

    pub fn get_piece_at_location(&self, location: &Location) -> MapPiece {
        // this logic doesn't work if the location is negative
        //   this *shouldn't* cause issues since the player *shouldn't* be there...
        match self.grid.get(location.y as usize) {
            None => MapPiece::OutOfBounds,
            Some(row) => match row.get(location.x as usize) {
                None => MapPiece::OutOfBounds,
                Some(piece) => *piece,
            },
        }
    }
}
