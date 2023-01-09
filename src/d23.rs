// if no elves in 8 positions around elf, do nothing
// else
// 1. if n, ne, nw is free, move n
// 2. if s, se, sw is free, move s
// 3. if w, nw, sw is free, move w
// 4. if e, ne, se is free, move e  
// order matters, first valid gets chosen

// if two elves end up at the same spot, don't move
// then rotate the moves backwards
use std::collections::{HashSet, HashMap};
use std::fmt::Write;

use crate::helpers::Complex;

type Pos = (i64, i64);

static INPUT: &str = include_str!("../data/d23.txt");
static _TEST: &str = include_str!("../data/d23_test.txt");

// coords: (x, y)
const N: Pos = (0, 1);
const NE: Pos = (1, 1);
const E: Pos = (1, 0);
const SE: Pos = (1, -1);
const S: Pos = (0, -1);
const SW: Pos = (-1, -1);
const W: Pos = (-1, 0);
const NW: Pos = (-1, 1);

fn parse(input: &str) -> HashSet::<Pos> {
    let mut elves = HashSet::new(); 
    for (y, line) in input.lines().rev().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }
    elves
}

fn check_dir(elf: Pos, elves: &HashSet<Pos>, dirs: &[Pos; 3]) -> bool {
    for dir in dirs {
        if elves.contains(&elf.c_add(*dir)) {
            return false;
        }
    }
    true
}

fn can_stay(elf: Pos, elves: &HashSet<Pos>) -> bool {
    for dir in [N, NE, E, SE, S, SW, W, NW] {
        if elves.contains(&elf.c_add(dir)) {
            return false;
        }
    }
    true
}

// determine all positions
// store old positions and proposed positions together and check fur duplicates
fn determine_positions(elves: &HashSet<Pos>, moves: &[[Pos; 3]]) -> (HashMap<Pos, Vec<Pos>>, bool) {
    let mut proposed_moves = HashMap::new();
    let mut not_moved_count = 0;
    for elf in elves {
        let new_pos = if can_stay(*elf, elves) {
            not_moved_count += 1;
            *elf
        } else {
            let mut new_pos = None;
            for dirs in moves {
                if check_dir(*elf, elves, dirs) {
                    new_pos = Some(elf.c_add(dirs[0]));
                    break;
                }
            }
            new_pos.unwrap_or(*elf)
        };
        let pos_entry = proposed_moves.entry(new_pos).or_insert(Vec::new());
        pos_entry.push(*elf);
    }
    (proposed_moves, not_moved_count == elves.len())
}

fn execute_moves(proposed_moves: HashMap<Pos, Vec<Pos>>) -> HashSet<Pos> {
    let mut elves = HashSet::new();
    for (proposed_move, old_positions) in proposed_moves {
        if old_positions.len() > 1 {
            for pos in old_positions {
                elves.insert(pos);
            }
        } else {
            elves.insert(proposed_move);
        }
    }   
    elves
}

fn determine_grid_size(elves: &HashSet<Pos>) -> (Pos, Pos) {
    let left = elves.iter().min_by(|(x1, _), (x2, _)| x1.cmp(x2)).unwrap().0;
    let right = elves.iter().max_by(|(x1, _), (x2, _)| x1.cmp(x2)).unwrap().0 + 1;
    let bottom = elves.iter().min_by(|(_, y1), (_, y2)| y1.cmp(y2)).unwrap().1;
    let top = elves.iter().max_by(|(_, y1), (_, y2)| y1.cmp(y2)).unwrap().1 + 1;

    ((left, right), (bottom, top))
}

fn count_empty_cells(elves: &HashSet<Pos>, grid_size: (u64, u64)) -> u64 {
    grid_size.0 * grid_size.1 - elves.len() as u64
}

fn do_round(elves: HashSet<Pos>, moves: &[[Pos; 3]]) -> (HashSet<Pos>, bool) {
    let (proposed_moves, none_moved) = determine_positions(&elves, moves);
    if none_moved {
        return (elves, true)
    }
    (execute_moves(proposed_moves), false)
}

fn move_elves(mut elves: HashSet<Pos>, n_rounds: u64) -> HashSet<Pos> {
    let mut moves = vec![
        [N, NE, NW],
        [S, SE, SW],
        [W, NW, SW],
        [E, NE, SE],
    ];

    for _ in 0..n_rounds {
        (elves, _) = do_round(elves, &moves);
        let first_m = moves.remove(0);
        moves.push(first_m);
    }

    elves
}
fn move_elves_p2(mut elves: HashSet<Pos>) -> u64 {
    let mut moves = vec![
        [N, NE, NW],
        [S, SE, SW],
        [W, NW, SW],
        [E, NE, SE],
    ];
    let mut none_moved;
    for i in 1.. {
        (elves, none_moved) = do_round(elves, &moves);
        if none_moved {
            return i;
        }
        let first_m = moves.remove(0);
        moves.push(first_m);
    }
    unreachable!();
}

fn _print_grid(elves: &HashSet<Pos>) {
    let mut grid = Vec::new();
    let (width, height) = determine_grid_size(elves);
    for _ in height.0..height.1 {
        grid.push(vec!['.'; width.0.abs_diff(width.1) as usize]);
    }
    for elf in elves {
        grid[(elf.1 - height.0) as usize][(elf.0 - width.0) as usize] = '#';
    }
    let mut s = String::new(); 
    for line in grid.iter().rev() {
        let _ = writeln!(s, "{}", line.iter().collect::<String>());
    }
    println!("{}", s);
}

pub fn get_solution_1() -> u64 {
    let mut elves = parse(INPUT);
    elves = move_elves(elves, 10);
    let (width, height) = determine_grid_size(&elves);
    count_empty_cells(&elves, (width.0.abs_diff(width.1), height.0.abs_diff(height.1)))
}

pub fn get_solution_2() -> u64 {
    let elves = parse(INPUT);
    move_elves_p2(elves)
}