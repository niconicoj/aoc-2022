use std::{fs::File, io::BufRead};

fn main() {
    let input_file = File::open("inputs/d1").unwrap();
    let buf_reader = std::io::BufReader::new(input_file);
    let mut elfs: Vec<i64> = vec![0];
    for line in buf_reader.lines() {
        let parsed = line.unwrap().parse::<i64>();
        match parsed {
            Ok(v) => {
                let i = elfs.len() - 1;
                unsafe { *elfs.get_unchecked_mut(i) += v };
            }
            Err(_) => {
                elfs.push(0);
            }
        }
    }

    elfs.sort();

    println!(
        "{}",
        elfs.get(elfs.len() - 1).unwrap()
            + elfs.get(elfs.len() - 2).unwrap()
            + elfs.get(elfs.len() - 3).unwrap()
    );
}
