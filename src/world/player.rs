use macroquad::time::get_frame_time;

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
    pub direction: f32, // direction player is facing in degrees
}

impl Player {
    const MOVEMENT_SPEED: f32 = 5.0; // positions per second
    const ROTATION_SPEED: f32 = 120.0; // degrees per second

    pub fn new(location: Location) -> Self {
        Player {
            location,
            direction: 0.0,
        }
    }

    pub fn look_right(&mut self) {
        self.direction += get_frame_time() * Player::ROTATION_SPEED;
        if self.direction >= 360.0 {
            self.direction -= 360.0;
        }
    }

    pub fn look_left(&mut self) {
        self.direction -= get_frame_time() * Player::ROTATION_SPEED;
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

        let frame_time = get_frame_time();

        self.location
            .move_x(frame_time * Player::MOVEMENT_SPEED * move_angle.to_radians().cos());
        self.location
            .move_y(frame_time * Player::MOVEMENT_SPEED * move_angle.to_radians().sin());
    }
}
