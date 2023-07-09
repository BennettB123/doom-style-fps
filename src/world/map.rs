use super::Location;

#[derive(Debug)]
pub enum MapPiece {
    Nothing,
    Wall,
}

#[derive(Debug)]
pub struct Map {
    pub grid: Vec<Vec<MapPiece>>,
    pub player_start: Location,
}

impl Map {
    const DISTANCE_INCREMENT: f32 = 0.1;

    pub fn distance_to_wall(&self, start_location: &Location, direction: f32) -> f32 {
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

    pub fn is_wall(&self, location: &Location) -> bool {
        match self.grid[location.x as usize][location.y as usize] {
            MapPiece::Wall => true,
            _ => false,
        }
    }
}
