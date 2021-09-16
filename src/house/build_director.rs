use super::*;
use std::mem::{swap, take};

const NO_BUILDER: &str = "No builder";

pub struct HouseBuildDirector {
    builder: Option<Box<dyn HouseBuilder>>,
}

impl HouseBuildDirector {
    pub fn new() -> Self {
        Self { builder: None }
    }

    pub fn update_builder(&mut self, builder: Box<dyn HouseBuilder>) {
        swap(&mut self.builder, &mut Some(builder));
    }

    pub fn build_fancy_house(&mut self) -> Result<House, String> {
        let mut builder = None;
        swap(&mut self.builder, &mut builder);

        builder
            .ok_or_else(|| NO_BUILDER.to_string())?
            .add_rooms_of_sizes(vec![40, 40, 40, 40])
            .add_bathrooms_of_sizes(vec![10, 10, 10])
            .kitchen_of_size(10)
            .pool_of_size(40)
            .build()
    }

    pub fn build_basic_house(&mut self) -> Result<House, String> {
        take(&mut self.builder)
            .ok_or_else(|| NO_BUILDER.to_string())?
            .add_rooms_of_sizes(vec![20, 20])
            .add_bathrooms_of_sizes(vec![5])
            .kitchen_of_size(5)
            .build()
    }
}
