use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Cart {
    direction: Direction,
    next_turn: Direction,
}

impl Cart {
    fn new(direction: Direction) -> Cart {
        Cart { direction, next_turn: Direction::Left }
    }

    fn to_char(&self) -> char {
        match self.direction {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
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

fn main() {
    let (rails, carts) = read_input();
    rails.iter().for_each(|c| println!("{:?}", c));

    println!("");
    carts.iter().for_each(|c| println!("{:?}", c));

    print_state(&rails, &carts);
}
