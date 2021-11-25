pub mod fancy_builder;
pub mod stone_builder;

use super::*;
pub use fancy_builder::*;
use std::fmt::Display;
pub use stone_builder::*;

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
    fn pool_of_size(self: Box<Self>, size: i8) -> Box<dyn HouseBuilder>;

    fn build(self: Box<Self>) -> House;
}

pub trait HouseBuilderAwaitingRooms {
    fn add_rooms_of_sizes(
        self: Box<Self>,
        room_sizes: Vec<i8>,
    ) -> Result<Box<dyn HouseBuilderAwaitingBathrooms>, String>;
}

pub trait HouseBuilderAwaitingBathrooms {
    fn add_bathrooms_of_sizes(
        self: Box<Self>,
        bathroom_sizes: Vec<i8>,
    ) -> Result<Box<dyn HouseBuilderAwaitingKitchen>, String>;
}

pub trait HouseBuilderAwaitingKitchen {
    fn add_kitchen_of_size(self: Box<Self>, size: i8) -> Result<Box<dyn HouseBuilder>, String>;
}
