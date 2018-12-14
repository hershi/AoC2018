//use std::io::prelude::*;
//use std::io::BufReader;
//use std::fs::File;

//fn read_input() -> Vec<usize> {
    //let input_file = File::open("src/input.txt").unwrap();
    //let reader = BufReader::new(input_file);
    //reader.lines()
        //.flat_map(|line|line.ok())
        //.map(|line|
             //line
                //.split_whitespace()
                //.flat_map(|s| s.parse::<usize>())
                //.collect::<Vec<usize>>())
        //.nth(0)
        //.unwrap()
//}

#[derive(Debug)]
struct MarbleCircle {
    marbles: Vec<usize>,
    current: usize,
}

impl MarbleCircle {
    fn new() -> MarbleCircle {
        MarbleCircle { marbles: vec![0], current: 0}
    }

    fn insert(&mut self, marble: usize) {
        let insert_position = (self.current + 1) % self.marbles.len() + 1;
        self.marbles.insert(insert_position, marble);
        self.current = insert_position;
    }

    fn remove(&mut self) -> usize {
        let removal_position =
            if self.current >= 7 {
                self.current - 7
            } else {
                self.current + self.marbles.len() - 7
            };

        let result = self.marbles.remove(removal_position);
        self.current = removal_position % self.marbles.len();

        result
    }
}

fn main() {
    let last_marble = 71032;
    let special = 23;

    let mut players = vec![0; 441];
    let mut circle = MarbleCircle::new();
    let mut current_player = 0;

    for marble in 1..last_marble + 1 {
        if (marble % 100 == 0 ) {
            println!("Current: {}", marble);
        }
        current_player = (current_player + 1) % players.len();
        if marble % special == 0 {
            players[current_player] += marble + circle.remove();
            continue;
        }

        circle.insert(marble);
    }

    println!("{:?}", players);
    println!("{:?}", players.iter().max().unwrap());
}
