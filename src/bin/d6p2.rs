use std::{collections::HashSet, io::Read};

use itertools::Itertools;

fn main() {
    let input_file = std::fs::File::open("inputs/d6").unwrap();
    let mut buf_reader = std::io::BufReader::new(input_file);

    let mut input = String::new();
    buf_reader.read_to_string(&mut input).unwrap();
    let input: Vec<char> = input.chars().collect();

    let result = input.windows(14).find_position(|s| {
        let set: HashSet<&char> = s.iter().collect();
        set.len() == s.len()
    });

    println!("{}", result.unwrap().0 + 14);
}
