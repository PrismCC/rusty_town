use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct RelationshipManager {
    likeability: HashMap<(String, String), i8>, // -100 to 100
    relationship: HashMap<(String, String), String>,
}

impl RelationshipManager {
    pub fn new() -> RelationshipManager {
        RelationshipManager {
            likeability: HashMap::new(),
            relationship: HashMap::new(),
        }
    }

    pub fn add_likeability(&mut self, from: &str, to: &str, likeability: i8) {
        let l = likeability.max(-100).min(100);
        self.likeability
            .insert((from.to_string(), to.to_string()), l);
    }

    pub fn get_likeability(&self, from: &str, to: &str) -> i8 {
        *self
            .likeability
            .get(&(from.to_string(), to.to_string()))
            .unwrap_or(&0)
    }

    pub fn update_likeability(&mut self, from: &str, to: &str, delta: i8) {
        let l = self
            .likeability
            .get(&(from.to_string(), to.to_string()))
            .unwrap_or(&0)
            + delta as i8;
        self.likeability
            .insert((from.to_string(), to.to_string()), l.max(-100).min(100));
    }

    pub fn add_relationship(&mut self, from: &str, to: &str, relationship: &str) {
        self.relationship
            .insert((from.to_string(), to.to_string()), relationship.to_string());
    }

    pub fn get_relationship(&self, from: &str, to: &str) -> String {
        self.relationship
            .get(&(from.to_string(), to.to_string()))
            .cloned()
            .unwrap_or("None".to_string())
    }

    pub fn update_relationship(&mut self, from: &str, to: &str, relationship: &str) {
        self.relationship
            .insert((from.to_string(), to.to_string()), relationship.to_string());
    }

    pub fn read_relationship(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).expect("Unable to read file");
        let v: Value = serde_json::from_str(&data).unwrap();
        let relations = v["relations"].as_array().unwrap();
        for relation in relations {
            let relation_array = relation.as_array().unwrap();
            let from = relation_array[0].as_str().unwrap();
            let to = relation_array[1].as_str().unwrap();
            let likeability = relation_array[2].as_i64().unwrap().clamp(-100, 100) as i8;
            let relationship = relation_array[3].as_str().unwrap();
            self.add_likeability(from, to, likeability);
            self.add_relationship(from, to, relationship);
        }
    }

    pub fn write_relationship(&self, path: &str) {
        let mut keys = HashSet::new();
        keys.extend(self.likeability.keys().cloned());
        keys.extend(self.relationship.keys().cloned());

        let mut relations: Vec<Value> = Vec::new();
        for (from, to) in keys {
            let likeability = self.get_likeability(&from, &to);
            let relationship = self.get_relationship(&from, &to);
            relations.push(Value::Array(vec![
                Value::String(from),
                Value::String(to),
                Value::Number(serde_json::Number::from(likeability)),
                Value::String(relationship),
            ]));
        }

        let v = Value::Object({
            let mut map = serde_json::Map::new();
            map.insert("relations".to_string(), Value::Array(relations));
            map
        });

        let file = std::fs::File::create(path).expect("Unable to create file");
        serde_json::to_writer_pretty(file, &v).expect("Unable to write file");
    }
}

impl std::fmt::Display for RelationshipManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut keys = HashSet::new();
        keys.extend(self.likeability.keys().cloned());
        keys.extend(self.relationship.keys().cloned());

        let mut s = String::new();
        for (from, to) in keys {
            let likeability = self.get_likeability(&from, &to);
            let relationship = self.get_relationship(&from, &to);
            s.push_str(&format!(
                "{:8} <- {:8}:\tlikeability: {:>4}\trelationship: {}\n",
                from, to, likeability, relationship
            ));
        }
        write!(f, "{}", s)
    }
}
