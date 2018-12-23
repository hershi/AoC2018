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
        warriors_vec.sort_by_key(|(pos, _)| (pos.1, pos.0));

        warriors_vec.iter().map(|(_,id)| **id).collect()
    }

    fn find_warrior_pos_by_id(&self, id_to_find: usize) -> Option<Point> {
        self.pos_to_id.iter()
            .filter(|(_,id)| **id == id_to_find)
            .map(|(k,_)| *k)
            .nth(0)
    }

    fn get_by_pos(&self, pos: &Point) -> Option<&Warrior> {
        self.pos_to_id.get(pos).and_then(|id|self.id_to_warrior.get(id))
    }

    fn get_by_race(&self, race: WarriorType) -> Vec<(&Point, &Warrior)> {
        self.pos_to_id.iter()
            .flat_map(|(pos, id)| self.id_to_warrior.get(id).and_then(|w|Some((pos,w))))
            .filter(|(_, warrior)| warrior.race == race)
            .collect()
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
            && self.warriors.get_by_pos(pos).is_none()
    }

    fn print(&self) {
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let pos = (x,y);
                print!("{}", self.warriors.get_by_pos(&pos).map_or(self.map.get(&pos).to_char(), |w| w.to_char()));
            }
            println!("");
        }
    }
}

fn flood_fill(board: &Board, starting_pos: &Point) -> Map<(i32, Point)> {
    let mut ff_map = Map::<(i32, Point)> {
        width: board.map.width,
        height: board.map.height,
        grid: vec![(std::i32::MAX, (0,0)); board.map.width * board.map.height],
    };

    ff_map.set(&starting_pos, (0,(0,0)));

    let mut distance = 1;
    let mut next_round =
        board.map.get_neighbours(starting_pos).into_iter()
            .map(|p| (p, *starting_pos))
            .collect::<Vec<(Point, Point)>>();

    while next_round.len() > 0 {
        let mut current_round = next_round;
        next_round = vec![];

        current_round.iter()
            .filter(|(p, _)| board.is_position_empty(p))
            .for_each(|(p, from)| {
                if ff_map.get(p).0 <= distance { return; }
                ff_map.set(p, (distance, *from));
                next_round.append(&mut  board.map.get_neighbours(&p).into_iter().map(|neighbour|(neighbour, *p)).collect());
            });

        distance += 1;
    }

    ff_map
}

fn has_adjacent_enemy(board: &Board, pos: &Point, race: &WarriorType) -> bool {
     board.map.get_neighbours(pos).iter()
        .flat_map(|p|board.warriors.get_by_pos(p))
        .any(|w| &w.race != race)
}

fn next_turn(board: &mut Board, pos: &Point) {
    // Do I have an adjacent enemy?
    //if adjacent_enemies(map, pos).is_some() { map.set(0,0, Tile::Empty); }
    let warrior = board.warriors.get_by_pos(pos).unwrap().clone();

    if !has_adjacent_enemy(board, pos, &warrior.race) {
        // Perform a flood fill
        //
        // Opimization opportunity: We don't really need to perform a full flood fill -
        // we can stop at the end of the first "generation" in which we reach a spot
        // adjacent to an enemy.
        let ff_map = flood_fill(board, pos);
        //println!();
            //for y in 0..ff_map.height {
                //for x in 0..ff_map.width {
                    //let pos = (x,y);
                    //print!("{:3}", if ff_map.get(&pos).0 < 1000 { ff_map.get(&pos).0 } else { -1 });
                //}
                //println!("");
            //}
        //println!();

        // Find all spots that are adjacent to an enemy and are reachable,
        // and pick the one that is closest, resoving ties based on reading-order
        let enemy_race =
            if warrior.race == WarriorType::Elf { WarriorType::Goblin } else { WarriorType:: Elf};
        let target = board.warriors.get_by_race(enemy_race).iter()
            .flat_map(|(pos, _)| board.map.get_neighbours(pos))
            .filter(|pos| ff_map.get(pos).0 != std::i32::MAX)
            .min_by(|p1, p2| ff_map.get(p1).0.cmp(&ff_map.get(p2).0)
                                .then((p1.1,p1.0).cmp(&(p2.1, p2.0))));

        // If we found a target, backtrack to the first step
        // Since we're following the reading order when performing the
        // flood fill, this should satisfy the reading-order requirement
        if let Some(mut target) = target {
            while ff_map.get(&target).1 != *pos {
                target = ff_map.get(&target).1;
                //println!("  {:?}", target);
            }
            //println!("{:?} --> {:?}", pos, target);
            board.warriors.pos_to_id.remove(pos);
            board.warriors.pos_to_id.insert(target, warrior.id);
        }
    }

    //if has_adjacent_enemy(board, , race) {
    //}
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


