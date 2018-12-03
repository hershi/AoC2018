use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

fn read_input() -> Vec<i32> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x| x.unwrap().trim().parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let input = read_input();
    let sum: i32 = input.iter().sum();

    let mut seen = HashSet::new();
    let mut current_freq = 0;
    for x in input.iter().cycle() {
        if !seen.insert(current_freq) { break; }
        current_freq += x;
    }

    println!("Input size : {}", input.len());
    println!("Result 1: {}", sum);
    println!("Result 2: {}", current_freq);
}
