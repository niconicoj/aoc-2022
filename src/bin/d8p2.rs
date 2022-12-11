use std::io::BufRead;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn main() {
    let input_file = std::fs::File::open("inputs/d8").unwrap();
    let buf_reader = std::io::BufReader::new(input_file);

    let input: Vec<i32> = buf_reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10).map(|v| v as i32))
                .collect_vec()
        })
        .flatten()
        .collect();

    let forest = Forest::new(input);

    let answer = forest.best_scenic_score();

    println!("{}", answer);
}

struct Forest {
    trees: Vec<i32>,
    dimension: usize,
}

impl Forest {
    pub fn new(trees: Vec<i32>) -> Self {
        Self {
            dimension: (trees.len() as f64).sqrt() as usize,
            trees,
        }
    }

    fn best_scenic_score(&self) -> usize {
        self.trees.iter().enumerate().fold(0, |top, (i, h)| {
            let local_score = self.compute_scenic_score(i, h);
            if local_score > top {
                local_score
            } else {
                top
            }
        })
    }

    fn compute_scenic_score(&self, i: usize, height: &i32) -> usize {
        let x = i % self.dimension;
        let y = i / self.dimension;

        self.score_right(x, y, height)
            * self.score_left(x, y, height)
            * self.score_top(x, y, height)
            * self.score_bottom(x, y, height)
    }

    fn score_top(&self, x: usize, y: usize, height: &i32) -> usize {
        self.trees
            .iter()
            .skip(x)
            .step_by(self.dimension)
            .take(y)
            .rev()
            .fold_while(0, |acc, h| {
                if h < height {
                    Continue(acc + 1)
                } else {
                    Done(acc + 1)
                }
            })
            .into_inner()
    }

    fn score_bottom(&self, x: usize, y: usize, height: &i32) -> usize {
        self.trees
            .iter()
            .skip(x + (y + 1) * self.dimension)
            .step_by(self.dimension)
            .fold_while(0, |acc, h| {
                if h < height {
                    Continue(acc + 1)
                } else {
                    Done(acc + 1)
                }
            })
            .into_inner()
    }

    fn score_left(&self, x: usize, y: usize, height: &i32) -> usize {
        self.trees
            .iter()
            .skip(y * self.dimension)
            .take(x)
            .rev()
            .fold_while(0, |acc, h| {
                if h < height {
                    Continue(acc + 1)
                } else {
                    Done(acc + 1)
                }
            })
            .into_inner()
    }

    fn score_right(&self, x: usize, y: usize, height: &i32) -> usize {
        self.trees
            .iter()
            .skip(y * self.dimension + x + 1)
            .take(self.dimension - (x + 1))
            .fold_while(0, |acc, h| {
                if h < height {
                    Continue(acc + 1)
                } else {
                    Done(acc + 1)
                }
            })
            .into_inner()
    }
}
