use crate::world::Location;

#[derive(Debug)]
pub struct Zombie {
    location: Location,
}

impl Zombie {
    pub fn new(location: Location) -> Self {
        Zombie { location }
    }

    pub fn from_locations(locations: Vec<Location>) -> Vec<Self> {
        let mut zombies = vec![];
        for location in locations {
            zombies.push(Zombie::new(location));
        }

        zombies
    }
}
