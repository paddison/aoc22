use std::collections::{HashSet, HashMap};

trait Rope {
    fn add(&mut self, other: &Self);
    fn diff(&self, other: &Self) -> Self;
    fn ident(&self) -> Self;
    fn moves(&self) -> bool;
}

impl Rope for (isize, isize) {
    fn add(&mut self, other: &Self) {
        self.0 += other.0;
        self.1 += other.1;
    }

    fn diff(&self, other: &Self) -> Self {
        (self.0 - other.0, self.1 - other.1)
    }

    fn ident(&self) -> Self {
        (if self.0 != 0 { self.0 / self.0.abs() } else { 0 }, 
         if self.1 != 0 { self.1 / self.1.abs() } else { 0 })
    }

    fn moves(&self) -> bool {
        !(self.0 > - 2 && self.0 < 2 && self.1 > -2 && self.1 < 2)    }
}

// head/tail = (x, y)
fn parse(input: &str) -> Vec<((isize, isize), usize)> {
    let dirs = HashMap::from([("U", (0, 1)), ("D", (0, -1)), ("R", (1, 0)), ("L", (-1, 0))]);
    
    input.lines()
         .map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
         .map(|parts| (*dirs.get(parts[0]).unwrap(), parts[1].parse::<usize>().unwrap()))
         .collect()
}

fn execute_steps<const N: usize>(steps: Vec<((isize, isize), usize)>) -> usize {
    let mut visited = HashSet::<(isize, isize)>::new();
    let mut knots = [(0, 0); N];

    for (dir, n_steps) in steps {
        for _ in 0..n_steps {
            knots[0].add(&dir);
            for i in 1..N {
                let diff = knots[i - 1].diff(&knots[i]);
                if diff.moves() {
                    knots[i].add(&diff.ident());
                }
            }
            visited.insert(knots[N - 1].clone());
        }
    } 

    visited.len()
}

pub fn get_solution_1() -> usize {
    execute_steps::<2>(parse(include_str!("../data/d09.txt")))
}

pub fn get_solution_2() -> usize {
    execute_steps::<10>(parse(include_str!("../data/d09.txt")))
    
}