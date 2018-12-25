use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::ops::Fn;

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
    warrior_type: WarriorType,
    id: IdType,
    hp: i32,
    attack: i32
}

impl Warrior {
    fn new(warrior_type: WarriorType, id: IdType, attack: i32) -> Warrior {
        Warrior{warrior_type, id, hp: 200, attack}
    }

    fn to_char(&self) -> char{
        match self.warrior_type {
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

    fn get_by_warrior_type(&self, warrior_type: WarriorType) -> Vec<(&Point, &Warrior)> {
        self.pos_to_id.iter()
            .flat_map(|(pos, id)| self.id_to_warrior.get(id).and_then(|w|Some((pos,w))))
            .filter(|(_, warrior)| warrior.warrior_type == warrior_type)
            .collect()
    }
}

struct Board {
    map: Map<Tile>,
    warriors: Warriors,
}

impl Board {
    fn from_input(elf_attack: i32) -> Board {
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
            .fold(Warriors::new(), |mut acc, (pos, warrior_type)| {
                let id = acc.next_id;
                let attack = if warrior_type == WarriorType::Elf { elf_attack } else { 3 };
                acc.next_id += 1;
                acc.pos_to_id.insert(pos, id);
                acc.id_to_warrior.insert(id, Warrior::new( warrior_type, id, attack ));
                acc
            });

        Board {map: Map { width, height, grid}, warriors }
    }

    fn is_position_empty(&self, pos: &Point) -> bool {
        *self.map.get(pos) == Tile::Empty
            && self.warriors.get_by_pos(pos).is_none()
    }

    fn is_combat_finished(&self) -> bool {
        !self.warriors.id_to_warrior.values().any(|w| w.warrior_type == WarriorType::Elf)
            || !self.warriors.id_to_warrior.values().any(|w| w.warrior_type == WarriorType::Goblin)
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
        let current_round = next_round;
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

fn get_adjacent_enemies<'a>(board: &'a Board, pos: &Point, warrior_type: &WarriorType) -> Vec<(Point, &'a Warrior)> {
     board.map.get_neighbours(pos).into_iter()
        .flat_map(|p| board.warriors.get_by_pos(&p).and_then(|w|Some((p,w))))
        .filter(|(_,w)| &w.warrior_type != warrior_type)
        .collect::<Vec<(Point, &Warrior)>>()
}

fn next_turn(board: &mut Board, warrior_pos: &Point) {
    // Do I have an adjacent enemy?
    //if adjacent_enemies(map, pos).is_some() { map.set(0,0, Tile::Empty); }
    let warrior = board.warriors.get_by_pos(warrior_pos).unwrap().clone();

    let mut warrior_pos = *warrior_pos;
    if get_adjacent_enemies(board, &warrior_pos, &warrior.warrior_type).is_empty() {
        // Perform a flood fill
        //
        // Opimization opportunity: We don't really need to perform a full flood fill -
        // we can stop at the end of the first "generation" in which we reach a spot
        // adjacent to an enemy.
        let ff_map = flood_fill(board, &warrior_pos);
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
        let enemy_warrior_type =
            if warrior.warrior_type == WarriorType::Elf { WarriorType::Goblin } else { WarriorType:: Elf};
        let target = board.warriors.get_by_warrior_type(enemy_warrior_type).iter()
            .flat_map(|(pos, _)| board.map.get_neighbours(pos))
            .filter(|pos| ff_map.get(pos).0 != std::i32::MAX)
            .min_by(|p1, p2| ff_map.get(p1).0.cmp(&ff_map.get(p2).0)
                                .then((p1.1,p1.0).cmp(&(p2.1, p2.0))));

        // If we found a target, backtrack to the first step
        // Since we're following the reading order when performing the
        // flood fill, this should satisfy the reading-order requirement
        if let Some(mut target) = target {
            while ff_map.get(&target).1 != warrior_pos {
                target = ff_map.get(&target).1;
                //println!("  {:?}", target);
            }
            //println!("{:?} --> {:?}", pos, target);
            board.warriors.pos_to_id.remove(&warrior_pos);
            board.warriors.pos_to_id.insert(target, warrior.id);
            warrior_pos = target;
        }
    }

    if let Some((pos, id)) =
        get_adjacent_enemies(board, &warrior_pos, &warrior.warrior_type).iter()
            .min_by(|w1, w2| w1.1.hp.cmp(&w2.1.hp)
                .then(((w1.0).1, (w1.0).0).cmp(&((w2.0).1, (w2.0).0))))
            .map(|(pos, w)| (*pos, w.id)) {
        board.warriors.pos_to_id.remove(&pos);
        let mut enemy = board.warriors.id_to_warrior.remove(&id).unwrap();
        enemy.hp -= warrior.attack;
        if enemy.hp > 0 {
            board.warriors.id_to_warrior.insert(id, enemy);
            board.warriors.pos_to_id.insert(pos, id);
        }
    }
}

// Go through the next round. Return `true` if combat is done at any
// point in the round, `false` otherwise
fn next_round(board: &mut Board) -> bool {
    // For every Warrior, in reading order, take the next step
    let turn_order = board.warriors.get_turn_order();

    for id in turn_order {
        let warrior_pos = board.warriors.find_warrior_pos_by_id(id);
        match warrior_pos {
            Some(pos) => {
                if board.is_combat_finished() { return true; }
                next_turn(board, &pos);
            },
            None => (),
        };
    }

    // Combat not finished yet
    return false;
}

fn perform_combat<F>(mut board: Board, eval: F,  twarrior_type: bool) -> Result<(i32, Board), (i32, Board)>
        where F: Fn(&Board) -> bool {
    if twarrior_type { board.print(); }

    let mut round_counter = 0;
    while !next_round(&mut board) {
        if twarrior_type {
            println!("End of round {}", round_counter);
            board.print();
        }

        if !eval(&board) { return Err((round_counter, board)); }
        round_counter += 1;
    }

    if twarrior_type { board.print(); }

    Ok((round_counter, board))
}

fn run_simulation(elf_attack: i32) -> bool {
    let board = Board::from_input(elf_attack);
    let num_elves = board.warriors.id_to_warrior.values().filter(|w|w.warrior_type == WarriorType::Elf).count();
    let res = perform_combat(board,
                   |board|num_elves == board.warriors.id_to_warrior.values().filter(|w|w.warrior_type == WarriorType::Elf).count(),
                   false);
    match res {
        Ok((round_counter, board)) => {
            let total_hp = board.warriors.id_to_warrior.values()
                .map(|w|w.hp)
                .sum::<i32>();

            println!("Elf attack {}. Finished on round {}. All units' HP is {}. Result {}",
                     elf_attack,
                     round_counter,
                     total_hp,
                     round_counter * total_hp);
            true
        },
        Err((round_counter, board)) => {
            println!("Elf attack {}. Error on round {}. An elf has died.",
                     elf_attack,
                     round_counter);
            false
        },
    }
}

fn main() {
    let mut lower = 3;
    let mut upper = 200;

    while upper >= lower {
        let elf_attack = lower + (upper - lower)/2;
        println!("Upper,Lower,ElfAttack {},{},{}", upper, lower, elf_attack);
        if run_simulation(elf_attack) {
            upper = elf_attack - 1;
        } else {
            lower = elf_attack + 1;
        }
    }
}


