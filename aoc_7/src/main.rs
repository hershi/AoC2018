#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

fn read_input() -> Vec<(char, char)> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    lazy_static! {
       static ref LINE_RE: Regex = Regex::new(r"Step (\S*) must be finished before step (\S*) can begin.").unwrap();
    }

    reader.lines()
        .flat_map(|line|line.ok())
        .flat_map(|line|LINE_RE.captures(&line).map(|cap| (cap[1].chars().nth(0).unwrap(), cap[2].chars().nth(0).unwrap())))
        .collect()
}

type NodesSet = HashSet<char>;
type Edges = HashMap<char, NodesSet>;

// Transform the raw input into a graph representation.
// Return value is a tuple:
// 1st element is "Blocks" - associating each node with the set of
// nodes that it blocks directtly
// 2nd element is "Blocked By" - associating each node with the set
// of node that is directly blocking it
// 3rd element is a set of the nodes in the graph
fn init_graph(input: Vec<(char, char)>) -> (Edges, Edges, NodesSet) {
    let mut blocks = Edges::new();
    let mut blocked_by = Edges::new();

    for (blocker, blocked) in input.iter() {
        blocks.entry(*blocker).or_insert(HashSet::new()).insert(*blocked);
        blocked_by.entry(*blocked).or_insert(HashSet::new()).insert(*blocker);
    }

    let nodes = blocks.keys()
        .chain(blocked_by.keys())
        .map(|x|(*x).clone())
        .collect::<HashSet<char>>();

    (blocks, blocked_by, nodes)
}

fn find_ready(blocked_by: &Edges, nodes: NodesSet) -> NodesSet {
    nodes.into_iter()
        .filter(|node| blocked_by.get(node).map_or(0, |v|v.len()) == 0)
        .collect::<NodesSet>()
}

fn finish_procesing(node: char, blocks: &Edges, blocked_by: &mut Edges, ready: &mut NodesSet) {
    for blocked_node in blocks.get(&node).unwrap_or(&HashSet::new()) {
        if blocked_by
                .get_mut(blocked_node)
                .map_or(0, |v| {
                    v.remove(&node);
                    v.len()
                }) == 0 {
            ready.insert((*blocked_node).clone());
            blocked_by.remove(blocked_node);
        }
    }
}

fn part1(blocks: &Edges, mut blocked_by: Edges, nodes: NodesSet) {
    let mut ready = find_ready(&blocked_by, nodes);

    let mut result = Vec::new();
    while !ready.is_empty() {
        let current = ready.iter().min().unwrap().clone();
        ready.remove(&current);
        result.push(current);

        finish_procesing(current, &blocks, &mut blocked_by, &mut ready);
    }

    println!("Result: {}", result.iter().collect::<String>());
}

fn part2(blocks: &Edges, mut blocked_by: Edges, nodes: NodesSet) {
    let mut ready = find_ready(&blocked_by, nodes);

    let mut workers = vec![None; 5];
    let mut result = Vec::new();

    let base_time = 60;

    loop {
        // Exit condition - no nodes currently under processing and
        // no more ready nodes - no action can be taken so we must be done
        // (ideally we'll confirm all nodes were processed, but...)
        if ready.is_empty() && workers.iter().all(|x|x.is_none()) {
            break;
        }

        // Assign - if there are ready nodes and idle workers, then
        // assign them the next ready nodes
        if !ready.is_empty() && workers.iter().any(|x|x.is_none()) {
            let mut sorted_ready = ready.iter()
                .map(|x|*x)
                .collect::<Vec<char>>();
            sorted_ready.sort();

            for (node, worker) in sorted_ready.into_iter()
                .zip(workers.iter_mut().filter(|x|x.is_none())) {
                    let time = base_time + (node as u32 - 'A' as u32);
                    *worker = Some((node, time));
                    ready.remove(&node);
                    result.push(node);
                }
        }

        // Advance - advance the time until the next node becomes ready
        let next_step = workers.iter().flat_map(|x|x.map(|y|y.1)).min().unwrap();

        // Collect any finished work, updating 'ready' accordingly
        for item in workers.iter_mut().filter(|x|x.is_some()) {
            item.unwrap().1 -= next_step;
            if item.unwrap().1 == 0 {
                let node = item.unwrap().0;
                *item = None;
                finish_procesing(node, &blocks, &mut blocked_by, &mut ready);
            }
        }
    }

    println!("Result: {}", result.iter().collect::<String>());
}

fn main() {
    let input = read_input();
    let (blocks, blocked_by, nodes) = init_graph(input);
    println!("Nodes: {:?}", nodes);
    println!("Blockers: {:?}", blocks);
    println!("Blocked: {:?}", blocked_by);

    part1(&blocks, blocked_by.clone(), nodes.clone());
    //part2(&blocks, blocked_by, nodes);
}
