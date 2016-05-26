// Find every Friday the 13th
extern crate chrono;
use chrono::*;
use std::io;

struct Days {
    year: u32,
    month: u32,
}

impl Days {
    fn new(month: i32, year: i32) -> Self {
        Days {
            month: month as u32,
            year: year as u32,
        }
    }
}

fn main() {
    let day = read_input("What number day are you looking for?");
    assert!(day < 32);
    let this_year = Local::now().year();
    let year = read_input("What year to go back to?");
    assert!(year <= this_year as u32);
    let day_of_week = read_input_str("What day of the week are you looking for?");
    let day_of_week_str = str_to_weekday(day_of_week);
    let mother_day = Days::new(1, year as i32);

    for y in mother_day.year..this_year as u32 + 1 {
        // bar is exclusive, foo is inclusive
        for m in mother_day.month..13 {
            for d in 1..get_num_days(&m, &y) + 1 {
                if day == d {
                    let dt = UTC.ymd(y as i32, m, d);
                    if dt.weekday() == day_of_week_str {
                        println!("{}", dt.format("%B %e %Y"));
                    }
                }
            }
        }
    }
}

fn str_to_weekday(day: String) -> Weekday {
    // the trim is necessary here because technically our day would have a \n appended to it,
    // since it was taking input from terminal.
    match day.to_lowercase().trim() {
        "monday" => Weekday::Mon,
        "tuesday" => Weekday::Tue,
        "wednesday" => Weekday::Wed,
        "thursday" => Weekday::Thu,
        "friday" => Weekday::Fri,
        "saturday" => Weekday::Sat,
        "sunday" => Weekday::Sun,
        _ => panic!("Please enter a day of the week (case insensitive)"),
    }
}

fn get_num_days(month: &u32, year: &u32) -> u32 {
    match *month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => is_leap_year(year),
        _ => 30,
    }
}

fn is_leap_year(year: &u32) -> u32 {
    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        return 29;
    } else {
        return 28;
    }
}

fn read_input(question: &str) -> u32 {
    let mut input = String::new();
    println!("{}", question);
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("failed to read input");
    match input.trim().parse() {
        Ok(number) => {
            return number;
        }
        Err(_) => {
            panic!("Please enter a valid positive number");
        }
    }
}

fn read_input_str(question: &str) -> String {
    let mut input = String::new();
    println!("{}", question);
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("failed to read input");
    input
}