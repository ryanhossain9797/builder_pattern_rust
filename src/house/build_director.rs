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
            .add_rooms(vec![
                Room { size: 40 },
                Room { size: 40 },
                Room { size: 40 },
                Room { size: 40 },
            ])
            .add_bathrooms(vec![
                Bathroom { size: 10 },
                Bathroom { size: 10 },
                Bathroom { size: 10 },
            ])
            .kitchen(Kitchen { size: 10 })
            .pool(Pool { size: 40 })
            .build()
    }

    pub fn build_basic_house(self) -> Result<House, String> {
        self.builder
            .add_rooms(vec![Room { size: 20 }, Room { size: 20 }])
            .add_bathrooms(vec![Bathroom { size: 5 }])
            .kitchen(Kitchen { size: 5 })
            .build()
    }
}
