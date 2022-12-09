use std::collections::HashMap;
use crate::helpers::BitMap;

trait Knot {
    fn add(&mut self, other: &Self);
    fn diff(&self, other: &Self) -> Self;
    fn ident(&self) -> Self;
    fn moves(&self) -> bool;
}

impl Knot for (isize, isize) {
    fn add(&mut self, other: &Self) {
        self.0 += other.0;
        self.1 += other.1;
    }

    fn diff(&self, other: &Self) -> Self {
        (self.0 - other.0, self.1 - other.1)
    }

    fn ident(&self) -> Self {
        (self.0.signum(), self.1.signum())
    }

    fn moves(&self) -> bool {
        !(self.0.abs() < 2  && self.1.abs() < 2)
    }
}

fn parse(input: &str) -> Vec<((isize, isize), usize)> {
    let dirs = HashMap::from([("U", (0, 1)), ("D", (0, -1)), ("R", (1, 0)), ("L", (-1, 0))]);
    
    input.lines()
         .map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
         .map(|parts| (*dirs.get(parts[0]).unwrap(), parts[1].parse::<usize>().unwrap()))
         .collect()
}

fn execute_steps<const N: usize>(steps: Vec<((isize, isize), usize)>) -> usize {
    let mut visited = [0; 8192];
    let mut knots = [(512, 512); N];

    for (dir, n_steps) in steps {
        for _ in 0..n_steps {
            knots[0].add(&dir);
            for i in 1..N {
                let diff = knots[i - 1].diff(&knots[i]);
                if diff.moves() {
                    knots[i].add(&diff.ident());
                }
            }
            visited.set_bit(knots[N - 1].1 as usize, knots[N - 1].0 as usize);
        }
    } 

    visited.count_bits()
    
}

pub fn get_solution_1() -> usize {
    execute_steps::<2>(parse(include_str!("../data/d09.txt")))
}

pub fn get_solution_2() -> usize {
    execute_steps::<10>(parse(include_str!("../data/d09.txt")))
}