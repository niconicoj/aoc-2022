use std::io::BufRead;

fn main() {
    let input_file = std::fs::File::open("inputs/d2").unwrap();
    let buf_reader = std::io::BufReader::new(input_file);

    let mut score = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let parts = line.split_at(1);
        score += compute_score(parts.0.try_into().unwrap(), parts.1.try_into().unwrap());
    }

    println!("score : {}", score);
}

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug)]
struct ParseHandError;

impl TryFrom<&str> for Hand {
    type Error = ParseHandError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissor),
            _ => Err(ParseHandError),
        }
    }
}

fn compute_score(other: Hand, me: Hand) -> i64 {
    match other {
        Hand::Rock => match me {
            Hand::Rock => 4,
            Hand::Paper => 8,
            Hand::Scissor => 3,
        },
        Hand::Paper => match me {
            Hand::Rock => 1,
            Hand::Paper => 5,
            Hand::Scissor => 9,
        },
        Hand::Scissor => match me {
            Hand::Rock => 7,
            Hand::Paper => 2,
            Hand::Scissor => 6,
        },
    }
}
