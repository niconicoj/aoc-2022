use std::io::BufRead;

fn main() {
    let input_file = std::fs::File::open("inputs/d2").unwrap();
    let buf_reader = std::io::BufReader::new(input_file);

    let mut score = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let parts = line.split_at(1);
        let opponent: Hand = parts.0.try_into().unwrap();
        let goal: Goal = parts.1.try_into().unwrap();
        score += compute_score(goal, opponent);
    }

    println!("score : {}", score);
}

fn compute_score(goal: Goal, opponent: Hand) -> i64 {
    match opponent {
        Hand::Rock => match goal {
            Goal::Win => 8,
            Goal::Draw => 4,
            Goal::Lose => 3,
        },
        Hand::Paper => match goal {
            Goal::Win => 9,
            Goal::Draw => 5,
            Goal::Lose => 1,
        },
        Hand::Scissor => match goal {
            Goal::Win => 7,
            Goal::Draw => 6,
            Goal::Lose => 2,
        },
    }
}

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug)]
enum Goal {
    Win,
    Draw,
    Lose,
}

#[derive(Debug)]
struct ParseError;

impl TryFrom<&str> for Hand {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissor),
            _ => Err(ParseError),
        }
    }
}

impl TryFrom<&str> for Goal {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "X" => Ok(Goal::Lose),
            "Y" => Ok(Goal::Draw),
            "Z" => Ok(Goal::Win),
            _ => Err(ParseError),
        }
    }
}
