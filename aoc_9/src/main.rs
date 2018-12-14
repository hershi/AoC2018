struct Node {
    value: usize,
    prev: usize,
    next: usize,
}

impl Node {
    fn new(value: usize, prev: usize, next: usize) -> Node {
        Node { value, prev, next }
    }
}

struct Circle {
    nodes: Vec<Node>,
    current: usize,
    size: usize,
}

impl Circle {
    fn new() -> Circle {
        Circle { nodes: Vec::new(), current: std::usize::MAX, size: 0 }
    }

    fn insert(&mut self, value: usize) {
        self.size += 1;
        if self.size == 1 {
            // Adding to an empty circle. Create a one-item circle
            self.nodes.push(Node::new(value, 0, 0));
            self.current = 0;
            return;
        }

        let next = self.nodes[self.current].next;
        self.nodes.push(Node::new(value, self.current, next));
        self.nodes[self.current].next = self.nodes.len() - 1;
        self.nodes[next].prev = self.nodes.len() - 1;
    }

    fn advance(&mut self, step: usize) {
        for _i in 0..step {
            self.current = self.nodes[self.current].next;
        }
    }

    fn rewind(&mut self, step: usize) {
        for i in 0..step {
            //println!("{} curr: {} val {}", i, self.current, self.nodes[self.current].value);
            self.current = self.nodes[self.current].prev;
        }
    }

    fn remove_current(&mut self) -> usize {
        let val = self.nodes[self.current].value;
        let prev = self.nodes[self.current].prev;
        let next = self.nodes[self.current].next;
        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;

        // TODO remove node from 'nodes'
        self.current = next;

        val
    }

    fn print(&self) {
        let mut c = self.current;
        loop {
            print!("{} ", self.nodes[c].value);
            c = self.nodes[c].next;
            if c == self.current { println!(""); break; }
        }
    }
}

fn optimized() {
    let last_marble = 71032 * 100;
    let special = 23;

    let mut players = vec![0; 441];
    let mut current_player = 0;
    let mut circle = Circle::new();
    circle.insert(0);

    for marble in 1..last_marble + 1 {
        //circle.print();
        //if marble % 100 == 0  {
            //println!("Current: {}", marble);
        //}
        current_player = (current_player + 1) % players.len();
        if marble % special == 0 {
            circle.rewind(7);
            players[current_player] += marble + circle.remove_current();
            continue;
        }

        circle.advance(1);
        circle.insert(marble);
        circle.advance(1);
    }

    println!("{:?}", players);
    println!("{:?}", players.iter().max().unwrap());
}


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
        if marble % 100 == 0  {
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

    optimized();
}
