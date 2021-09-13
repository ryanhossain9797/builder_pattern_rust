struct Pool {
    pub size: i8,
}
struct Bathroom {
    pub size: i8,
}
struct Room {
    pub size: i8,
}
struct House {
    pub rooms: Vec<Room>,
    pub pool: Option<Pool>,
    pub bathrooms: Vec<Bathroom>,
}

impl House {
    fn builder() -> RealHouseBuilder {
        RealHouseBuilder::new()
    }
}

struct RealHouseBuilder {
    pub rooms: Vec<Room>,
    pub pool: Option<Pool>,
    pub bathrooms: Vec<Bathroom>,
}

impl RealHouseBuilder {
    fn new() -> Self {
        Self {
            rooms: Vec::new(),
            pool: None,
            bathrooms: Vec::new(),
        }
    }

    fn add_room(mut self, room: Room) -> RealHouseBuilder {
        self.rooms.push(room);
        self
    }

    fn pool(mut self, pool: Pool) -> RealHouseBuilder {
        self.pool = Some(pool);
        self
    }

    fn add_bathroom(mut self, bathroom: Bathroom) -> RealHouseBuilder {
        self.bathrooms.push(bathroom);
        self
    }

    fn build(self) -> Result<House, String> {
        Ok(House {
            rooms: self.rooms,
            pool: self.pool,
            bathrooms: self.bathrooms,
        })
    }
}

struct ToyHouseBuilder {
    pub rooms: Vec<Room>,
    pub pool: Option<Pool>,
    pub bathrooms: Vec<Bathroom>,
}

impl ToyHouseBuilder {
    fn new() -> Self {
        Self {
            rooms: Vec::new(),
            pool: None,
            bathrooms: Vec::new(),
        }
    }

    fn add_room(mut self, room: Room) -> ToyHouseBuilder {
        self.rooms.push(room);
        self
    }

    fn pool(mut self, pool: Pool) -> ToyHouseBuilder {
        self.pool = Some(pool);
        self
    }

    fn add_bathroom(mut self, bathroom: Bathroom) -> ToyHouseBuilder {
        self.bathrooms.push(bathroom);
        self
    }

    fn build(self) -> Result<House, String> {
        Ok(House {
            rooms: self.rooms,
            pool: self.pool,
            bathrooms: self.bathrooms,
        })
    }
}

struct HouseBuildDirector {
    builder: RealHouseBuilder,
}

impl HouseBuildDirector {
    fn new(builder: RealHouseBuilder) -> Self {
        Self { builder }
    }

    fn build_pool_house(self) -> Result<House, String> {
        self.builder
            .add_bathroom(Bathroom { size: 10 })
            .add_room(Room { size: 30 })
            .pool(Pool { size: 0 })
            .build()
    }
}

fn main() {
    let house = HouseBuildDirector::new(House::builder()).build_pool_house();
}
