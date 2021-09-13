use std::fmt::Display;

pub struct Pool {
    pub size: i8,
}

pub struct Kitchen {
    pub size: i8,
}

pub struct Bathroom {
    pub size: i8,
}

pub struct Room {
    pub size: i8,
}

pub struct House {
    pub material: String,
    pub rooms: Vec<Room>,
    pub kitchen: Kitchen,
    pub pool: Option<Pool>,
    pub bathrooms: Vec<Bathroom>,
}

impl Display for House {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = format!(
            "A house made of {} with {} room(s), {} bathroom(s) and {}",
            self.material,
            self.rooms.len(),
            self.bathrooms.len(),
            match &self.pool {
                Some(_) => "with a pool",
                _ => "without a pool",
            }
        );
        write!(f, "{}", message)
    }
}
