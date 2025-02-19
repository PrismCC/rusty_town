use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Relationship {
    Stranger,
    Acquaintance,
    Friend,
    BestFriend,
    Rival,
    Colleague,
    Boss,
    Employee,
    Teacher,
    Student,
    Girlfriend,
    Boyfriend,
    Wife,
    Husband,
    Mum,
    Dad,
    Daughter,
    Son,
    Sister,
    Brother,
}

impl std::fmt::Display for Relationship {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Relationship::Stranger => write!(f, "Stranger"),
            Relationship::Acquaintance => write!(f, "Acquaintance"),
            Relationship::Friend => write!(f, "Friend"),
            Relationship::BestFriend => write!(f, "Best Friend"),
            Relationship::Rival => write!(f, "Rival"),
            Relationship::Colleague => write!(f, "Colleague"),
            Relationship::Boss => write!(f, "Boss"),
            Relationship::Employee => write!(f, "Employee"),
            Relationship::Teacher => write!(f, "Teacher"),
            Relationship::Student => write!(f, "Student"),
            Relationship::Girlfriend => write!(f, "Girlfriend"),
            Relationship::Boyfriend => write!(f, "Boyfriend"),
            Relationship::Wife => write!(f, "Wife"),
            Relationship::Husband => write!(f, "Husband"),
            Relationship::Mum => write!(f, "Mum"),
            Relationship::Dad => write!(f, "Dad"),
            Relationship::Daughter => write!(f, "Daughter"),
            Relationship::Son => write!(f, "Son"),
            Relationship::Sister => write!(f, "Sister"),
            Relationship::Brother => write!(f, "Brother"),
        }
    }
}