use std::{fs::File, io::BufRead};

fn main() {
    let input_file = File::open("inputs/d1/p1").unwrap();
    let buf_reader = std::io::BufReader::new(input_file);
    let mut curr_elf: i64 = 0;
    let mut top_elf: i64 = 0;
    for line in buf_reader.lines() {
        let parsed = line.unwrap().parse::<i64>();
        match parsed {
            Ok(v) => {
                curr_elf += v;
            }
            Err(_) => {
                if top_elf < curr_elf {
                    top_elf = curr_elf
                }
                curr_elf = 0;
            }
        }
    }

    println!("{}", top_elf);
}
