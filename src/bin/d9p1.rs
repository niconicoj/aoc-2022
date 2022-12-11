use std::{collections::HashSet, io::BufRead};

fn main() {
    let input_file = std::fs::File::open("inputs/d9").unwrap();
    let buf_reader = std::io::BufReader::new(input_file);

    let instructions: Vec<Instruction> = buf_reader
        .lines()
        .filter_map(|l| {
            l.ok().map(|l| {
                let parts = l.split_once(' ').expect("bad intruction format");
                let direction: Direction = parts.0.try_into().unwrap();
                let times: u64 = parts.1.parse().expect("bad intruction times");
                Instruction { times, direction }
            })
        })
        .collect();

    let mut rope = Rope::default();

    instructions
        .into_iter()
        .for_each(|instruction| rope.apply_instruction(instruction));

    println!("{:?}", rope.position_record);
    println!("{}", rope.position_record.len());
}

struct Rope {
    head: (isize, isize),
    tail: (isize, isize),
    position_record: HashSet<(isize, isize)>,
}

impl Rope {
    pub fn default() -> Self {
        let mut position_record = HashSet::new();
        position_record.insert((0, 0));
        Self {
            head: (0, 0),
            tail: (0, 0),
            position_record,
        }
    }

    pub fn apply_instruction(&mut self, instruction: Instruction) {
        let delta: (isize, isize) = instruction.direction.into();
        println!("{:?}", instruction);
        (0..instruction.times).for_each(|_| {
            self.move_head(delta);
            self.move_tail();
        });
    }

    fn move_head(&mut self, delta: (isize, isize)) {
        self.head.0 += delta.0;
        self.head.1 += delta.1;
    }

    fn move_tail(&mut self) {
        let delta = self.head_tail_delta();
        if delta.0.abs() > 1 || delta.1.abs() > 1 {
            self.tail.0 += delta.0.min(1).max(-1);
            self.tail.1 += delta.1.min(1).max(-1);
            self.position_record.insert(self.tail);
        }
    }

    fn head_tail_delta(&self) -> (isize, isize) {
        (self.head.0 - self.tail.0, self.head.1 - self.tail.1)
    }
}

#[derive(Debug)]
struct Instruction {
    times: u64,
    direction: Direction,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Into<(isize, isize)> for Direction {
    fn into(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err("un recongnized direction glyph"),
        }
    }
}
