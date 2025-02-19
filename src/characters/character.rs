use super::relation::Relationship;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterStatus {
    pub location: String,
    pub gold: u32,
    pub energy: u8,
    pub health: u8,
    pub hunger: u8,
    pub mood: u8,

    relations: HashMap<String, (Relationship, i8)>,
}

impl CharacterStatus {
    pub fn read_character_status(path: &str) -> CharacterStatus {
        let file = std::fs::File::open(path).expect("Unable to open file");
        serde_json::from_reader(file).expect("Unable to parse JSON")
    }

    pub fn write_character_status(&self, path: &str) {
        let file = std::fs::File::create(path).expect("Unable to create file");
        serde_json::to_writer_pretty(file, self).expect("Unable to write JSON");
    }
}

#[derive(Debug)]
pub struct Character {
    name: String,
    gender: String,
    age: u8,
    identity: String,
    residence: String,

    status: CharacterStatus,
}

impl Character {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_gender(&self) -> &str {
        &self.gender
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn get_identity(&self) -> &str {
        &self.identity
    }

    pub fn get_residence(&self) -> &str {
        &self.residence
    }

    pub fn get_location(&self) -> &str {
        &self.status.location
    }

    pub fn get_gold(&self) -> u32 {
        self.status.gold
    }

    pub fn get_energy(&self) -> u8 {
        self.status.energy
    }

    pub fn get_health(&self) -> u8 {
        self.status.health
    }

    pub fn get_hunger(&self) -> u8 {
        self.status.hunger
    }

    pub fn get_mood(&self) -> u8 {
        self.status.mood
    }

    pub fn set_location(&mut self, location: &str) {
        self.status.location = location.to_string();
    }

    pub fn set_gold(&mut self, gold: u32) {
        self.status.gold = gold;
    }

    pub fn set_energy(&mut self, energy: u8) {
        self.status.energy = energy;
    }

    pub fn set_health(&mut self, health: u8) {
        self.status.health = health;
    }

    pub fn set_hunger(&mut self, hunger: u8) {
        self.status.hunger = hunger;
    }

    pub fn set_mood(&mut self, mood: u8) {
        self.status.mood = mood;
    }

    pub fn get_likeability(&self, name: &str) -> i8 {
        match self.status.relations.get(name) {
            Some((_, likeability)) => (*likeability).min(100).max(-100),
            None => 0,
        }
    }

    pub fn set_likeability(&mut self, name: &str, likeability: i8) {
        match self.status.relations.get_mut(name) {
            Some((_, old_likeability)) => {
                *old_likeability = likeability.min(100).max(-100);
            }
            None => {
                self.status
                    .relations
                    .insert(name.to_string(), (Relationship::Stranger, likeability));
            }
        }
    }

    pub fn get_relationship(&self, name: &str) -> Relationship {
        match self.status.relations.get(name) {
            Some((relationship, _)) => (*relationship).clone(),
            None => Relationship::Stranger,
        }
    }

    pub fn set_relationship(&mut self, name: &str, relationship: Relationship) {
        match self.status.relations.get_mut(name) {
            Some((old_relationship, _)) => {
                *old_relationship = relationship;
            }
            None => {
                self.status
                    .relations
                    .insert(name.to_string(), (relationship, 0));
            }
        }
    }

    pub fn read_character(name: &str) -> Character {
        let character_path = format!("resources/characters/{}.json", name);
        let initial_status_path = format!("resources/characters/initial_status/{}.json", name);

        let status = CharacterStatus::read_character_status(&initial_status_path);

        let json_str = std::fs::read_to_string(character_path).expect("Unable to read file");
        let json_value: Value = serde_json::from_str(&json_str).expect("Unable to parse JSON");
        let character = Character {
            name: json_value["name"].as_str().unwrap().to_string(),
            gender: json_value["gender"].as_str().unwrap().to_string(),
            age: json_value["age"].as_u64().unwrap() as u8,
            identity: json_value["identity"].as_str().unwrap().to_string(),
            residence: json_value["residence"].as_str().unwrap().to_string(),
            status,
        };
        character
    }

    pub fn write_character(&self, status_path: &str) {
        self.status.write_character_status(status_path);
    }
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}:\n\
            gender: {}  age: {}\n\
            identity: {}  residence: {}\n\
            location: {}\n\
            gold: {}\n\
            energy: {}  health: {}  hunger: {}  mood: {}\n",
            self.name,
            self.gender,
            self.age,
            self.identity,
            self.residence,
            self.status.location,
            self.status.gold,
            self.status.energy,
            self.status.health,
            self.status.hunger,
            self.status.mood
        )
    }
}
