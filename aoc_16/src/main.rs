#[macro_use] extern crate lazy_static;
extern crate regex;

mod ops;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

type Registers = [i32; 4];

#[derive(Debug)]
struct Instruction {
    opcode: u8,
    a: i32,
    b: i32,
    c: i32,
}

#[derive(Debug)]
struct Sample {
    before: Registers,
    after: Registers,
    instruction: Instruction,
}

fn read_input() -> (Vec<Sample>, Vec<Instruction>) {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    lazy_static! {
       static ref BEFORE: Regex = Regex::new(r"Before:\s*\[(\d+),\s*(\d+),\s*(\d+),\s*(\d+)\s*\]").unwrap();
       static ref AFTER: Regex = Regex::new(r"After:\s*\[(\d+),\s*(\d+),\s*(\d+),\s*(\d+)\s*\]").unwrap();
       static ref INSTRUCTION: Regex = Regex::new(r"(\d+)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();
    }

    let lines = reader.lines().flat_map(|l|l.ok()).collect::<Vec<String>>();

    let mut samples = vec![];
    let mut program = vec![];
    let mut iter = lines.iter();
    loop {
        let line = iter.next();
        if line.is_none() { break; }
        if let Some(captures) = BEFORE.captures(&line.unwrap()) {
            let before = [
                captures[1].parse::<i32>().unwrap(),
                captures[2].parse::<i32>().unwrap(),
                captures[3].parse::<i32>().unwrap(),
                captures[4].parse::<i32>().unwrap()];

            let captures = INSTRUCTION.captures(iter.next().unwrap()).unwrap();
            let instruction = Instruction {
                opcode: captures[1].parse::<u8>().unwrap(),
                a: captures[2].parse::<i32>().unwrap(),
                b: captures[3].parse::<i32>().unwrap(),
                c: captures[4].parse::<i32>().unwrap(),
            };

            let captures = AFTER.captures(iter.next().unwrap()).unwrap();
            let after = [
                captures[1].parse::<i32>().unwrap(),
                captures[2].parse::<i32>().unwrap(),
                captures[3].parse::<i32>().unwrap(),
                captures[4].parse::<i32>().unwrap()];


            samples.push(Sample { before, after, instruction } );
        } else if let Some(captures) = INSTRUCTION.captures(&line.unwrap()) {
            program.push(Instruction {
                opcode: captures[1].parse::<u8>().unwrap(),
                a: captures[2].parse::<i32>().unwrap(),
                b: captures[3].parse::<i32>().unwrap(),
                c: captures[4].parse::<i32>().unwrap(),
            });
        }
    }

    (samples, program)
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Operation {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

type OpMap = HashMap<Operation, &'static Fn(i32, i32, i32, &Registers) -> Registers>;

fn init_operations() -> OpMap {
    let mut operations: OpMap  = HashMap::new();
    operations.insert(Operation::Addr, &ops::addr);
    operations.insert(Operation::Addi, &ops::addi);
    operations.insert(Operation::Mulr, &ops::mulr);
    operations.insert(Operation::Muli, &ops::muli);
    operations.insert(Operation::Banr, &ops::banr);
    operations.insert(Operation::Bani, &ops::bani);
    operations.insert(Operation::Borr, &ops::borr);
    operations.insert(Operation::Bori, &ops::bori);
    operations.insert(Operation::Setr, &ops::setr);
    operations.insert(Operation::Seti, &ops::seti);
    operations.insert(Operation::Gtir, &ops::gtir);
    operations.insert(Operation::Gtri, &ops::gtri);
    operations.insert(Operation::Gtrr, &ops::gtrr);
    operations.insert(Operation::Eqir, &ops::eqir);
    operations.insert(Operation::Eqri, &ops::eqri);
    operations.insert(Operation::Eqrr, &ops::eqrr);
    operations
}

fn part_1(samples: &Vec<Sample>) {
    let operations = init_operations();

    let res = samples.iter()
        .map(|sample|
             operations.values()
                .map(|op| op(
                    sample.instruction.a,
                    sample.instruction.b,
                    sample.instruction.c,
                    &sample.before))
                .filter(|r| &sample.after == r)
                .count())
        .filter(|x| x >= &3)
        .count();

    println!("# of samples matching 3 or more operators: {}", res);
}

fn resolve_opcodes(samples: &Vec<Sample>) -> Vec<&'static Fn(i32, i32, i32, &Registers) -> Registers> {
    let operations = init_operations();

    let all_opcodes: HashSet<u8> = (0..16).collect();
    let mut ops_to_opcodes: HashMap<Operation, HashSet<u8>> =
        operations.keys()
            .map(|op| (op.clone(), all_opcodes.clone()))
            .collect();

    for sample in samples {
        operations.iter()
            .filter(|(_op, f)| f(
                    sample.instruction.a,
                    sample.instruction.b,
                    sample.instruction.c,
                    &sample.before) != sample.after)
            .for_each(|(op,_f)| {
                if let Some(op_entry) = ops_to_opcodes.get_mut(op) {
                    op_entry.remove(&sample.instruction.opcode);
                }
            });
    }

    for _i in 0..16 {
        let to_remove = ops_to_opcodes.iter()
            .filter(|(_op, codes)| codes.len() == 1)
            .map(|(op, codes)| (op.clone(), *codes.iter().nth(0).unwrap()))
            .collect::<Vec<(Operation, u8)>>();

        for (op, opcode) in to_remove {
            ops_to_opcodes.iter_mut()
                .for_each(|(k, codes)| if k != &op {codes.remove(&opcode); });
        }
    }

    if ops_to_opcodes.values()
            .filter(|opcodes| opcodes.len() != 1)
            .count() > 0 {
        panic!("Failed to find singular mapping!");
    }

    let mut opcodes_to_funcs: Vec<&'static Fn(i32, i32, i32, &Registers) -> Registers> = vec![&ops::addr; 16];
    ops_to_opcodes.iter()
        .map(|(op, opcodes)| (*opcodes.iter().nth(0).unwrap(), op.clone()))
        //.inspect(|(opcode,op)| println!("{:?}:{:?}", opcode, op))
        .for_each(|(opcode, op)| opcodes_to_funcs[opcode as usize] = operations.get(&op).unwrap().clone());

    opcodes_to_funcs
}

fn part_2(samples: &Vec<Sample>, program: &Vec<Instruction>) {
    let opcodes_to_funcs = resolve_opcodes(samples);

    let mut registers: Registers = [0; 4];
    program.iter()
        .for_each(|instruction| registers = opcodes_to_funcs[instruction.opcode as usize](
                instruction.a,
                instruction.b,
                instruction.c,
                &registers));

    println!("End state {:?}", registers);
}

fn main() {
    let (samples, program) = read_input();
    part_1(&samples);
    part_2(&samples, &program);
}
