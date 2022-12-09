use std::{str::FromStr, collections::HashSet};

// head/tail = (x, y)
fn parse(input: &str) -> Vec<(Dir, usize)> {
    input.lines()
         .map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
         .map(|parts| (Dir::from_str(parts[0]).unwrap(), parts[1].parse::<usize>().unwrap()))
         .collect()
}


fn update_head(dir: Dir, head: &mut (isize, isize)) {
    match dir {
        Dir::Up => head.1 += 1,
        Dir::Right => head.0 += 1,
        Dir::Down => head.1 -= 1,
        Dir::Left => head.0 -= 1,
    }
}

fn update_tail(tail: &mut (isize, isize), head: &(isize, isize)) {
    match (head.0 - tail.0, head.1 - tail.1) {
        (x, y) if x > - 2 && x < 2 && y > -2 && y < 2 => return,
        (2, 0) => tail.0 += 1,              // right
        (-2, 0) => tail.0 -= 1,             // left
        (0, 2) => tail.1 += 1,              //up
        (0, -2) => tail.1 -= 1,             // down
        (x, y) if x.is_positive() && y.is_positive() => { tail.0 += 1; tail.1 += 1; },     // up right
        (x, y) if x.is_positive() && y.is_negative() => { tail.0 += 1; tail.1 -= 1; },     // down right
        (x, y) if x.is_negative() && y.is_positive() => { tail.0 -= 1; tail.1 += 1; },    // up left
        (x, y) if x.is_negative() && y.is_negative() => { tail.0 -= 1; tail.1 -= 1; },    // down left
        _ => unreachable!(),
    }
}

fn execute_steps(steps: Vec<(Dir, usize)>) -> usize {
    let mut visited = HashSet::<(isize, isize)>::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    for (dir, n_steps) in steps {
        for _ in 0..n_steps {
            update_head(dir, &mut head);
            update_tail(&mut tail, &head);
            visited.insert(tail.clone());
        }
    }

    visited.len()
}

fn execute_steps_p2(steps: Vec<(Dir, usize)>) -> usize {
    let mut visited = HashSet::<(isize, isize)>::new();
    let mut knots = [(0, 0); 10];
    for (dir, n_steps) in steps {

        for _ in 0..n_steps {
            update_head(dir, &mut knots[0]);
            for i in 1..10 {
                let head = knots[i - 1];
                update_tail(knots.get_mut(i).unwrap(), &head);
            }
            visited.insert(knots[9].clone());
        }
    } 

    visited.len()
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Dir {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            _ => Err("wrong string")
        }
    }
}

pub fn get_solution_1() -> usize {
    execute_steps(parse(include_str!("../data/d09.txt")))
}

pub fn get_solution_2() -> usize {
    execute_steps_p2(parse(include_str!("../data/d09.txt")))
}