use std::io::BufRead;

fn main() {
    let input_file = std::fs::File::open("inputs/d3").unwrap();
    let buf_reader = std::io::BufReader::new(input_file);

    let mut total = 0;

    buf_reader.lines().map(|l| l.unwrap()).for_each(|l| {
        let parts = l.split_at(l.len() / 2);
        let item = parts
            .0
            .chars()
            .find(|c| parts.1.contains(*c))
            .expect("there was no error");

        total += compute_priority(item);
    });

    println!("total : {}", total);
}

fn compute_priority(c: char) -> u64 {
    let priority = ((c.to_ascii_uppercase() as u64) - 64) + ((c.is_uppercase() as u64) * 26);
    return priority;
}
