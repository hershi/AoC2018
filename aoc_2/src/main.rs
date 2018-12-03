use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

fn read_input() -> Vec<String> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines().map(|x| x.unwrap()).collect()
}

fn get_freq(input: &str) -> HashMap<char, i32> {
    input.chars().fold(
        HashMap::new(),
        |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
}

fn main() {
    let input = read_input();
    let input_freq = input.iter().map(|s| get_freq(&s)).collect::<Vec<HashMap<char, i32>>>();
    let exactly_two : i32 =
        input_freq.iter().map(|freq| if freq.values().any(|&x| x == 2) { 1 } else {0}).sum();
    let exactly_three : i32 =
        input_freq.iter().map(|freq| if freq.values().any(|&x| x == 3) { 1 } else {0}).sum();

    println!("Result {:?}", exactly_two * exactly_three);

    'outer: for (i, x) in input.iter().enumerate() {
        for y in input.iter().skip(i) {
            if x.chars().zip(y.chars()).filter(|x| x.0 != x.1).count() == 1 {
                println!("{}:{} - diffs\n{}", x, y, x.chars().zip(y.chars()).filter(|x| x.0 == x.1).map(|x| x.0).collect::<String>());
                break 'outer;
            }
        }
    }
}
