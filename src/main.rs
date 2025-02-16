mod places;
mod characters;
use places::place::{Place, PlaceMap};
use places::town::Town;
use characters::character::Character;

fn main() {
    println!("Hello, Rusty Town!");
    let town = Town::read_town("resources/places/town.json");
    println!("{}", town);
    let place_list: Vec<Place> = town
        .get_places()
        .iter()
        .map(|place| Place::read_place(&format!("resources/places/{}.json", place)))
        .collect();
    let mut place_map = PlaceMap::new();
    for place in place_list {
        place_map.add_place(place);
    }
    println!("places: \n{:?}", place_map.get_places());
    println!("place: \n{}", place_map.get_place("school").unwrap());
    println!(
        "distance between school and library: {}",
        place_map.get_distance("school", "library").unwrap()
    );
    println!("");

    let mut alice = Character::read_character("resources/characters/Alice.json");
    alice.read_status("resources/characters/initial_status/Alice.json");
    println!("{}", alice);
}
