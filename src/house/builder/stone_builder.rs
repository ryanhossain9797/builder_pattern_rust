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

impl HouseBuilderAwaitingRooms for StoneHouseBuilder {
    fn add_rooms_of_sizes(
        mut self: Box<Self>,
        room_sizes: Vec<i8>,
    ) -> Result<Box<dyn HouseBuilderAwaitingBathrooms>, String> {
        if !room_sizes.is_empty() {
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
            Ok(self)
        } else {
            Err("Need at least one room".to_string())
        }
    }
}

impl HouseBuilderAwaitingBathrooms for StoneHouseBuilder {
    fn add_bathrooms_of_sizes(
        mut self: Box<Self>,
        bathroom_sizes: Vec<i8>,
    ) -> Result<Box<dyn HouseBuilderAwaitingKitchen>, String> {
        if bathroom_sizes.len() < (self.rooms.len() / 2) + 1 {
            Err("Insufficient number of bathrooms".to_string())
        } else if !bathroom_sizes.is_empty() {
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
            Ok(self)
        } else {
            Err("Need at least one room".to_string())
        }
    }
}

impl HouseBuilderAwaitingKitchen for StoneHouseBuilder {
    fn add_kitchen_of_size(mut self: Box<Self>, size: i8) -> Result<Box<dyn HouseBuilder>, String> {
        match size {
            0 => Err("Kitchen size can't be 0".to_string()),
            _ => {
                self.kitchen = Some(Kitchen {
                    wall_material: WallMaterial::Stone,
                    floor_material: FloorMaterial::Stone,
                    size,
                });
                Ok(self)
            }
        }
    }
}

impl HouseBuilder for StoneHouseBuilder {
    fn pool_of_size(mut self: Box<Self>, size: i8) -> Box<dyn HouseBuilder> {
        self.pool = Some(Pool { size });
        self
    }

    fn build(self: Box<Self>) -> House {
        House {
            material: "Stone".to_string(),
            rooms: self.rooms,
            bathrooms: self.bathrooms,
            kitchen: self.kitchen.unwrap(),
            pool: self.pool,
        }
    }
}
