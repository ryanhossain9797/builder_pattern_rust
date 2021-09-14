use super::*;
pub struct StoneHouseBuilder {
    pub rooms: Vec<Room>,
    pub bathrooms: Vec<Bathroom>,
    pub kitchen: Option<Kitchen>,
    pub pool: Option<Pool>,
}

impl StoneHouseBuilder {
    pub fn new() -> StoneHouseBuilder {
        Self {
            rooms: Vec::new(),
            kitchen: None,
            bathrooms: Vec::new(),
            pool: None,
        }
    }
}

impl HouseBuilder for StoneHouseBuilder {
    fn add_rooms_of_sizes(mut self: Box<Self>, mut room_sizes: Vec<i8>) -> Box<dyn HouseBuilder> {
        self.rooms.append(
            &mut room_sizes
                .iter()
                .map(|size| Room {
                    wall_material: WallMaterial::Stone,
                    floor_material: FloorMaterial::Stone,
                    size: *size,
                })
                .collect(),
        );
        self
    }

    fn add_bathrooms_of_sizes(
        mut self: Box<Self>,
        mut bathroom_sizes: Vec<i8>,
    ) -> Box<dyn HouseBuilder> {
        self.bathrooms.append(
            &mut bathroom_sizes
                .iter()
                .map(|size| Bathroom {
                    wall_material: WallMaterial::Stone,
                    floor_material: FloorMaterial::Stone,
                    size: *size,
                })
                .collect(),
        );
        self
    }

    fn kitchen_of_size(mut self: Box<Self>, size: i8) -> Box<dyn HouseBuilder> {
        self.kitchen = Some(Kitchen {
            wall_material: WallMaterial::Stone,
            floor_material: FloorMaterial::Stone,
            size,
        });
        self
    }

    fn pool_of_size(mut self: Box<Self>, size: i8) -> Box<dyn HouseBuilder> {
        self.pool = Some(Pool { size });
        self
    }

    fn build(self: Box<Self>) -> Result<House, String> {
        match (self.rooms.len(), self.bathrooms.len(), self.kitchen) {
            (0, _, _) => Err(String::from("Where are you gonna sleep? in the toilet?")),
            (_, 0, _) => Err(String::from("Pooping in the woods is probably forbidden")),
            (rooms, bathrooms, _) if rooms > (bathrooms * 2) => Err(String::from(
                "You guys gonna fight over who gets to go take a shower first?",
            )),
            (_, _, None) => Err(String::from("What're you gonna eat?")),
            (_, _, Some(kitchen)) => Ok(House {
                material: "Stone".to_string(),
                rooms: self.rooms,
                bathrooms: self.bathrooms,
                kitchen,
                pool: self.pool,
            }),
        }
    }
}
