// You are given the following information, but you may prefer to do some research for
// yourself.

// 1 Jan 1900 was a Monday.
// Thirty days has September,
// April, June and November.
// All the rest have thirty-one,
// Saving February alone,
// Which has twenty-eight, rain or shine.
// And on leap years, twenty-nine.
//
// A leap year occurs on any year evenly divisible by 4, but not on a century unless it
// is divisible by 400.
//
// How many Sundays fell on the first of the month during the twentieth century (1 Jan
// 1901 to 31 Dec 2000)?

use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum DayName {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug)]
struct Tracker {
    day_name: DayName,
    day: usize,
    month: usize,
    year: usize,

    day_map: HashMap<DayName, DayName>,
}

impl Tracker {
    fn new() -> Self {
        Self {
            day_name: DayName::Monday,
            day: 1,
            month: 1,
            year: 1900,
            day_map: HashMap::from([
                (DayName::Monday, DayName::Tuesday),
                (DayName::Tuesday, DayName::Wednesday),
                (DayName::Wednesday, DayName::Thursday),
                (DayName::Thursday, DayName::Friday),
                (DayName::Friday, DayName::Saturday),
                (DayName::Saturday, DayName::Sunday),
                (DayName::Sunday, DayName::Monday),
            ]),
        }
    }

    fn advance(&mut self) {
        self.day_name = *(self.day_map.get(&self.day_name).unwrap());
        self.day += 1;
        // TODO: Handle leap years somehow
        (self.month, self.day) = match (self.month, self.day) {
            (1, 32) => (2, 1),
            (2, 29) => check_for_leap_year(self.year),
            (2, 30) => (3, 1),
            (3, 32) => (4, 1),
            (4, 31) => (5, 1),
            (5, 32) => (6, 1),
            (6, 31) => (7, 1),
            (7, 32) => (8, 1),
            (8, 32) => (9, 1),
            (9, 31) => (10, 1),
            (10, 32) => (11, 1),
            (11, 31) => (12, 1),
            (12, 32) => (1, 1),
            _ => (self.month, self.day),
        };

        if self.day == 1 && self.month == 1 {
            self.year += 1;
        }
    }
}

fn check_for_leap_year(year: usize) -> (usize, usize) {
    // A leap year occurs on any year evenly divisible by 4, but not on a century unless it
    // is divisible by 400.
    if year % 4 == 0 {
        if year % 400 == 0 {
            return (2, 29);
        } else if year % 100 == 0 {
            return (3, 1);
        } else {
            return (2, 29);
        }
    }
    (3, 1)
}

#[test]
fn test_tracker_new() {
    let tracker = Tracker::new();
    assert_eq!(tracker.day, 1);
    assert_eq!(tracker.month, 1);
    assert_eq!(tracker.year, 1900);
    assert_eq!(tracker.day_name, DayName::Monday);
}

#[test]
fn test_tracker_advance_day() {
    let mut tracker = Tracker::new();
    tracker.advance();
    assert_eq!(tracker.day, 2);
    assert_eq!(tracker.month, 1);
    assert_eq!(tracker.year, 1900);
    assert_eq!(tracker.day_name, DayName::Tuesday);
}

#[test]
fn test_tracker_advance_month() {
    let mut tracker = Tracker::new();
    for _ in 0..31 {
        tracker.advance();
    }
    assert_eq!(tracker.day, 1);
    assert_eq!(tracker.month, 2);
    assert_eq!(tracker.year, 1900);
    assert_eq!(tracker.day_name, DayName::Thursday);
}

#[test]
fn test_tracker_advance_year() {
    let mut tracker = Tracker::new();
    for _ in 0..365 {
        tracker.advance();
    }
    assert_eq!(tracker.day, 1);
    assert_eq!(tracker.month, 1);
    assert_eq!(tracker.year, 1901);
    assert_eq!(tracker.day_name, DayName::Tuesday);
}

#[test]
fn test_tracker_advance_non_leap_year() {
    let mut tracker = Tracker::new();
    // Advance 4 years and
    for _ in 0..59 {
        tracker.advance();
    }
    assert_eq!(tracker.day, 1);
    assert_eq!(tracker.month, 3);
    assert_eq!(tracker.year, 1900);
    assert_eq!(tracker.day_name, DayName::Thursday);

    for _ in 0..365 {
        tracker.advance();
    }
    assert_eq!(tracker.day, 1);
    assert_eq!(tracker.month, 3);
    assert_eq!(tracker.year, 1901);
    assert_eq!(tracker.day_name, DayName::Friday);
}

#[test]
fn test_tracker_advance_leap_year() {
    let mut tracker = Tracker::new();
    // Advance 4 years and
    for _ in 0..(365 * 4 + 59) {
        tracker.advance();
    }
    assert_eq!(tracker.day, 29);
    assert_eq!(tracker.month, 2);
    assert_eq!(tracker.year, 1904);
    assert_eq!(tracker.day_name, DayName::Monday);
}

fn main() {
    let start = Instant::now();

    // Initialize tracker
    let mut tracker = Tracker::new();

    // Advance tracker to 1-1-1901
    for _ in 0..365 {
        tracker.advance();
    }

    let mut sunday_count = 0;
    loop {
        // Ask tracker to advance 1 day at a time
        tracker.advance();

        if tracker.year > 2000 {
            break;
        }

        if tracker.day == 1 && tracker.day_name == DayName::Sunday {
            sunday_count += 1;
        }
    }
    println!("Number of Sundays that fall on the first of a month: {sunday_count}");
    println!("Time to complete: {}", start.elapsed().as_secs_f64());
}
