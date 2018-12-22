use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
enum Tile {
    Empty,
    Wall,
}

impl Tile {
    fn to_char(&self) -> char{
        match self {
            Tile::Empty => '.',
            Tile::Wall => '#',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
enum WarriorType {
    Goblin,
    Elf,
}

type IdType = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Warrior {
    race: WarriorType,
    id: IdType,
}

impl Warrior {
    fn to_char(&self) -> char{
        match self.race {
            WarriorType::Elf => 'E',
            WarriorType::Goblin => 'G',
        }
    }
}

#[derive(Debug)]
struct Map<T> {
    width: usize,
    height: usize,
    grid: Vec<T>,
}

impl<T> Map<T> {
    fn get(&self, x: usize, y: usize) -> &T {
        &self.grid[self.calculate_index(x,y)]
    }

    fn calculate_index(&self, x: usize, y: usize) -> usize {
        (self.width * y + x) as usize
    }

    fn set(&mut self, x: usize, y: usize, val: T) {
        self.grid[self.height * y + x] = val;
    }
}

type Point = (usize, usize);

struct Warriors {
    pos_to_id: HashMap<Point, IdType>,
    id_to_warrior: HashMap<IdType, Warrior>,
    next_id: IdType,
}

impl Warriors {
    fn new() -> Warriors {
        Warriors{ pos_to_id: HashMap::new(), id_to_warrior: HashMap::new(), next_id: 0 }
    }

    fn get_turn_order(&self) -> Vec<IdType> {
        let mut warriors_vec = self.pos_to_id.iter()
            .collect::<Vec<(&Point, &IdType)>>();
        warriors_vec.sort_by_key(|(pos, _)| *pos);

        warriors_vec.iter().map(|(_,id)| **id).collect()
    }

    fn find_warrior_pos_by_id(&self, id_to_find: usize) -> Option<Point> {
        self.pos_to_id.iter()
            .filter(|(_,id)| **id == id_to_find)
            .map(|(k,_)| *k)
            .nth(0)
    }

    fn get(&self, pos: &Point) -> Option<&Warrior> {
        self.pos_to_id.get(pos).and_then(|id|self.id_to_warrior.get(id))
    }
}

struct Board {
    map: Map<Tile>,
    warriors: Warriors,
}

impl Board {
    fn from_input() -> Board {
        let input_file = File::open("src/input.txt").unwrap();
        let reader = BufReader::new(input_file);

        let lines = reader.lines().flat_map(|l|l.ok()).collect::<Vec<String>>();

        let height = lines.len();
        let width = lines[0].len();
        let grid = lines.iter()
            .flat_map(
                |line| line.chars()
                    .map(|c| {
                        match c {
                            '#' => Tile::Wall,
                            _ => Tile::Empty,
                        }
                    }))
            .collect();

        let warriors = lines.iter()
            .flat_map(|line|line.chars())
            .enumerate()
            .flat_map(|(i,c)| {
                let pos = (i/width, i%width);
                match c {
                    'E' => Some((pos, WarriorType::Elf)),
                    'G' => Some((pos, WarriorType::Goblin)),
                    _ => None,
                }
            })
            .fold(Warriors::new(), |mut acc, (pos, race)| {
                let id = acc.next_id;
                acc.next_id += 1;
                acc.pos_to_id.insert(pos, id);
                acc.id_to_warrior.insert(id, Warrior { race, id });
                acc
            });

        Board {map: Map { width, height, grid}, warriors }
    }

    fn print(&self) {
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                print!("{}", self.warriors.get(&(x, y)).map_or(self.map.get(x,y).to_char(), |w| w.to_char()));
            }
            println!("");
        }
    }
}

fn next_turn(board: &mut Board, pos: Point) {
    // Do I have an adjacent enemy?
    //if adjacent_enemies(map, pos).is_some() { map.set(0,0, Tile::Empty); }
    board.map.set(0,0, Tile::Empty);
}

fn next_round(board: &mut Board) {
    // For every Warrior, in reading order, take the next step
    let turn_order = board.warriors.get_turn_order();

    for id in turn_order {
        let warrior_pos = board.warriors.find_warrior_pos_by_id(id);
        match warrior_pos {
            Some(pos) => next_turn(board, pos),
            None => (),
        };
    }
}

fn main() {
    let mut board = Board::from_input();
    board.print();

    next_round(&mut board);
    board.print();
}
