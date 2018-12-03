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

fn has_exactly_2_or_3(input: &str) -> (bool, bool) {
    let freq = get_freq(input);
    (freq.values().any(|&x| x == 2), freq.values().any(|&x| x == 3))
}

fn main() {
    let input = read_input();
    let res = input.clone()
        .iter()
        .map(|x| has_exactly_2_or_3(&x))
        .fold((0,0), |acc, x| (acc.0 + if x.0 {1} else {0}, acc.1 + if x.1 {1} else {0}));

    println!("Result {:?}", res.0 * res.1);

    'outer: for (i, x) in input.iter().enumerate() {
        for y in input.iter().skip(i) {
            if x.chars().zip(y.chars()).filter(|x| x.0 != x.1).count() == 1 {
                println!("{}:{} - diffs\n{}", x, y, x.chars().zip(y.chars()).filter(|x| x.0 == x.1).map(|x| x.0).collect::<String>());
                break 'outer;
            }
        }
    }
}
