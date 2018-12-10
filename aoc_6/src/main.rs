use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;
//use std::collections::HashMap;

fn read_input() -> Vec<(isize, isize)> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .flat_map(|line|line.ok())
        .map(|line|line
             .split(',')
             .map(|x|x.trim().parse::<isize>().unwrap())
             .collect())
        .flat_map(|v : Vec<isize>| if v.len() == 2 { Some((v[0], v[1])) } else { None })
        .collect()
}

#[derive(Debug)]
struct ProcessingEntry {
    ordinal: i32,
    gen: i32,
    coordinates: (isize, isize),
}

impl ProcessingEntry {
    fn new(ordinal: i32, gen: i32, coordinates: (isize, isize)) -> ProcessingEntry {
        ProcessingEntry { ordinal, gen, coordinates }
    }
}


#[derive(Debug)]
struct Grid {
    width: isize,
    height: isize,

    // First value is the coordinate ordinal to which it belongs
    // Second value is the 'generation' in which it was set
    grid: Vec<(i32, i32)>,
}

impl Grid {
    fn new(width: isize, height: isize) -> Grid {
        Grid { width: width as isize, height: height as isize, grid: vec![(-1, -1); (width * height) as usize] }
    }

    fn set(&mut self, entry: &ProcessingEntry) {
        if !self.in_bounds(entry.coordinates) { return; }

        let index = self.calculate_index(entry.coordinates);
        // Clear spot
        if self.grid[index].1 == -1 {
            self.grid[index] = (entry.ordinal, entry.gen);
            return;
        }

        // Already visited from the same source point - stop traversing
        if self.grid[index].0 == entry.ordinal { return; }

        // Already visited from another source. If it was visited on the
        // same generation, then mark it as a contention point
        if self.grid[index].1 == entry.gen { self.grid[index].0 = -1 }
    }

    fn get(&self, coordinates: (isize,isize)) -> (i32, i32) {
        self.grid[self.calculate_index(coordinates)]
    }

    fn in_bounds(&self, coordinates: (isize,isize)) -> bool {
        let y = coordinates.1;
        let x = coordinates.0;
        y >= 0 && y < self.height && x >= 0 && x < self.width
    }

    fn calculate_index(&self, coordinates: (isize, isize)) -> usize {
        (self.width * coordinates.1 + coordinates.0) as usize
    }

    fn print(&self) {
        for i in 0..self.height as usize{
            for j in 0..self.width as usize {
                print!("{:4}", self.grid[self.calculate_index((j as isize,i as isize))].0);
            }
            println!("");
            //println!("{:?}###",
                     //self.grid.iter()
                        //.skip(self.width as usize *i)
                        //.take(self.width as usize)
                        //.map(|(x,y)|(*x))
                        //.collect::<Vec<(i32)>>());
        }
    }
}

fn part_1(input: &Vec<(isize, isize)>) {
    let max_x = input.iter().map(|(x,_)|x).max().unwrap();
    let max_y = input.iter().map(|(_,y)|y).max().unwrap();

    println!("Max cooridnates: x {} y {}", max_x, max_y);

    let mut grid = Grid::new(max_x + 2, max_y + 1);

    let mut infinite = HashSet::new();
    infinite.insert(-1);

    let mut stacks = vec![vec![], vec![]];
    let mut current_gen = 0;
    for (i, coordinates) in input.iter().enumerate() {
        stacks[0].push(ProcessingEntry::new(i as i32, 0, *coordinates));
    }

    loop {
        let current_gen_index = current_gen % 2;
        if stacks[current_gen_index].is_empty() { break; }

        println!("Processing gen {} - num entries {}", current_gen, stacks[current_gen_index].len());

        let next_gen_index = (current_gen + 1) % 2;

        stacks[next_gen_index].clear();

        for entry in stacks[current_gen_index].iter() {
            if !grid.in_bounds(entry.coordinates) {
                if infinite.insert(entry.ordinal) {
                    println!("Adding {:?} to infinite", entry);
                }
                continue;
            }

            //println!("Setting {:?} on grid", entry);
            grid.set(entry);
        }

        let mut next_gen = vec![];
        for entry in stacks[current_gen_index].iter()
                .filter(|e| grid.in_bounds(e.coordinates))
                .filter(|e| grid.get(e.coordinates).1 == current_gen as i32) {
            let ordinal = grid.get(entry.coordinates).0;
            next_gen.push(ProcessingEntry::new(ordinal, entry.gen + 1, (entry.coordinates.0 + 1, entry.coordinates.1)));
            next_gen.push(ProcessingEntry::new(ordinal, entry.gen + 1, (entry.coordinates.0 - 1, entry.coordinates.1)));
            next_gen.push(ProcessingEntry::new(ordinal, entry.gen + 1, (entry.coordinates.0, entry.coordinates.1 + 1)));
            next_gen.push(ProcessingEntry::new(ordinal, entry.gen + 1, (entry.coordinates.0, entry.coordinates.1 - 1)));
        }
        stacks[next_gen_index] = next_gen;

        current_gen += 1;
        //grid.print();
        //println!("-------------------------");
    }

    let freq = grid.grid.iter()
        .map(|(ordinal, _)| ordinal)
        .filter(|val| !infinite.contains(*val))
        .fold(HashMap::new(), |mut acc, val| { *acc.entry(val).or_insert(0) += 1; acc });

    let item = freq.iter().max_by_key(|(_,v)|*v).unwrap();
    println!("The winning item is {:?}", item);
}

fn main() {
    let input = read_input();
    part_1(&input);
}
