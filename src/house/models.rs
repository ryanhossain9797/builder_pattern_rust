pub enum WallMaterial {
    Brick,
    Stone,
}

pub enum FloorMaterial {
    Wood,
    Marble,
    Stone,
}

pub struct Pool {
    pub size: i8,
}

pub struct Kitchen {
    pub floor_material: FloorMaterial,
    pub wall_material: WallMaterial,
    pub size: i8,
}

pub struct Bathroom {
    pub floor_material: FloorMaterial,
    pub wall_material: WallMaterial,
    pub size: i8,
}

pub struct Room {
    pub floor_material: FloorMaterial,
    pub wall_material: WallMaterial,
    pub size: i8,
}
