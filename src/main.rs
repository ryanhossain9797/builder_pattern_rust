// #![warn(clippy::pedantic)]

mod house;

use house::{build_director::*, stone_builder::*, wood_builder::*, *};

fn main() {
    build_manual_house();
    build_fancy_house();
    build_oldie_house();
}

fn build_manual_house() {
    let manual_house = Box::new(WoodHouseBuilder::new())
        .add_rooms(vec![
            Room { size: 10 },
            Room { size: 10 },
            Room { size: 10 },
        ])
        .add_bathrooms(vec![Bathroom { size: 10 }])
        .pool(Pool { size: 30 })
        .build();

    match manual_house {
        Ok(house) => println!("{}", house),
        Err(err) => println!("{}", err),
    }
}

fn build_fancy_house() {
    let fancy_house =
        HouseBuildDirector::new(Box::new(WoodHouseBuilder::new())).build_fancy_house();

    match fancy_house {
        Ok(house) => println!("{}", house),
        Err(err) => println!("{}", err),
    }
}

fn build_oldie_house() {
    let oldie_house =
        HouseBuildDirector::new(Box::new(StoneHouseBuilder::new())).build_basic_house();

    match oldie_house {
        Ok(house) => println!("{}", house),
        Err(err) => println!("{}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manual_house() {
        build_manual_house();
    }

    #[test]
    fn test_fancy_house() {
        build_fancy_house();
    }

    #[test]
    fn test_oldie_house() {
        build_oldie_house();
    }
}
