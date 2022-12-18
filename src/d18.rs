use std::{collections::HashSet, ops::Range};

static N_SIDES: u64 = 6;
static INPUT: &str = include_str!("../data/d18.txt"); 
static _TEST: &str = include_str!("../data/d18_test.txt"); 
type Cube = (i64, i64, i64);

fn parse(input: &str) -> HashSet<Cube> {
    input.split('\n').map(|l| match &l.split(',').collect::<Vec<&str>>()[..] {
        [x, y, z] => (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()),
        _ => unreachable!(),
    }).collect()
}

fn count_overlapping(cubes: HashSet<Cube>) -> u64 {
    let mut n_overlapping = 0;
    let mut overlapping = Vec::new();

    for cube in cubes {
        n_overlapping += N_SIDES - 2 * overlapping.iter().filter(|lhs| does_overlap(lhs, &cube)).count() as u64;
        overlapping.push(cube);
    }

    n_overlapping
}

fn does_overlap(lhs: &Cube, rhs: &Cube) -> bool {
    lhs.0.abs_diff(rhs.0) + lhs.1.abs_diff(rhs.1) + lhs.2.abs_diff(rhs.2) == 1
}

fn get_ranges(cubes: &HashSet<Cube>) -> (Range<i64>, Range<i64>, Range<i64>) {
    (cubes.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0 - 1..cubes.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0 + 2,
    cubes.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1 - 1..cubes.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1 + 2,
    cubes.iter().min_by(|a, b| a.2.cmp(&b.2)).unwrap().2 - 1..cubes.iter().max_by(|a, b| a.2.cmp(&b.2)).unwrap().2 + 2)
}

// inspired by https://github.com/ClouddJR/advent-of-code-2022/blob/main/src/main/kotlin/com/clouddjr/advent2022/Day18.kt
fn flow(cubes: &HashSet<Cube>) -> u64 {
    let ranges = get_ranges(cubes);
    let mut visited = HashSet::new();
    let mut frontier = vec![(ranges.0.start, ranges.1.start, ranges.2.start)];
    let mut surface = 0;

    while !frontier.is_empty() {
        let next = frontier.pop().unwrap();
        if visited.contains(&next) {
            continue;
        }
        neighbours(&next).iter()
        .filter(|n| ranges.0.contains(&n.0) && ranges.1.contains(&n.1) && ranges.2.contains(&n.2))
        .for_each(|n| {
            if cubes.contains(&n) {
                surface += 1;
            } else {
                frontier.push(*n);
            }
            visited.insert(next);
        }); 
    }

    surface
}

fn neighbours(cube: &Cube) -> Vec<Cube> {
    vec![(cube.0 - 1, cube.1, cube.2), (cube.0 + 1, cube.1, cube.2),
        (cube.0, cube.1 - 1, cube.2), (cube.0, cube.1 + 1, cube.2),
        (cube.0, cube.1, cube.2 - 1), (cube.0, cube.1, cube.2 + 1)] 
}

pub fn get_solution_1() -> u64 {
    count_overlapping(parse(INPUT))
}   

pub fn get_solution_2() -> u64 {
    let cubes = parse(INPUT);
    flow(&cubes)
}

#[test]
fn test() {
    get_solution_2();
}