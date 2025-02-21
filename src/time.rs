use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Time {
    pub day: u16,
    pub hour: u8,
    pub minute: u8,
}

impl Time {
    pub fn new(hour: u8, minute: u8, day: u16) -> Time {
        Time { hour, minute, day }
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.day == 0 {
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
        let mut s = self;
        let mut o = other;
        if self.minute < other.minute {
            s.minute += 60;
            o.hour += 1;
        }
        if self.hour < other.hour {
            s.hour += 24;
            o.day += 1;
        }
        if s.day < o.day {
            panic!("Time subtraction resulted in negative time");
        }
        Time {
            day: s.day - o.day,
            hour: s.hour - o.hour,
            minute: s.minute - o.minute,
        }
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.day != other.day {
            self.day.cmp(&other.day)
        } else if self.hour != other.hour {
            self.hour.cmp(&other.hour)
        } else {
            self.minute.cmp(&other.minute)
        }
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
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
