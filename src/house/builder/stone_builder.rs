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
    fn add_rooms(mut self: Box<Self>, mut rooms: Vec<Room>) -> Box<dyn HouseBuilder> {
        self.rooms.append(&mut rooms);
        self
    }

    fn add_bathrooms(mut self: Box<Self>, mut bathrooms: Vec<Bathroom>) -> Box<dyn HouseBuilder> {
        self.bathrooms.append(&mut bathrooms);
        self
    }

    fn kitchen(mut self: Box<Self>, kitchen: Kitchen) -> Box<dyn HouseBuilder> {
        self.kitchen = Some(kitchen);
        self
    }

    fn pool(mut self: Box<Self>, pool: Pool) -> Box<dyn HouseBuilder> {
        self.pool = Some(pool);
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
