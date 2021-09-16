// #![warn(clippy::pedantic)]

mod house;

use house::{build_director::*, fancy_builder::*, stone_builder::*, *};

#[async_std::main]
async fn main() {
    build_manual_house();
    build_fancy_house();
    build_oldie_house();
    build_fancy_and_oldie_house();
}

fn build_manual_house() {
    let manual_house = Box::new(FancyHouseBuilder::new())
        .add_rooms_of_sizes(vec![10, 10, 10])
        .add_bathrooms_of_sizes(vec![10])
        .pool_of_size(30)
        .build();

    match manual_house {
        Ok(house) => println!("{}", house),
        Err(err) => println!("{}", err),
    }
}

fn build_fancy_house() {
    let mut director = HouseBuildDirector::new();
    let builder = Box::new(FancyHouseBuilder::new());
    director.update_builder(builder);

    let fancy_house = director.build_fancy_house();

    match fancy_house {
        Ok(house) => println!("{}", house),
        Err(err) => println!("{}", err),
    }
}

fn build_oldie_house() {
    let mut director = HouseBuildDirector::new();
    let builder = Box::new(StoneHouseBuilder::new());
    director.update_builder(builder);

    let oldie_house = director.build_basic_house();

    match oldie_house {
        Ok(house) => println!("{}", house),
        Err(err) => println!("{}", err),
    }
}

fn build_fancy_and_oldie_house() {
    let mut director = HouseBuildDirector::new();
    let builder = Box::new(FancyHouseBuilder::new());
    director.update_builder(builder);

    let fancy_house = director.build_fancy_house();

    match fancy_house {
        Ok(house) => println!("{}", house),
        Err(err) => println!("{}", err),
    }

    let builder = Box::new(StoneHouseBuilder::new());
    director.update_builder(builder);

    let oldie_house = director.build_basic_house();

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
