use serde_json::Value;

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
