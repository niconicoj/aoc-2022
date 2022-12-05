#![feature(map_many_mut)]
use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
};

use itertools::Itertools;

type CrateMap = HashMap<usize, VecDeque<char>>;

fn main() {
    let input_file = std::fs::File::open("inputs/d5").unwrap();
    let buf_reader = std::io::BufReader::new(input_file);

    let mut lines = buf_reader.lines().map(|l| l.unwrap());

    let mut crate_map: CrateMap = HashMap::new();

    lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .for_each(|line| {
            line.chars().enumerate().for_each(|(i, c)| {
                if c.is_ascii_alphabetic() {
                    let idx = i / 4 + 1;
                    let crates = crate_map.entry(idx).or_insert(VecDeque::new());
                    crates.push_front(c);
                }
            })
        });

    print_crate_map(&crate_map);

    lines.by_ref().for_each(|line| {
        let instruction = parse_instruction(&line);
        apply_instruction(&instruction, &mut crate_map);
        print_crate_map(&crate_map);
    });

    crate_map.iter().sorted_by_key(|x| x.0).for_each(|(_, v)| {
        print!("{}", v.back().unwrap());
    })
}

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_instruction(instruction: &str) -> Instruction {
    let mut parts = instruction.split_whitespace();
    let count = parts.nth(1).map(|c| c.parse().unwrap()).unwrap_or(0);
    let from = parts.nth(1).map(|c| c.parse().unwrap()).unwrap_or(0);
    let to = parts.nth(1).map(|c| c.parse().unwrap()).unwrap_or(0);
    Instruction { from, to, count }
}

fn apply_instruction(instruction: &Instruction, crate_map: &mut CrateMap) {
    println!("{:?}", instruction);
    let [to, from] = crate_map
        .get_many_mut([&instruction.to, &instruction.from])
        .expect("invalid targets");
    for _ in 0..instruction.count {
        to.push_back(from.pop_back().expect("no more crates to pop"));
    }
}

fn print_crate_map(crate_map: &CrateMap) {
    crate_map.iter().for_each(|(k, v)| {
        println!("{} : {:?}", k, v);
    })
}
