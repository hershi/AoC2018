#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn from_input(line: &str) -> Claim {
        lazy_static! {
           static ref RE: Regex = Regex::new(r"^\s*#(\d+)\s*@\s*(\d+),(\d+):\s*(\d+)x(\d+)\s*$").unwrap();
        }

        let captures = RE.captures(line).unwrap();
        Claim {
            id: captures[1].parse::<usize>().unwrap(),
            left: captures[2].parse::<usize>().unwrap(),
            top: captures[3].parse::<usize>().unwrap(),
            width: captures[4].parse::<usize>().unwrap(),
            height: captures[5].parse::<usize>().unwrap(),
        }
    }
}

fn read_input() -> Vec<Claim> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines().map(|x| Claim::from_input(&x.unwrap())).collect()
}

fn main() {
    let input = read_input();
    let mut fabric = HashMap::new();

    for claim in input.iter() {
        for i in (claim.left..).take(claim.width) {
            for j in (claim.top..).take(claim.height) {
                *fabric.entry((i,j)).or_insert(0) += 1;
            }
        }
    }

    println!("fabric size: {}", fabric.keys().count());
    println!("num with more than 1 {}", fabric.values().filter(|&&x| x > 1).count());
    println!("num with 1 {}", fabric.values().filter(|&&x| x == 1).count());

    'claims: for claim in input {
        for i in (claim.left..).take(claim.width) {
            for j in (claim.top..).take(claim.height) {
                if *fabric.get(&(i,j)).unwrap() > 1 { continue 'claims; }
            }
        }

        println!("Found claim {:?}", claim);
        break;
    }
}
