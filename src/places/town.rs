use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Town {
    name: String,
    cname: String,
    description: String,
    places: Vec<String>,
}

impl Town {
    pub fn read_town(path: &str) -> Town {
        let town = std::fs::read_to_string(path).expect("Unable to read file");
        let town: Town = serde_json::from_str(&town).expect("Unable to parse JSON");
        town
    }

    pub fn get_places(&self) -> &Vec<String> {
        &self.places
    }
}

impl std::fmt::Display for Town {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {} \nplaces: {}\n", self.cname, self.description, self.places.join(", "))
    }
}
