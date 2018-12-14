use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_input() -> Vec<usize> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .flat_map(|line|line.ok())
        .map(|line|
             line
                .split_whitespace()
                .flat_map(|s| s.parse::<usize>())
                .collect::<Vec<usize>>())
        .nth(0)
        .unwrap()
}

type MetadataEntry = usize;

#[derive(Debug)]
struct TreeNode {
    children: Vec<TreeNode>,
    metadata: Vec<MetadataEntry>,
}

fn build_tree(current: &mut std::slice::Iter<usize>) -> TreeNode {
    let num_children = current.next().unwrap();
    let num_metadata = current.next().unwrap();

    let mut children = Vec::with_capacity(*num_children);
    let mut metadata = Vec::with_capacity(*num_metadata);

    println!("New node: {} children {} MD", num_children, num_metadata);

    for _i in 0..*num_children {
        children.push(build_tree(current));
    }

    for _i in 0..*num_metadata {
        metadata.push(*current.next().unwrap());
    }

    TreeNode{children, metadata}
}

fn traverse(tree: &TreeNode) -> usize {
    tree.children.iter().map(|c| traverse(c)).sum::<usize>() +
        tree.metadata.iter().sum::<usize>()
}

fn traverse2(tree: &TreeNode) -> usize {
    if tree.children.len() == 0 {
        return tree.metadata.iter().sum::<usize>();
    }

    // We have children, so use 'metadata' as an indexer
    tree.metadata
        .iter()
        .flat_map(|idx| tree.children.get(idx-1))
        .map(|node|traverse2(node))
        .sum::<usize>()
}

fn main() {
    let input = read_input();
    println!("Input {:?}", input);

    let tree = build_tree(&mut input.iter());

    println!("Part 1: Tree {:?}", traverse(&tree));
    println!("Part 2: Tree {:?}", traverse2(&tree));
}
