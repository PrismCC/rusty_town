use serde_json::{Value, json};

#[derive(Debug)]
pub struct Character {
    name: String,
    gender: String,
    age: u8,
    identity: String,
    residence: String,
 
    location: String,
    gold: u32,
    energy: u8, // 0-100
    health: u8, // 0-100
    hunger: u8, // 0-100
    mood: u8,   // 50: neutral, 0: sad, 100: happy
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
        &self.location
    }

    pub fn get_gold(&self) -> u32 {
        self.gold
    }

    pub fn get_energy(&self) -> u8 {
        self.energy
    }

    pub fn get_health(&self) -> u8 {
        self.health
    }

    pub fn get_hunger(&self) -> u8 {
        self.hunger
    }

    pub fn get_mood(&self) -> u8 {
        self.mood
    }

    pub fn set_location(&mut self, location: &str) {
        self.location =
            location.to_string();
    }

    pub fn set_gold(&mut self, gold: u32) {
        self.gold = gold;
    }

    pub fn set_energy(&mut self, energy: u8) {
        self.energy = energy;
    }

    pub fn set_health(&mut self, health: u8) {
        self.health = health;
    }

    pub fn set_hunger(&mut self, hunger: u8) {
        self.hunger = hunger;
    }

    pub fn set_mood(&mut self, mood: u8) {
        self.mood = mood;
    }

    pub fn read_character(path: &str) -> Character {
        let json_str = std::fs::read_to_string(path).expect("Unable to read file");
        let json_value: Value = serde_json::from_str(&json_str).expect("Unable to parse JSON");
        let character = Character {
            name: json_value["name"].as_str().unwrap().to_string(),
            gender: json_value["gender"].as_str().unwrap().to_string(),
            age: json_value["age"].as_u64().unwrap() as u8,
            identity: json_value["identity"].as_str().unwrap().to_string(),
            residence: json_value["residence"].as_str().unwrap().to_string(),
            location: "houses".to_string(),
            gold: 0,
            energy: 100,
            health: 100,
            hunger: 100,
            mood: 50,
        };
        character
    }

    pub fn read_status(&mut self, path: &str) {
        let json_str = std::fs::read_to_string(path).expect("Unable to read file");
        let json_value: Value = serde_json::from_str(&json_str).expect("Unable to parse JSON");
        self.location = json_value["location"].as_str().unwrap().to_string();
        self.gold = json_value["gold"].as_u64().unwrap() as u32;
        self.energy = json_value["energy"].as_u64().unwrap() as u8;
        self.health = json_value["health"].as_u64().unwrap() as u8;
        self.hunger = json_value["hunger"].as_u64().unwrap() as u8;
        self.mood = json_value["mood"].as_u64().unwrap() as u8;
    }

    pub fn write_status(&self, path: &str) {
        let json_value = json!({
            "location": self.location,
            "gold": self.gold,
            "energy": self.energy,
            "health": self.health,
            "hunger": self.hunger,
            "mood": self.mood,
        });
        let file = std::fs::File::create(path).expect("Unable to create file");
        serde_json::to_writer_pretty(file, &json_value).expect("Unable to write JSON");
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
            self.gender, self.age, 
            self.identity, self.residence, 
            self.location, 
            self.gold, 
            self.energy, self.health, self.hunger, self.mood
        )
    }
}
