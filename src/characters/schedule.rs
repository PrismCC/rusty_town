use crate::time::{Time, Weekday};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub name: String,
    pub when: Time,
    pub place: String,
    pub activity: String,
    pub duration: Time,
    pub repeat: u8, // 0~7 bit: 0: Sunday, 1: Monday, 2: Tuesday, 3: Wednesday, 4: Thursday, 5: Friday, 6: Saturday, 7: Once
}

impl Transaction {
    pub fn new(
        name: String,
        when: Time,
        place: String,
        activity: String,
        duration: Time,
        repeat: u8,
    ) -> Transaction {
        Transaction {
            name,
            when,
            place,
            activity,
            duration,
            repeat,
        }
    }

    pub fn repeat_today(&self, weekday: Weekday) -> bool {
        let mask = 1 << weekday as u8;
        self.repeat & mask == mask
    }

    pub fn is_once(&self) -> bool {
        self.repeat & 0b1000 == 0b1000
    }
}

#[derive(Debug)]
pub struct Schedule {
    transactions: Vec<Transaction>,
    todo_list: Vec<Transaction>,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule {
            transactions: Vec::new(),
            todo_list: Vec::new(),
        }
    }

    pub fn read_transactions(&mut self, path: &str) {
        let schedule_file = std::fs::read_to_string(path).expect("Unable to read file");
        let schedule_file: ScheduleFile =
            serde_json::from_str(&schedule_file).expect("Unable to parse JSON");
        for raw_transaction in schedule_file.schedules {
            let transaction = Transaction::new(
                raw_transaction.name,
                Time::new(raw_transaction.when[0], raw_transaction.when[1], 0),
                raw_transaction.place,
                raw_transaction.activity,
                Time::new(raw_transaction.duration[0], raw_transaction.duration[1], 0),
                raw_transaction.repeat,
            );
            self.transactions.push(transaction);
        }
    }

    pub fn update_todo_list(&mut self, weekday: Weekday) {
        for transaction in &self.transactions {
            if transaction.repeat_today(weekday) {
                self.todo_list.push(transaction.clone());
            }
        }
        self.transactions
            .retain(|transaction| !transaction.is_once());
        self.todo_list.sort_by(|a, b| a.when.cmp(&b.when));
    }
}

#[derive(Debug, Deserialize)]
struct ScheduleFile {
    schedules: Vec<RawTransaction>,
}

#[derive(Debug, Deserialize)]
struct RawTransaction {
    name: String,
    when: [u8; 2],
    place: String,
    activity: String,
    duration: [u8; 2],
    repeat: u8,
}
