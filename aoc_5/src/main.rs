use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

fn read_input() -> String {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines().map(|x|x.unwrap()).filter(|x|!x.is_empty()).next().unwrap_or("".to_string())
}

fn react<'a, I>(input: I) -> String
    where I: Iterator<Item = char>
{
    let mut stack: Vec<char> = Vec::new();
    for c in input {
        if stack.last().map_or(false, |&x| x.to_ascii_uppercase() == c.to_ascii_uppercase() && x != c) {
            stack.pop();
            continue;
        }

        stack.push(c);
    }

    stack.into_iter().collect()
}

fn main() {
    let input = read_input();
    println!("Output size: {}", react(input.chars()).len());

    let unique_units =
        input.chars()
            .map(|x| x.to_ascii_uppercase())
            .fold(HashSet::new(), |mut acc, c| { acc.insert(c); acc });
    println!("Unique units: {:?}", unique_units);

    let min_item =
        unique_units.iter()
            .map(|&u| (u, react(input.chars().filter(|&c| c.to_ascii_uppercase() != u)).len()))
            .inspect(|(u,s)| println!("{},{}", u, s))
            .min_by_key(|(_,s)| *s)
            .unwrap();
    println!("Min item {:?}", min_item);
}
