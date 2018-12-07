#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate chrono;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;
use chrono::*;

#[derive(Debug)]
enum InputRecord {
    ShiftStart{guard_id: u32, date: NaiveDateTime},
    FallAsleep{date: NaiveDateTime},
    WakeUp{date: NaiveDateTime},
}

impl InputRecord {
    fn from_input(line: &str) -> Option<InputRecord> {
        lazy_static! {
           static ref DATE_RE: Regex = Regex::new(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\]").unwrap();
           static ref SHIFT_START: Regex = Regex::new(r"Guard #(\d+) begins shift$").unwrap();
           static ref FALL_ASLEEP: Regex = Regex::new(r"falls asleep").unwrap();
           static ref WAKE_UP: Regex = Regex::new(r"wakes up").unwrap();
        }

        let captures = DATE_RE.captures(line);
        let date = captures.map(
            |captures| NaiveDate::from_ymd(
                captures[1].parse::<i32>().unwrap(),
                captures[2].parse::<u32>().unwrap(),
                captures[3].parse::<u32>().unwrap())
            .and_hms(
                captures[4].parse::<u32>().unwrap(),
                captures[5].parse::<u32>().unwrap(),
                0));

        if date.is_none() {
            return None;
        }

        let date = date.unwrap();
        let captures = SHIFT_START.captures(line);
        if captures.is_some() {
            return Some(InputRecord::ShiftStart{ guard_id: captures.unwrap()[1].parse::<u32>().unwrap(), date });
        }

        if FALL_ASLEEP.is_match(line) {
            return Some(InputRecord::FallAsleep{ date });
        }
        if WAKE_UP.is_match(line) {
            return Some(InputRecord::WakeUp{ date });
        }

        None
    }
}

fn read_input() -> Vec<InputRecord> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let mut lines = reader.lines().map(|x|x.unwrap()).collect::<Vec<String>>();
    lines.as_mut_slice().sort();
    lines.iter().filter_map(|x| InputRecord::from_input(&x)).collect()
}

#[derive(Debug)]
struct ShiftStats {
    guard_id: u32,
    minutes: HashSet<u32>,
}

impl ShiftStats {
    fn new(id: u32) -> ShiftStats {
        ShiftStats { guard_id: id, minutes: HashSet::new() }
    }

    fn update(&mut self, sleep: &NaiveDateTime, wake: &NaiveDateTime) {
        for min in sleep.time().minute()..wake.time().minute() {
            self.minutes.insert(min);
        }
    }
}


#[derive(Debug)]
struct GuardStats {
    minute_counts: HashMap<u32, u32>,
}

impl GuardStats {
    fn new() -> GuardStats {
        GuardStats { minute_counts: HashMap::new() }
    }

    fn update(&mut self, shift: &ShiftStats) {
        for i in shift.minutes.iter() {
            *self.minute_counts.entry(*i).or_insert(0) += 1;
        }
    }

    fn total(&self) -> u32 {
        self.minute_counts.values().sum()
    }
}

fn main() {
    let records = read_input();

    let mut stats : HashMap<u32, GuardStats> = HashMap::new();

    let mut current_shift = ShiftStats::new(0);

    let mut i = records.iter();
    loop {
        let record = i.next();
        if record.is_none(){ break; }

        let record = record.unwrap();

        match record {
            InputRecord::ShiftStart{guard_id, date} => {
                stats.entry(current_shift.guard_id).or_insert(GuardStats::new()).update(&current_shift);
                current_shift = ShiftStats::new(*guard_id);
            }
            InputRecord::FallAsleep{date:sleep} => {
                let wake = i.next().unwrap();
                if let InputRecord::WakeUp{date:wake} = wake {
                    current_shift.update(sleep, wake);
                }
            }
            _ => panic!("Unexpected type")
        }
    }

    stats.entry(current_shift.guard_id).or_insert(GuardStats::new()).update(&current_shift);

    //for (k,v) in stats {
        //println!("ID: {:?} Total: {:?}", k, v.total());
    //}

    let guard = stats.iter()
        .max_by_key(|(k,v)| v.total())
        .map(|(k,v)| (k, v.minute_counts.iter().max_by_key(|(min,freq)|*freq)));
    println!("{:?} {}", guard,  guard.unwrap().0 * guard.unwrap().1.unwrap().0);

    let guard =
        stats
            .iter()
            .max_by_key(|(k,v)| v.minute_counts.values().max());

    let id = guard.unwrap().0;
    let min = guard.unwrap().1.minute_counts.iter().max_by_key(|(k,v)|*v).unwrap().0;
    println!("ID {} min {} result {}", id, min, id * min);
}
