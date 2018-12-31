#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::iter;
use std::cmp::max;
use std::cmp::min;
use regex::Regex;
use std::collections::HashMap;

type Coordinate = usize;

#[derive(Debug)]
struct Entry {
    top: Coordinate,
    left: Coordinate,
    bottom: Coordinate,
    right: Coordinate,
}

fn read_input() -> Vec<Entry> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    reader.lines().flat_map(|l|l.ok())
        .flat_map(|line| parse_line(&line))
        .collect()
}

impl Entry {
    fn new(top: Coordinate,
           left: Coordinate,
           bottom: Coordinate,
           right: Coordinate) -> Entry {
        Entry { top, left, bottom, right }
    }

    fn coordinates(&self) -> Vec<(Coordinate, Coordinate)> {
        (self.left..=self.right)
            .flat_map(|x|iter::repeat(x).take(self.bottom-self.top+1))
            .zip((self.top..=self.bottom).cycle())
            .collect::<Vec<(Coordinate, Coordinate)>>()
    }
}

fn parse_line(line: &str) -> Option<Entry> {
    lazy_static! {
       static ref X_RE: Regex = Regex::new(r"x=([\d\.]+)").unwrap();
       static ref Y_RE: Regex = Regex::new(r"y=([\d\.]+)").unwrap();
    }

    let x = X_RE.captures(line)
        .map(|captures|parse_value(&captures[1]));
    let y = Y_RE.captures(line)
        .map(|captures|parse_value(&captures[1]));

    x.and_then(|x_vals| y.map(|y_vals| Entry::new(y_vals.0, x_vals.0, y_vals.1, x_vals.1)))
}

fn parse_value(val_str: &str) -> (Coordinate, Coordinate) {
    lazy_static! {
       static ref RANGE_RE: Regex = Regex::new(r"(\d+)\.\.(\d+)").unwrap();
    }

    RANGE_RE.captures(val_str)
        .map_or_else(
            || { let v = val_str.parse::<Coordinate>().unwrap(); (v,v) },
            |captures| (captures[1].parse::<Coordinate>().unwrap(), captures[2].parse::<Coordinate>().unwrap()))
}

enum Tile {
    Empty,
    Clay,
    Water,
    Visited,
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Clay => '#',
            Tile::Water => '~',
            Tile::Visited => '|',
        }
    }
}

type Point = (Coordinate, Coordinate);

struct Map {
    y_range: (Coordinate, Coordinate),
    vals: HashMap<Point, Tile>,
}

impl Map {
    fn create(entries: Vec<Entry>) -> Map {
        let y_range = entries.iter()
            .fold((std::usize::MAX ,std::usize::MIN),
                  |acc, e| (min(acc.0, e.top), max(acc.1, e.bottom)));

        let mut vals = HashMap::new();
        entries.iter()
            .flat_map(|e|e.coordinates())
            .for_each(|(x,y)| { vals.insert((x,y),Tile::Clay); });

        Map {vals, y_range}
    }

    fn get(&self, p: &Point) -> &Tile {
        self.vals.get(p).unwrap_or(&Tile::Empty)
    }

    fn print(&self) {
        let x_range = self.vals.keys()
            .map(|&(x,_)|x)
            .fold((std::usize::MAX, std::usize::MIN),
                  |acc, x| (min(acc.0,x), max(acc.1, x)));

        for y in self.y_range.0..=self.y_range.1 {
            for x in x_range.0..=x_range.1 {
                print!("{}", self.vals.get(&(x,y)).map_or('.', |v| v.to_char()));
            }
            println!();
        }
    }
}

fn main() {
    let input = read_input();
    //println!("{:?}", input.iter().flat_map(|e|e.coordinates()).collect::<Vec<(Coordinate,Coordinate)>>());
    let map = Map::create(input);
    //println!("{:?}\n{:?}", map.vals, map.y_range);
    map.print();
}
