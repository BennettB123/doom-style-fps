use super::Location;

pub enum Direction {
    Forward,
    Back,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Player {
    pub location: Location,
    pub direction: f32, // direction player is facing in degrees (0 - 360)
    pub fov: f32,       // field of view in degrees
}

impl Player {
    const MOVEMENT_SPEED: f32 = 0.1; // speed per second. TODO
    const ROTATION_SPEED: f32 = 90.0; // angle rotation per second. TODO

    pub fn new(location: Location) -> Self {
        Player {
            location,
            direction: 0.0,
            fov: 90.0,
        }
    }

    pub fn look_right(&mut self) {
        self.direction += 2.0;
        if self.direction >= 360.0 {
            self.direction -= 360.0;
        }
    }

    pub fn look_left(&mut self) {
        self.direction -= 2.0;
        if self.direction <= 0.0 {
            self.direction += 360.0;
        }
    }

    pub fn move_direction(&mut self, dir: &Direction) {
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
