use std::{str::FromStr, collections::HashSet};

// head/tail = (x, y)
fn parse(input: &str) -> Vec<(Dir, usize)> {
    input.lines()
         .map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
         .map(|parts| (Dir::from_str(parts[0]).unwrap(), parts[1].parse::<usize>().unwrap()))
         .collect()
}


fn update_head(dir: Dir, head: &mut(isize, isize)) {
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
        (2, n) if n.is_positive() => { tail.0 += 1; tail.1 += 1; },     // up right
        (2, n) if n.is_negative() => { tail.0 += 1; tail.1 -= 1; },     // down right
        (-2, n) if n.is_positive() => { tail.0 -= 1; tail.1 += 1; },    // up left
        (-2, n) if n.is_negative() => { tail.0 -= 1; tail.1 -= 1; },    // down left
        (n, 2) if n.is_positive() => { tail.0 += 1; tail.1 += 1; },     // up right
        (n, 2) if n.is_negative() => { tail.0 -= 1; tail.1 += 1; },     // up left
        (n, -2) if n.is_positive() => { tail.0 += 1; tail.1 -= 1; },    // down right
        (n, -2) if n.is_negative() => { tail.0 -= 1; tail.1 -= 1; },    // down left
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
        // println!("{:?}, {}",dir, n_steps);

        for _ in 0..n_steps {
            update_head(dir, &mut knots[0]);
            for i in 1..10 {
                let head = knots[i - 1];
                update_tail(knots.get_mut(i).unwrap(), &head);
            }
            // println!("{:?}", knots);
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

fn test_data() -> Vec<(Dir, usize)> {
    parse("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2")
}

#[test]
fn test_update_head() {
    let mut head = (0, 0);
    update_head(Dir::Up, &mut head);
    assert_eq!((0, 1), head);
}

#[test]
fn test_update_tail() {
    let steps = test_data();
    let n_visited = execute_steps(steps);
    assert_eq!(n_visited, 13);
}

#[test]
fn test_update_tail_p2() {
    let steps = parse("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");
    let n_visited = execute_steps_p2(steps);
    assert_eq!(n_visited, 36);
}

#[test]
fn test_2() {
    get_solution_2();
}