pub mod stone_builder;
pub mod wood_builder;

use super::*;
use std::fmt::Display;
pub use stone_builder::*;
pub use wood_builder::*;

pub struct House {
    material: String,
    rooms: Vec<Room>,
    kitchen: Kitchen,
    pool: Option<Pool>,
    bathrooms: Vec<Bathroom>,
}

impl Display for House {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = format!(
            "A house made of {} with {} room(s), {} bathroom(s), a Kitchen of size {} and {}",
            self.material,
            self.rooms.len(),
            self.bathrooms.len(),
            self.kitchen.size,
            match &self.pool {
                Some(_) => "with a pool",
                _ => "without a pool",
            }
        );
        write!(f, "{}", message)
    }
}

pub trait HouseBuilder {
    fn add_rooms(self: Box<Self>, rooms: Vec<Room>) -> Box<dyn HouseBuilder>;

    fn add_bathrooms(self: Box<Self>, bathrooms: Vec<Bathroom>) -> Box<dyn HouseBuilder>;

    fn pool(self: Box<Self>, pool: Pool) -> Box<dyn HouseBuilder>;

    fn kitchen(self: Box<Self>, kitchen: Kitchen) -> Box<dyn HouseBuilder>;

    fn build(self: Box<Self>) -> Result<House, String>;
}
