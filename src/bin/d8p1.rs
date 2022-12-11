use std::{collections::HashSet, io::BufRead};

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

    let visiblity_set = forest.visibility();

    println!("{:?}", visiblity_set.len());
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

    fn visibility(&self) -> HashSet<(usize, usize)> {
        let mut visibility_set = HashSet::new();
        self.visibility_l2r(&mut visibility_set);
        self.visibility_r2l(&mut visibility_set);
        self.visibility_b2t(&mut visibility_set);
        self.visibility_t2b(&mut visibility_set);
        visibility_set
    }

    fn visibility_l2r(&self, visibility_set: &mut HashSet<(usize, usize)>) {
        self.trees
            .chunks_exact(self.dimension)
            .enumerate()
            .for_each(|(y, row)| {
                row.iter().enumerate().fold(-1, |top_height, (x, height)| {
                    if height > &top_height {
                        visibility_set.insert((x, y));
                        *height
                    } else {
                        top_height
                    }
                });
            });
    }

    fn visibility_r2l(&self, visibility_set: &mut HashSet<(usize, usize)>) {
        self.trees
            .chunks_exact(self.dimension)
            .enumerate()
            .for_each(|(y, row)| {
                row.iter()
                    .enumerate()
                    .rev()
                    .fold(-1, |top_height, (x, height)| {
                        if height > &top_height {
                            visibility_set.insert((x, y));
                            *height
                        } else {
                            top_height
                        }
                    });
            });
    }

    fn visibility_t2b(&self, visibility_set: &mut HashSet<(usize, usize)>) {
        (0..self.dimension).for_each(|x| {
            self.trees
                .iter()
                .skip(x)
                .step_by(self.dimension)
                .enumerate()
                .fold(-1, |top_height, (y, height)| {
                    if height > &top_height {
                        visibility_set.insert((x, y));
                        *height
                    } else {
                        top_height
                    }
                });
        });
    }

    fn visibility_b2t(&self, visibility_set: &mut HashSet<(usize, usize)>) {
        (0..self.dimension).for_each(|x| {
            self.trees
                .iter()
                .skip(x)
                .step_by(self.dimension)
                .enumerate()
                .rev()
                .fold(-1, |top_height, (y, height)| {
                    if height > &top_height {
                        visibility_set.insert((x, y));
                        *height
                    } else {
                        top_height
                    }
                });
        });
    }
}
