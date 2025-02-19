use std::ops::{Add, Sub};

pub struct Time {
    pub day: u32,
    pub hour: u8,
    pub minute: u8,
}

impl Time {
    pub fn new(hour: u8, minute: u8) -> Time {
        Time {
            hour,
            minute,
            day: 0,
        }
    }

    pub fn new(day: u32, hour: u8, minute: u8) -> Time {
        Time { hour, minute, day }
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if (self.day == 0) {
            write!(f, "{:02}:{:02}", self.hour, self.minute)
        } else {
            write!(f, "{}d {:02}:{:02}", self.day, self.hour, self.minute)
        }
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        let mut day = self.day;
        let mut hour = self.hour + other.hour;
        let mut minute = self.minute + other.minute;
        if minute >= 60 {
            hour += 1;
            minute -= 60;
        }
        if hour >= 24 {
            day += 1;
            hour -= 24;
        }
        day += other.day;
        Time { day, hour, minute }
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        let mut day = self.day;
        let mut hour = self.hour - other.hour;
        let mut minute = self.minute - other.minute;
        if minute < 0 {
            hour -= 1;
            minute += 60;
        }
        if hour < 0 {
            day -= 1;
            hour += 24;
        }
        day -= other.day;
        Time { day, hour, minute }
    }
}

#[derive(Debug)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug)]
pub struct TimeManager {
    pub time: Time,
    pub weekday: Weekday,
    update_interval: Time,
}

impl TimeManager {
    pub fn new(time: Time, weekday: Weekday, update_interval: Time) -> TimeManager {
        TimeManager {
            time,
            weekday,
            update_interval,
        }
    }

    pub fn update(&mut self) {
        let last_day = self.time.day;
        self.time = self.time + self.update_interval;

        if last_day != self.time.day {
            self.weekday = match self.weekday {
                Weekday::Sunday => Weekday::Monday,
                Weekday::Monday => Weekday::Tuesday,
                Weekday::Tuesday => Weekday::Wednesday,
                Weekday::Wednesday => Weekday::Thursday,
                Weekday::Thursday => Weekday::Friday,
                Weekday::Friday => Weekday::Saturday,
                Weekday::Saturday => Weekday::Sunday,
            };
        }
    }

    pub fn get_time(&self) -> Time {
        self.time
    }

    pub fn get_weekday(&self) -> Weekday {
        self.weekday
    }
}
