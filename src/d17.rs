use std::collections::{HashMap, HashSet};
use std::fmt::Write;

type Pos = (isize, isize); // (row, col)

static _TEST: &str = include_str!("../data/d17_test.txt");
static INPUT: &str = include_str!("../data/d17.txt");

static MINUS: [Pos; 4] = [(0, 2), (0, 3), (0, 4), (0, 5)];
static PLUS: [Pos; 4] = [(2,  3), (1, 2), (1, 4), (0, 3)];
static L: [Pos; 5] = [(2,  4), (1, 4), (0, 2), (0, 3), (0, 4)];
static I: [Pos; 4] = [(3, 2), (2, 2), (1, 2), (0, 2)];
static QUADRAT: [Pos; 4] = [(1, 2), (1, 3), (0, 2), (0, 3)];

static MIN_DIST: isize = 3;

struct Chamber {
    rocks: HashSet<Pos>,
    width: isize,
}

impl Chamber {
    fn new() -> Self {
        Chamber{ rocks: HashSet::new(), width: 7 }
    }

    fn height(&self) -> isize {
        self.rocks.iter().map(|(row, _)| row + 1).max().unwrap_or(0)
    }

    fn determine_rock_pos(&self, rock: &Rock, dir: Dir) -> Option<Vec<Pos>> {
        // move shape in direction
        let shape = rock.pos.iter().map(|pos| dir + *pos).collect::<Vec<Pos>>();
        for pos in &shape {
            // collision with wall, floor or other rocks
            if pos.1 < 0 || pos.1 >= self.width || pos.0 < 0 || self.rocks.contains(&pos) {
                return None;
            }
        }
        Some(shape)
    }

    fn step(&self, rock: &mut Rock, dir: Dir) -> bool {
        if let Some(positions) = self.determine_rock_pos(&rock, dir) {
            rock.pos = positions;
        }
        match self.determine_rock_pos(&rock, Dir::Down) {
            Some(positions) => { 
                rock.pos = positions;
                true
            },
            None => false
        }
}
}

impl ToString for Chamber {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for row in (0..self.height() + 1).rev() {
            let mut row_repr = Vec::new();
            for col in 0..self.width {
                row_repr.push(if self.rocks.contains(&(row, col)) { '#' } else { '.' });
            }
            let _ = writeln!(s, "{}", row_repr.iter().collect::<String>());
        }
        s
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Rock {
    shape: Shape,
    pos: Vec<Pos>,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Shape {
    Minus,
    Plus,
    L,
    I,
    Quadrat,
}

impl Shape {
    fn spawn_rock(self, height: isize) -> Rock {
        let mut pos = match self {
            Self::Minus => MINUS.to_vec(),
            Self::Plus => PLUS.to_vec(),
            Self::L => L.to_vec(),
            Self::I => I.to_vec(),
            Self::Quadrat => QUADRAT.to_vec(),
        };
        pos.iter_mut().for_each(|(row, _)| *row += height + MIN_DIST);
        Rock { shape: self, pos }
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Down,
    Left,
    Right,
}

impl std::ops::Add<Pos> for Dir {
    type Output = Pos;

    fn add(self, other: Pos) -> Self::Output {
        let dir = match self {
            Self::Down => (-1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        };
        (other.0 + dir.0, other.1 + dir.1)
    }
}

fn parse(input: &str) -> Vec<Dir> {
    input.chars().map(|c| if c == '<' { Dir::Left } else { Dir::Right }).collect()
}

fn shapes() -> Vec<Shape> {
    vec![Shape::Minus, Shape::Plus, Shape::L, Shape::I, Shape::Quadrat]
} 

fn determine_repeat_interval(dirs: &[Dir]) -> ((isize, isize), (isize, isize)) {
    let mut chamber = Chamber::new();
    let mut shapes = shapes().into_iter().cycle();
    let mut rocks = HashMap::new();
    let mut rock = shapes.next().unwrap().spawn_rock(chamber.height());
    let mut n_blocks = 0;

    for (i, dir) in dirs.into_iter().enumerate().cycle() {
        if !chamber.step(&mut rock, *dir) {
            if let Some((blocks_init, height_init)) = rocks.insert((rock.shape, rock.pos[0].1, i), (n_blocks, chamber.height())) {
                return (
                    (blocks_init, height_init), 
                    (n_blocks - blocks_init, chamber.height() - height_init)
                );
            }
            rock.pos.iter().for_each(|pos| assert!(chamber.rocks.insert(*pos)));
            n_blocks += 1;
            rock = shapes.next().unwrap().spawn_rock(chamber.height()); 
        }
    }
    unreachable!();
}

fn build_n_blocks(n: isize, dirs: &[Dir]) -> isize {
    if n == 0 {
        return 0;
    }
    let mut chamber = Chamber::new();
    let mut n_blocks = 0;
    let mut shapes = shapes().into_iter().cycle();
    let mut rock = shapes.next().unwrap().spawn_rock(chamber.height());

    for dir in dirs.iter().cycle() {
        if !chamber.step(&mut rock, *dir) {
            rock.pos.iter().for_each(|pos| assert!(chamber.rocks.insert(*pos)));
            n_blocks += 1;
            if n_blocks == n {
                return chamber.height();
            }
            rock = shapes.next().unwrap().spawn_rock(chamber.height()); 
        }
    }

    unreachable!();
}

fn height_after_n_blocks(n: isize, dirs: Vec<Dir>) -> isize {
    let ((blocks_init, height_init), (blocks_repeat, height_repeat)) = determine_repeat_interval(&dirs);
    let (blocks_rem, cur_height) = match n.cmp(&(blocks_init)) {
        std::cmp::Ordering::Less => (n, 0),
        std::cmp::Ordering::Equal => return height_init,
        std::cmp::Ordering::Greater => {
            let cur_height = (n - blocks_init) / blocks_repeat * height_repeat + height_init;
            let blocks_rem = (n - blocks_init) % blocks_repeat;
            (blocks_rem, cur_height)
        },
    };
    return build_n_blocks(blocks_rem + blocks_init, &dirs) - height_init + cur_height;
}

pub fn get_solution_1() -> isize {
    height_after_n_blocks(2022, parse(INPUT))
}

pub fn get_solution_2() -> isize {
    height_after_n_blocks(1_000_000_000_000, parse(INPUT))
}