#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    fn new(year:&str, month:&str, day:&str) -> Date {
        Date {
            year: year.parse::<u16>().unwrap(),
            month: month.parse::<u8>().unwrap(),
            day: day.parse::<u8>().unwrap()
        }
    }
}


#[derive(Debug)]
struct GuardDutyRecord{
    guard_id: u32,
    start_date: Date,
    sleep_minutes: HashSet<u16>,
}

impl GuardDutyRecord {
    fn from_input(line: &str) -> Option<GuardDutyRecord> {
        lazy_static! {
           static ref SHIFT_START_RE: Regex = Regex::new(r"^\[(\d+)-(\d+)-(\d+).*\] Guard #(\d+) begins shift$").unwrap();
        }

        SHIFT_START_RE.captures(line).and_then(|captures| {
            Some(GuardDutyRecord {
                guard_id: captures[4].parse::<u32>().unwrap(),
                start_date: Date::new(&captures[1], &captures[2], &captures[3]),
                sleep_minutes: HashSet::new()
            })
        })
    }
}

fn read_input() -> Vec<GuardDutyRecord> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines().filter_map(|x| GuardDutyRecord::from_input(&x.unwrap())).collect()
}

fn main() {
    let records = read_input();
    println!("Records {:?}", records);
}
