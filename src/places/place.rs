use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Place {
    name: String,
    cname: String,
    description: String,
    activities: Vec<String>,
    distances: HashMap<String, i32>,
}

impl Place {
    pub fn read_place(path: &str) -> Place {
        let place = std::fs::read_to_string(path).expect("Unable to read file");
        let place: Place = serde_json::from_str(&place).expect("Unable to parse JSON");
        place
    }

    pub fn get_distance(&self, place: &str) -> Option<i32> {
        self.distances.get(place).copied()
    }

    pub fn get_activities(&self) -> &Vec<String> {
        &self.activities
    }

    pub fn get_neighbours(&self) -> Vec<String> {
        self.distances.keys().cloned().collect()
    }
}

impl std::fmt::Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}\nactivities: {}\nneighbors: {}\n",
            self.cname,
            self.description,
            self.get_activities().join(", "),
            self.get_neighbours().join(", ")
        )
    }
}


pub struct PlaceMap {
    map: HashMap<String, Place>,
}

impl PlaceMap {
    pub fn new() -> PlaceMap {
        PlaceMap {
            map: HashMap::new(),
        }
    }

    pub fn add_place(&mut self, place: Place) {
        self.map.insert(place.name.clone(), place);
    }

    pub fn get_place(&self, place: &str) -> Option<&Place> {
        self.map.get(place)
    }

    pub fn get_places(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    pub fn get_distance(&self, place1: &str, place2: &str) -> Option<i32> {
        self.map.get(place1).and_then(|place| place.get_distance(place2))
    }
}