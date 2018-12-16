#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate ncurses;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use std::ops::Add;
use std::ops::AddAssign;
use regex::Regex;
use ncurses::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pair {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Entry {
    position: Pair,
    velocity: Pair,
}

impl Pair {
    fn from_str(s: &str) -> Pair {
        let vals = s.split(',')
            .map(|x|x.trim())
            .map(|x|x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        Pair { x: vals[0], y: vals[1] }
    }
}

impl Add for Pair {
    type Output = Pair;

    fn add(self, other: Pair) -> Pair {
        Pair { x: self.x + other.x, y: self.y + other.y}
    }
}

impl AddAssign for Pair {
    fn add_assign(&mut self, other: Pair) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn read_input() -> Vec<Entry> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    lazy_static! {
       static ref LINE_RE: Regex = Regex::new(r"(<([^>]+)>).*(<([^>]+)>)").unwrap();
    }

    reader.lines()
        .flat_map(|line|line.ok())
        .flat_map(|line| LINE_RE.captures(&line)
                 .map(|cap| (Pair::from_str(&cap[2]), Pair::from_str(&cap[4]))))
        .map(|(position, velocity)| Entry { position, velocity })
        .collect()
}

fn print_output(coordinates: &Vec<Entry>, i: usize) {
    let row_limit = 20;
    let col_limit = 100;
    let min_row = coordinates.iter().map(|e|e.position.y).min().unwrap();
    let max_row = coordinates.iter().map(|e|e.position.y).max().unwrap();
    let min_col = coordinates.iter().map(|e|e.position.x).min().unwrap();
    let max_col = coordinates.iter().map(|e|e.position.x).max().unwrap();

    if (max_col - min_col + 1 > col_limit || max_row - min_row + 1 > row_limit) {
        return;
    }

    println!("{}: Min/Max Row: {}/{} ; Min/Max Col: {}/{}", i, min_row, max_row, min_col, max_col);
    println!("{}:{}", max_row - min_row + 1, max_col - min_col + 1);

    let coordinates = coordinates
        .iter()
        .map(|e| (e.position.x, e.position.y))
        .collect::<HashSet<(i32,i32)>>();

    for y in min_row..max_row+1 {
        for x in min_col..max_col+1 {
            print!("{}", if coordinates.contains(&(x,y)) { '#' } else { '.' });
        }
        println!("|");
    }

    println!("--------------------------\n");
}

fn main() {
    let mut input = read_input();

    for i in 1..12000 {
        input
            .iter_mut()
            .for_each(|ref mut e| e.position += e.velocity.clone());
        print_output(&input, i);
    }
}
