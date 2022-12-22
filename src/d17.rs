use std::{ops::{Index, IndexMut}, fmt::Display};

const LINE: [[bool; 4]; 1] = [[true, true, true, true]];
const CROSS: [[bool; 3]; 3] = [[false, true, false],
                                [true, true, true],
                                [false, true, false]];
const L: [[bool; 3]; 3] = [[false, false, true],
                            [false, false, true],
                            [true, true, true]];

const STICK: [[bool; 1]; 4] = [[true],
                                [true],
                                [true],
                                [true]];

const BLOCK: [[bool; 2]; 2] = [[true, true],
                                [true, true]];

const N_SHAPES: usize = 5;

static SHAPE_ORDER: [Shape; N_SHAPES] = [Shape::Line, Shape::Cross, Shape::L, Shape::Stick, Shape::Block];
static CHAMBER_WIDTH: usize = 7;
static INPUT: &str = include_str!("../data/d17.txt");
static _TEST: &str = include_str!("../data/d17_test.txt");


/// positions are: (x, y), where 0, 0 is top left of the grid
/// new rows are added at the top
#[derive(Debug)]
struct Chamber {
    grid: Vec<[bool; 7]>,
    cur_shape: usize,
}

impl Chamber {
    fn new() -> Self {
        Self { grid: vec![[false; 7]; 3], cur_shape: 0 }
    }

    /// Adds the necessary rows to the grid for the specified rock
    fn add_rows(&mut self, rock: &Rock) {
        // determine how many rows need to be added
        let n_rows = rock.height() as isize - self.get_highest() as isize + 3;
        if n_rows < 0 {
            for _ in 0..-n_rows {
                self.grid.pop();
            }
        } else {
            for _ in 0..n_rows {
                self.grid.push([false; 7]);
            }
        }
    }
    /// Gets highest position where a rock is
    fn get_highest(&self) -> usize {
        self.grid.iter().rev().enumerate().find(|(_, r)| r.iter().any(|r| *r)).map(|(i, _)| i).unwrap_or(self.grid.len())
    } 

    /// return height of the grid
    #[inline(always)]
    fn height(&self) -> usize {
        self.grid.len()
    }

    fn collides(&self, rock: &Rock) -> bool {
        // scan rock from top to bottom 
        // for each position, check if there's a collision
        for (y, row) in rock.shape.iter().enumerate() {
            for (x, brick) in row.iter().enumerate() {
                if *brick && self[rock.pos(x, y)]{
                    return true;
                }                                 
            }
        }
        false 
    }
    //......#
    fn move_rock(&self, rock: &mut Rock, dir: Dir) -> bool {
        let new_pos = match (dir, rock.pos) {
            (Dir::Down, (_, y_pos)) if y_pos > 0 => (rock.pos.0, rock.pos.1 - 1),
            (Dir::Left, (x_pos, _)) if x_pos > 0 => (rock.pos.0 - 1, rock.pos.1),
            (Dir::Right, (x_pos, _)) if x_pos + rock.width() < CHAMBER_WIDTH => (rock.pos.0 + 1, rock.pos.1),
            _ => return false,
        };

        let old_pos = rock.pos;
        rock.pos = new_pos;

        if self.collides(rock) {
            rock.pos = old_pos;
            return false;
        }

        return true;
    }

    fn snip(&mut self, n: usize) -> usize {
        let snipped = n;
        self.grid = self.grid.drain(n..).collect();
        snipped
    }

    fn stop_rock(&mut self, rock: &Rock) {
        for (y, row) in rock.shape.iter().enumerate().rev() {
            for (x, brick) in row.iter().enumerate() {
                if *brick {
                    self[rock.pos(x, y)] = true;                              
                }
            }
        }
    }
}

fn start_next(chamber: &mut Chamber) -> Rock {
    let mut rock = chamber.next().unwrap();
    chamber.add_rows(&rock);
    rock.pos.1 = chamber.grid.len() - 1; // set y coordinate
    rock
}

impl Iterator for Chamber {
    type Item = Rock ;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.cur_shape;
        self.cur_shape = (self.cur_shape + 1) % N_SHAPES;
        Some((&SHAPE_ORDER[idx]).into())
    }
}

impl Index<(usize, usize)> for Chamber {
    type Output= bool;
    
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.grid[idx.1][idx.0]
    }
}

impl IndexMut<(usize, usize)> for Chamber {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[idx.1][idx.0]
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter().rev() {
            let _ = writeln!(f, "{}", row.iter().map(|b| if *b { '#' } else { '.' }).collect::<String>());
        }
        Ok(())
    }
}

struct Rock {
    pos: (usize, usize),
    shape: Vec<Vec<bool>>,
}

impl Rock {
    fn height(&self) -> usize {
       self.shape.len()
    }
    
    fn width(&self) -> usize {
        self.shape[0].len()
    }

    #[inline(always)]
    fn pos(&self, x: usize, y: usize) -> (usize, usize) {
        (self.pos.0 + x, self.pos.1 - y)
    }
}

impl From<&Shape> for Rock {
    fn from(shape: &Shape) -> Self {
        Self { pos: (2, 0), shape: shape.to_vec() }
    }
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Down,
    Left,
    Right
} 

enum Shape {
    Line,
    Cross,
    L,
    Stick,
    Block
}

impl Shape {
    fn to_vec(&self) -> Vec<Vec<bool>> {
        match self {
            Shape::Line => shape_to_vec(LINE),
            Shape::Cross => shape_to_vec(CROSS),
            Shape::L => shape_to_vec(L),
            Shape::Stick => shape_to_vec(STICK),
            Shape::Block => shape_to_vec(BLOCK),
        }
    }
}

fn parse(input: &str) -> Vec<Dir> {
    input.chars().map(|c| if c == '>' { Dir::Right } else { Dir::Left} ).collect()
}

fn run(dirs: Vec<Dir>) -> usize {
    let mut chamber = Chamber::new();
    let mut rock = start_next(&mut chamber);
    let mut c = 0_usize;
    let mut h = 0;
    for dir in dirs.into_iter().cycle() {
        chamber.move_rock(&mut rock, dir);
        if !chamber.move_rock(&mut rock, Dir::Down) {
            chamber.stop_rock(&rock);
            c += 1;
            if c == 1_000_000_000_000 {
                // println!("{}", chamber);
                return chamber.height() - chamber.get_highest() + h;
            }
            if chamber.height() >= 500 {
                h += chamber.snip(400);
            }
            rock = start_next(&mut chamber);
        }        
    }
    unreachable!();
}

fn shape_to_vec<S, I, T>(shape: S) -> Vec<Vec<T>> 
where S: IntoIterator<Item = I>,
      I: IntoIterator<Item = T>,
{
    shape.into_iter().map(|inner| inner.into_iter().collect()).collect()
}

pub fn get_solution_1() -> usize {
    let dirs = parse(_TEST);
    let h = run(dirs);
    h
}

#[test]
fn test_run() {
    get_solution_1();
    println!("{}", parse(INPUT).len());
}