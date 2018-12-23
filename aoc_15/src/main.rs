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
    fn get(&self, pos: &Point) -> &T {
        &self.grid[self.calculate_index(pos)]
    }

    fn calculate_index(&self, pos: &Point) -> usize {
        (self.width * pos.1 + pos.0) as usize
    }

    fn set(&mut self, pos: &Point, val: T) {
        self.grid[self.height * pos.1 + pos.0] = val;
    }

    fn get_neighbours(&self, pos: &Point) -> Vec<Point> {
        let pos = (pos.0 as isize, pos.1 as isize);
        vec![
            (pos.0, pos.1 - 1),
            (pos.0 - 1 , pos.1),
            (pos.0 + 1 , pos.1),
            (pos.0, pos.1 + 1),
        ].into_iter()
            .filter(|p| p.0 >= 0 && p.0 < self.width as isize
                    && p.1 >= 0 && p.1 < self.height as isize)
            .map(|p|(p.0 as usize, p.1 as usize))
            .collect()
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
                let pos = (i%width, i/width);
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

    fn is_position_empty(&self, pos: &Point) -> bool {
        *self.map.get(pos) == Tile::Empty
            && self.warriors.get(pos).is_none()
    }

    fn print(&self) {
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let pos = (x,y);
                print!("{}", self.warriors.get(&pos).map_or(self.map.get(&pos).to_char(), |w| w.to_char()));
            }
            println!("");
        }
    }
}

fn flood_fill(board: &Board, starting_pos: &Point) -> Map<i32> {
    let mut ff_map = Map::<i32> {
        width: board.map.width,
        height: board.map.height,
        grid: vec![std::i32::MAX; board.map.width * board.map.height],
    };

    ff_map.set(&starting_pos, 0);

    let mut distance = 1;
    let mut next_round = board.map.get_neighbours(starting_pos);

    while next_round.len() > 0 {
        let mut current_round = next_round;
        next_round = vec![];

        current_round.iter()
            .filter(|p| board.is_position_empty(p))
            .for_each(|p| {
                if *ff_map.get(p) < distance { return; }
                ff_map.set(p, distance);
                next_round.append(&mut board.map.get_neighbours(&p));
            });

        distance += 1;
    }

    ff_map
}

fn next_turn(board: &mut Board, pos: &Point) {
    // Do I have an adjacent enemy?
    //if adjacent_enemies(map, pos).is_some() { map.set(0,0, Tile::Empty); }
    let race = &board.warriors.get(pos).unwrap().race;

    let has_adjacent_enemy = board.map.get_neighbours(pos).iter()
        .flat_map(|p|board.warriors.get(p))
        .any(|w| &w.race != race);

    if !has_adjacent_enemy {
        let ff_map = flood_fill(board, pos);
        println!();
            for y in 0..ff_map.height {
                for x in 0..ff_map.width {
                    let pos = (x,y);
                    print!("{:3}", if *ff_map.get(&pos) < 1000 { ff_map.get(&pos) } else { &-1 });
                }
                println!("");
            }
        println!();
    }
}

fn next_round(board: &mut Board) {
    // For every Warrior, in reading order, take the next step
    let turn_order = board.warriors.get_turn_order();

    for id in turn_order {
        let warrior_pos = board.warriors.find_warrior_pos_by_id(id);
        match warrior_pos {
            Some(pos) => next_turn(board, &pos),
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


