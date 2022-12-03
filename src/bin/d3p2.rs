#![feature(iter_array_chunks)]
use std::io::BufRead;

fn main() {
    let input_file = std::fs::File::open("inputs/d3").unwrap();
    let buf_reader = std::io::BufReader::new(input_file);

    let mut total = 0;

    buf_reader
        .lines()
        .map(|l| l.unwrap())
        .array_chunks()
        .for_each(|[s1, s2, s3]| {
            let badge = s1
                .chars()
                .find(|c| s2.contains(*c) && s3.contains(*c))
                .expect("could not find a common item");
            total += compute_priority(badge);
        });

    println!("total : {}", total);
}

fn compute_priority(c: char) -> u64 {
    let priority = ((c.to_ascii_uppercase() as u64) - 64) + ((c.is_uppercase() as u64) * 26);
    return priority;
}
