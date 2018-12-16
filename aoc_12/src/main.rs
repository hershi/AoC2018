#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use regex::Regex;

type Pots = HashSet<isize>;

fn pots_from_str(s: &str) -> Pots {
    s.chars()
        .enumerate()
        .fold(HashSet::new(), |mut acc, (i,c)| {
            if c == '#' { acc.insert(i as isize); }
            acc
        })
}

type Rule = Vec<bool>;

fn rule_from_str(s: &str) -> Rule {
    s.chars()
        .map(|c| c == '#')
        .collect()
}

type Rules = HashSet<Rule>;

fn read_input() -> (Pots, Rules) {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    lazy_static! {
       static ref INIT_STATE: Regex = Regex::new(r"initial state: (\S*)").unwrap();
       static ref RULE: Regex = Regex::new(r"^(.....) => (.)").unwrap();
    }

    let lines : Vec<String> = reader.lines().flat_map(|line|line.ok()).collect();
    let init_state = lines
        .iter()
        .flat_map(|line| INIT_STATE
                  .captures(line)
                  .map(|cap| cap[1].to_string()))
        .last()
        .unwrap();

    let rules = lines
        .iter()
        .flat_map(|line| RULE.captures(line).map(|cap| if &cap[2] == "#" { Some(cap[1].to_string()) } else { None }))
        .flat_map(|s| s)
        .map(|s| rule_from_str(&s))
        .collect();

    (pots_from_str(&init_state), rules)
}

fn input_for(pots: &Pots, position: isize) -> Rule {
    (-2..3).map(|m|position+m)
        .map(|p|pots.contains(&p))
        .collect()
}

fn next_gen(pots: &Pots, rules: &Rules) -> Pots {
    let from = pots.iter().min().unwrap() - 2;
    let to = pots.iter().max().unwrap() + 2;

    (from..to+1)
        .flat_map(|p| if rules.contains(&input_for(pots, p)) { Some(p) } else { None } )
        .collect()
}

fn print_pots(pots: &Pots) {
    let from = pots.iter().min().unwrap() - 2;
    let to = pots.iter().max().unwrap() + 2;
    println!("Pots {}..{}", from, to);
    for i in from..to+1 {
        print!("{}", if pots.contains(&i) { '#' } else { '.' });
    }
    println!("");
}

fn main() {
    let d = 50000000000;
    let num_gens :isize = 1000;
    let (mut pots, rules) = read_input();
    print_pots(&pots);
    for i in 0..num_gens {
        if i % 1000 == 0 { println!("Gen {}", i); print_pots(&pots); }

        let new_pots = next_gen(&pots, &rules);
        if new_pots == pots { return; }
        pots = new_pots;
    }
    print_pots(&pots);

    println!("Result 1: {}", pots.iter().map(|x|x+d-num_gens).sum::<isize>());
}
