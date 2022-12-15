use std::{fmt::Display, ops::{Index, IndexMut}};

static INPUT: &str = include_str!("../data/d14.txt");
static _TEST: &str = include_str!("../data/d14_test.txt");

const CAVE_ROWS: usize = 164;
const CAVE_COLS: usize = 500 ;
const LEFT_SHIFT: usize = CAVE_COLS / 2;
const START: usize = 500 - LEFT_SHIFT;

fn parse(input: &str) -> Vec<Vec<(usize, usize)>> {
    let mut structure = Vec::new();

    for line in input.lines() {
        let coords = line.split("->").collect::<Vec<&str>>();
        let mut cave_line = Vec::new();
        for coord in coords {
            let n_coord = coord.split(',').collect::<Vec<&str>>();
            cave_line.push((n_coord[0].trim().parse::<usize>().unwrap() - LEFT_SHIFT, n_coord[1].trim().parse::<usize>().unwrap()));
        }
        structure.push(cave_line);
    }

    structure
}

#[derive(Debug)]
struct Cave {
    coords: [[bool; CAVE_COLS]; CAVE_ROWS]
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.coords {
            let _ = writeln!(f, "{}", row.iter().map(|g| if *g { '#' } else { '.' }).collect::<String>());
        }   
        Ok(())
    }
}

impl Index<(usize, usize)> for Cave {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.coords[index.1][index.0]
    }
} 

impl IndexMut<(usize, usize)> for Cave {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.coords[index.1][index.0]
    }
}

impl Cave {
    fn new(structure: Vec<Vec<(usize, usize)>>) -> Self {
        let mut coords = [[false; CAVE_COLS]; CAVE_ROWS];
        for line in structure {
            for ((left_x, left_y), (right_x, right_y)) in line.iter().zip(&line[1..]) {
                let x_range = *left_x.min(right_x)..(*left_x.max(right_x) + 1);
                let y_range = *left_y.min(right_y)..(*left_y.max(right_y) + 1);
                for x in x_range {
                    for y in y_range.clone().into_iter() {
                        coords[y][x] = true;
                    }
                }
            }
        }

        Cave { coords }
    }

    fn update_pos(&self, sand: (usize, usize)) -> Option<(usize, usize)> {
        let updates: [(usize, usize); 3] = [(0, 1), (usize::MAX , 1), (1 , 1)];
        for update in updates {
            let new_pos = (sand.0.overflowing_add(update.0).0, sand.1 + 1);
            if !self[new_pos] {
                return Some(new_pos);
            }
        }
        None
    }

    fn falls_through(&self, sand: (usize, usize)) -> bool {
        sand.1 == self.coords.len() - 1
    }

    fn drop_sand(&self, drop_path: &mut Vec<(usize, usize)>) -> Option<(usize, usize)> {
        let mut sand = *drop_path.last().unwrap();//(START, 0);
        while let Some(new_pos) = self.update_pos(sand) {
            if self.falls_through(new_pos) {
                return None;
            }
            drop_path.push(new_pos);
            sand = new_pos;
        }
        Some(sand)
    }

    fn is_end(&self, sand: (usize, usize)) -> bool {
        sand == (START, 0)
    }

    fn simulate_p1(&mut self) -> usize {
        let mut count = 0;
        let mut drop_path = vec![(START, 0)];
        while let Some(sand) = self.drop_sand(&mut drop_path) {
            Self::trim_path(&mut drop_path);
            self[sand] = true;
            count += 1;
        }
        count
    }

    fn simulate_p2(&mut self, bottom: usize) -> usize {
        self.coords[bottom] = [true; CAVE_COLS];
        let mut count = 0;
        let mut drop_path = vec![(START, 0)]; 
        while let Some(sand) = self.drop_sand(&mut drop_path) {
            self[sand] = true;
            count += 1;
            if self.is_end(sand) {
                break;
            }
            Self::trim_path(&mut drop_path);
        }

        count
    }

    fn trim_path(drop_path: &mut Vec<(usize, usize)>) {
        let mut prev_pos = drop_path.last().unwrap();
        let mut drain_start = 0;
        for (i, pos) in drop_path.iter().enumerate().rev().skip(1) {
            if pos.0 == prev_pos.0 {
                drain_start = i;
                break;
            }
            prev_pos = pos;
        }
        drop_path.drain(drain_start + 1..);
    }
}

pub fn get_solution_1() -> usize {
    Cave::new(parse(INPUT)).simulate_p1() 
}

pub fn get_solution_2() -> usize {
    Cave::new(parse(INPUT)).simulate_p2(CAVE_ROWS - 1)
}