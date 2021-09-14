use super::*;
pub struct HouseBuildDirector {
    builder: Box<dyn HouseBuilder>,
}

impl HouseBuildDirector {
    pub fn new(builder: Box<dyn HouseBuilder>) -> Self {
        Self { builder }
    }

    pub fn build_fancy_house(self) -> Result<House, String> {
        self.builder
            .add_rooms_of_sizes(vec![40, 40, 40, 40])
            .add_bathrooms_of_sizes(vec![10, 10, 10])
            .kitchen_of_size(10)
            .pool_of_size(40)
            .build()
    }

    pub fn build_basic_house(self) -> Result<House, String> {
        self.builder
            .add_rooms_of_sizes(vec![20, 20])
            .add_bathrooms_of_sizes(vec![5])
            .kitchen_of_size(5)
            .build()
    }
}
