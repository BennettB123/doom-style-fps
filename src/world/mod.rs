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
}
