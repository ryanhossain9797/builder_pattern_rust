pub mod stone_builder;
pub mod wood_builder;

use super::*;
pub use stone_builder::*;
pub use wood_builder::*;

pub trait HouseBuilder {
    fn add_rooms(self: Box<Self>, rooms: Vec<Room>) -> Box<dyn HouseBuilder>;

    fn add_bathrooms(self: Box<Self>, bathrooms: Vec<Bathroom>) -> Box<dyn HouseBuilder>;

    fn pool(self: Box<Self>, pool: Pool) -> Box<dyn HouseBuilder>;

    fn kitchen(self: Box<Self>, kitchen: Kitchen) -> Box<dyn HouseBuilder>;

    fn build(self: Box<Self>) -> Result<House, String>;
}
