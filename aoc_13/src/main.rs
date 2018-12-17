use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn next(&self, dir: &Direction) -> Point {
        match dir {
            Direction::Up => Point{x: self.x, y: self.y - 1},
            Direction::Down => Point{x: self.x, y: self.y + 1},
            Direction::Left => Point{x: self.x - 1, y: self.y},
            Direction::Right => Point{x: self.x + 1, y: self.y},
            Direction::Crash => Point{x: self.x, y: self.y},
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Crash,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Crash => Direction::Crash,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Crash => Direction::Crash,
        }
    }

    fn turn(&self, t: &Turn) -> Direction{
        match t {
            Turn::Left => self.turn_left(),
            Turn::Straight => self.clone(),
            Turn::Right => self.turn_right(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&self) -> Turn {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cart {
    direction: Direction,
    next_turn: Turn,
}

impl Cart {
    fn new(direction: Direction) -> Cart {
        Cart { direction, next_turn: Turn::Left }
    }

    fn to_char(&self) -> char {
        match self.direction {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Crash => 'X',
        }
    }
}

#[derive(Debug)]
enum Rail {
    Vertical,
    Horizontal,
    Curve1,
    Curve2,
    Intersection,
}

impl Rail {
    fn to_char(&self) -> char {
        match self {
            Rail::Vertical => '|',
            Rail::Horizontal => '-',
            Rail::Curve1 => '/',
            Rail::Curve2 => '\\',
            Rail::Intersection => '+',
        }
    }
}

type Carts = HashMap<Point, Cart>;
type Rails = HashMap<Point, Rail>;

fn read_input() -> (Rails, Carts) {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    reader.lines()
        .enumerate()
        .flat_map(|(i,line)|line.map(|l|(i,l)))
        .flat_map(|(y,line)|
            line.chars()
                .enumerate()
                .flat_map(|(x, c)| {
                    match c {
                        '/' => Some((Point {x, y}, Rail::Curve1, None)),
                        '-' => Some((Point {x, y}, Rail::Horizontal, None)),
                        '\\' => Some((Point {x, y}, Rail::Curve2, None)),
                        '|' => Some((Point {x, y}, Rail::Vertical, None)),
                        '+' => Some((Point {x, y}, Rail::Intersection, None)),
                        '>' => Some((Point {x, y}, Rail::Horizontal, Some(Cart::new(Direction::Right)))),
                        '<' => Some((Point {x, y}, Rail::Horizontal, Some(Cart::new(Direction::Left)))),
                        '^' => Some((Point {x, y}, Rail::Vertical, Some(Cart::new(Direction::Up)))),
                        'v' => Some((Point {x, y}, Rail::Vertical, Some(Cart::new(Direction::Down)))),
                        _ => None
                    }
                }).collect::<Vec<(Point, Rail, Option<Cart>)>>()

        )
        .fold((Rails::new(), Carts::new()), |(mut rails, mut carts), (pos, rail, cart)| {
            if let Some(c) = cart { carts.insert(pos.clone(), c); }
            rails.insert(pos, rail);
            (rails, carts)
        })
}

fn print_state(rails: &Rails, carts: &Carts) {
    let max_x = rails.keys().map(|p| p.x).max().unwrap();
    let max_y = rails.keys().map(|p| p.y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let p = Point{x,y};
            print!("{}", carts.get(&p)
                   .map_or(
                       rails.get(&p).map_or(' ', |r|r.to_char()),
                       |c|c.to_char())
            );
        }
        println!("");
    }
}

fn next_state(pos: &Point, rail: &Rail, cart: Cart) -> (Point, Cart) {
    match rail {
        Rail::Vertical => {
            match cart.direction {
                Direction::Up | Direction::Down => (pos.next(&cart.direction), cart),
                _ => panic!("Direction doesn't match rail: {:?} {:?} {:?}", rail, cart.direction, pos)
            }
        },
        Rail::Horizontal => {
            match cart.direction {
                Direction::Left | Direction::Right => (pos.next(&cart.direction), cart),
                _ => panic!("Direction doesn't match rail: {:?} {:?} {:?}", rail, cart.direction, pos)
            }
        },
        Rail::Curve1 => {
            // Curve 1: '/'
            match cart.direction {
                Direction::Up => (pos.next(&Direction::Right), Cart{direction: Direction::Right, next_turn: cart.next_turn}),
                Direction::Down => (pos.next(&Direction::Left), Cart{direction: Direction::Left, next_turn: cart.next_turn}),
                Direction::Left => (pos.next(&Direction::Down), Cart{direction: Direction::Down, next_turn: cart.next_turn}),
                Direction::Right => (pos.next(&Direction::Up), Cart{direction: Direction::Up, next_turn: cart.next_turn}),
                _ => panic!("Direction doesn't match rail: {:?} {:?} {:?}", rail, cart.direction, pos)
            }
        },
        Rail::Curve2 => {
            // Curve 1: '\\'
            match cart.direction {
                Direction::Up => (pos.next(&Direction::Left), Cart{direction: Direction::Left, next_turn: cart.next_turn}),
                Direction::Down => (pos.next(&Direction::Right), Cart{direction: Direction::Right, next_turn: cart.next_turn}),
                Direction::Left => (pos.next(&Direction::Up), Cart{direction: Direction::Up, next_turn: cart.next_turn}),
                Direction::Right => (pos.next(&Direction::Down), Cart{direction: Direction::Down, next_turn: cart.next_turn}),
                _ => panic!("Direction doesn't match rail: {:?} {:?} {:?}", rail, cart.direction, pos)
            }
        },
        Rail::Intersection => {
            let dir = cart.direction.turn(&cart.next_turn);
            (pos.next(&dir), Cart{direction: dir, next_turn: cart.next_turn.next()})
        },
    }
}

fn next_tick(rails: &Rails, carts: &mut Carts) {
    let mut positions = carts.keys().map(|k|k.clone()).collect::<Vec<(Point)>>();
    positions.sort();

    for p in positions {
        // This covers two scenarios:
        // This was a crash even before - then it should remain static
        // One of the previously moved carts crashes with the cart we're about to move. By
        // skipping it we're basically merging the two carts at the crash site.
        if carts.get(&p).map_or(false, |c|c.direction == Direction::Crash) { continue; }

        let (next_pos, next_cart) = next_state(&p, rails.get(&p).unwrap(), carts.remove(&p).unwrap());
        if carts.contains_key(&next_pos) {
            carts.insert(next_pos, Cart::new(Direction::Crash));
        } else {
            carts.insert(next_pos, next_cart);
        }
    }
}

fn part_1() {
    let (rails, mut carts) = read_input();
    let mut i = 0;
    loop {
        next_tick(&rails, &mut carts);
        {
            let crash = carts.iter().filter(|(_,c)| c.direction == Direction::Crash).last();
            if let Some((pos, cart)) = crash {
                println!("Crash at tick {}, position {:?} ({:?})", i, pos, cart);
                print_state(&rails, &carts);
                break;
            }
        }
        i+= 1;
    }
}

fn part_2() {
    let (rails, mut carts) = read_input();
    let mut i = 0;
    loop {
        if i % 100 == 0 { println!("Tick {}, {} carts left", i, carts.len()); }
        carts.retain(|_,c| c.direction != Direction::Crash);
        if carts.len() <= 1 { println!("Finished on tick {} with cart: {:?}", i, carts.iter().last()); break; }
        next_tick(&rails, &mut carts);
        i+= 1;
    }
}

fn main() {
    part_1();
    part_2();
}
