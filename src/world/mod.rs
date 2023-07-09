pub mod map;
pub mod map_builder;
pub mod player;

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
