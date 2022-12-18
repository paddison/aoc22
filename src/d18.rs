use std::collections::HashSet;

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

// naive approach: for each cube, check if it overlaps with other cubes
fn count_overlapping(cubes: HashSet<Cube>) -> u64 {
    let mut n_overlapping = 0;
    let mut overlapping = Vec::new();

    for cube in cubes {
        let mut non_overlapping_sides = N_SIDES;
        for other in &overlapping {
            if does_overlap(other, &cube) {
                // subtract overlapping side from both cubes
                non_overlapping_sides -= 1;
                n_overlapping -= 1;
                if non_overlapping_sides == 0 {
                    break;
                }
            }
        }

        n_overlapping += non_overlapping_sides;
        overlapping.push(cube);
    }

    n_overlapping
}

fn does_overlap(lhs: &Cube, rhs: &Cube) -> bool {
    lhs.0.abs_diff(rhs.0) + lhs.1.abs_diff(rhs.1) + lhs.2.abs_diff(rhs.2) == 1
}

fn create_grid(start: i64, end: i64) -> HashSet<Cube> {
    let mut grid = HashSet::new();
    for x in start..end {
        for y in start..end {
            for z in start..end {
                grid.insert((x, y, z));
            }
        }
    }
    grid
}

fn get_air<'a>(grid: &'a HashSet<Cube>, cubes: &'a HashSet<Cube>) -> HashSet<Cube> {
    grid.difference(&cubes).cloned().collect()
}


fn get_adjacent(cube: &Cube, cubes: &Vec<Cube>) -> Option<Cube> {
    let adjacent = [(cube.0 - 1, cube.1, cube.2), (cube.0, cube.1 + 1, cube.2),
                    (cube.0, cube.1 - 1, cube.2), (cube.0, cube.1 + 1, cube.2),
                    (cube.0, cube.1, cube.2 - 1), (cube.0, cube.1, cube.2 + 1)]; 
    for adj in adjacent {
        if cubes.contains(&adj) {
            return Some(adj)
        }
    }
    None
}

fn search_pockets(mut air: HashSet<Cube>, cubes: &HashSet<Cube>) -> Vec<HashSet<Cube>> {
    let mut pockets = vec![];
    let mut visited = HashSet::new();
    let mut frontier = Vec::new();
    let air_cube = *air.iter().next().unwrap();
    air.remove(&air_cube);
    frontier.push(air_cube);

    while !frontier.is_empty() {
        let next = frontier.pop().unwrap();
        visited.insert(next);
        for n in neighbours(&next, &air, cubes) {
            if !visited.contains(&n) {
                frontier.push(n);
                air.remove(&n);
            }
        }
        if frontier.is_empty() {
            pockets.push(visited.drain().collect());
            if !air.is_empty() {
                let air_cube = *air.iter().next().unwrap(); 
                air.remove(&air_cube);
                frontier.push(air_cube);
            }
        }
    }

    pockets
}

fn neighbours(cube: &Cube, air: &HashSet<Cube>, cubes: &HashSet<Cube>) -> Vec<Cube> {
    let adjacent = [(cube.0 - 1, cube.1, cube.2), (cube.0 + 1, cube.1, cube.2),
                    (cube.0, cube.1 - 1, cube.2), (cube.0, cube.1 + 1, cube.2),
                    (cube.0, cube.1, cube.2 - 1), (cube.0, cube.1, cube.2 + 1)]; 
    
    let mut neighbours = Vec::new();
    for adj in adjacent {
        if !cubes.contains(&adj) && air.contains(&adj) {
            neighbours.push(adj);
        }
    }

    neighbours
}

// fn is_air_pocket(coord: &Cube, pockets: &HashSet<Cube>, cubes: &HashSet<Cube>, adjacent: [Cube; 6]) -> bool {
//     adjacent.iter().all(|c| cubes.contains(c) || pockets.contains(c)) && !cubes.contains(coord)
// }

pub fn get_solution_1() -> u64 {
    count_overlapping(parse(INPUT))
}   

pub fn get_solution_2() -> u64 {
    let cubes = parse(INPUT);
    // let air_pockets = get_air_pockets(&cubes);
    let grid = create_grid(0, 22);
    let air = get_air(&grid, &cubes);
    let pockets = search_pockets(air, &cubes);
    let mut pocket_surface = 0;
    for pocket in pockets {
        if pocket.contains(&(0, 0, 0)) {
            continue;
        }
        pocket_surface += count_overlapping(pocket); 
    }
    count_overlapping(cubes) - pocket_surface 
}

#[test]
fn test_does_overlap() {
    assert!(does_overlap(&(1, 1, 1), &(2, 1, 1)));
    assert!(!does_overlap(&(1, 1, 1), &(3, 1, 1)));
}

#[test]
fn test_count_overlapping() {
    let cubes = parse(_TEST);
    assert_eq!(count_overlapping(cubes), 64);
}

#[test]
fn test_search_pockets() {
    let cubes = parse(_TEST);
    let grid = create_grid(0, 8);
    let air = get_air(&grid, &cubes);
    let pockets = search_pockets(air, &cubes);
    println!("{:?}", pockets[1]);
}

#[test]
fn test() {
    get_solution_2();
}