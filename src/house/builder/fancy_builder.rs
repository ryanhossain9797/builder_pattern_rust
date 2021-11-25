use super::*;
pub struct FancyHouseBuilder {
    pub rooms: Vec<Room>,
    pub kitchen: Option<Kitchen>,
    pub bathrooms: Vec<Bathroom>,
    pub pool: Option<Pool>,
}

impl FancyHouseBuilder {
    pub fn new() -> Box<dyn HouseBuilderAwaitingRooms> {
        Box::new(Self {
            rooms: Vec::new(),
            kitchen: None,
            bathrooms: Vec::new(),
            pool: None,
        })
    }
}

impl HouseBuilderAwaitingRooms for FancyHouseBuilder {
    fn add_rooms_of_sizes(
        mut self: Box<Self>,
        room_sizes: Vec<i8>,
    ) -> Result<Box<dyn HouseBuilderAwaitingBathrooms>, String> {
        if !room_sizes.is_empty() {
            self.rooms.append(
                &mut room_sizes
                    .iter()
                    .map(|size| Room {
                        wall_material: WallMaterial::Brick,
                        floor_material: FloorMaterial::Wood,
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

impl HouseBuilderAwaitingBathrooms for FancyHouseBuilder {
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
                        wall_material: WallMaterial::Brick,
                        floor_material: FloorMaterial::Marble,
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

impl HouseBuilderAwaitingKitchen for FancyHouseBuilder {
    fn add_kitchen_of_size(mut self: Box<Self>, size: i8) -> Result<Box<dyn HouseBuilder>, String> {
        match size {
            0 => Err("Kitchen size can't be 0".to_string()),
            _ => {
                self.kitchen = Some(Kitchen {
                    wall_material: WallMaterial::Brick,
                    floor_material: FloorMaterial::Wood,
                    size,
                });
                Ok(self)
            }
        }
    }
}

impl HouseBuilder for FancyHouseBuilder {
    fn pool_of_size(mut self: Box<Self>, size: i8) -> Box<dyn HouseBuilder> {
        self.pool = Some(Pool { size });
        self
    }

    fn build(self: Box<Self>) -> House {
        House {
            material: "Wood".to_string(),
            rooms: self.rooms,
            bathrooms: self.bathrooms,
            kitchen: self.kitchen.unwrap(),
            pool: self.pool,
        }
    }
}
