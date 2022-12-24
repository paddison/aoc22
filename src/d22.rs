use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::ops::{Add, Sub, Index};

use crate::helpers::gcd;

static INPUT: &str = include_str!("../data/d22.txt");
static _TEST: &str = include_str!("../data/d22_test.txt");

fn parse(input: &str) -> (Maze, Vec<Instr>)  {
    let len = input.find('\n').unwrap();
    let mut tiles = input.lines().take_while(|l| !l.is_empty())
        .map(|l| l.chars()
            .map(|c| match c {
                '.' => Tile::Floor,
                '#' => Tile::Wall,
                _ => Tile::None,
            })
            .collect())
        .collect::<Vec<Vec<Tile>>>();
    
    for row in tiles.iter_mut() {
        while row.len() < len {
            row.push(Tile::None);
        }
    }
    
    let pos = (0, tiles[0].iter().position(|t| t == &Tile::Floor).unwrap());
    (Maze { tiles, player: Player::new(pos) }, parse_instructions(input.lines().last().unwrap()))
}

fn parse_instructions(line: &str) -> Vec<Instr> {
    let mut instructions = Vec::new();
    let steps: Vec<Instr> = line.split(|c: char| c.is_alphabetic()).map(|n| Instr::Step(n.parse().unwrap())).collect();
    let dirs: Vec<Instr> = line.replace(|c: char| c.is_numeric(), "").chars().map(|s| Instr::Turn(if s == 'R' { Turn::Right } else { Turn::Left })).collect();
    let size = steps.len().max(dirs.len());
    
    for i in 0..size {
        if let Some(step) = steps.get(i) {
            instructions.push(*step);
        }
        if let Some(dir) = dirs.get(i) {
            instructions.push(*dir);
        }
    }

    instructions
}

fn parse_cube(input: &str) -> (Cube, Vec<Instr>) {
    let cube_raw = input.lines().take_while(|l| !l.is_empty()).map(|l| l.chars().collect()).collect::<Vec<Vec<_>>>();
    // determine side length: 
    // get positions of sides on 2d plane
    let (sides, positions) = parse_cube_sides(&cube_raw);
    let mut adj_sides = HashMap::new();
    // relative position of where an adjacent side might be
    for side in 0..6 {
        for (dir, other_side, other_dir) in get_adj_sides(side, &positions, &adj_sides) {
            adj_sides.insert((side, dir), (other_side, other_dir));
            // directions will flip if going the other way
            adj_sides.insert((other_side, ((other_dir + 2) % 4)), (side, ((dir + 2) % 4)));
        }
    }

    (Cube { sides, positions, adj_sides, player: Player { pos: (0, 0), facing: 1 }, cur_side: 0 }, parse_instructions(input.lines().last().unwrap()))
}

fn parse_cube_sides(cube_raw: &Vec<Vec<char>>) -> ([Vec<Vec<Tile>>; 6], [(usize, usize); 6]) {
    const EMPTY_V: Vec<Vec<Tile>> = Vec::new();
    let height = cube_raw.len();
    let width = cube_raw.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap().len();
    let side_len = gcd(height, width);
    let mut side = 0;
    let mut sides = [EMPTY_V; 6];
    let mut positions = [(0, 0); 6];

    for row in 0..height / side_len {
        for col in 0..width / side_len {
            let row_start = row * side_len;
            let col_start = col * side_len;
            if let Some(true) = cube_raw.get(row_start)
                                        .map(|row| row.get(col_start).map(|tile| !tile.is_whitespace()))
                                        .flatten() {
                sides[side] = cube_raw[row_start..row_start + side_len]
                    .iter()
                    .map(|row| row.iter().skip(col_start).take(side_len)
                        .map(|c| if *c == '.' { Tile::Floor } else { Tile::Wall })
                        .collect())
                    .collect::<Vec<Vec<_>>>();
                positions[side] = (row, col);
                side += 1;
            }
        }
    }

    (sides, positions)
}

fn get_adj_sides(side: usize, positions: &[(usize, usize)], adj_sides_map: &HashMap<(usize, u8), (usize, u8)>) -> Vec<(u8, usize, u8)> {
    // look for dist 1
    let mut adj_sides = Vec::new();
    for dir in 0..4 {
        if !adj_sides_map.contains_key(&(side, dir)) {
            let (side, other_dir) = look_for_adj_side(side, dir, positions);
            adj_sides.push((dir, side, other_dir));
        }
    }
    adj_sides
}

// returns side_id, dir
fn look_for_adj_side(side: usize, dir: u8, positions: &[(usize, usize)]) -> (usize, u8) {
    // if dir is facing downwards, directions stay the same
    // otherwise rotate directions: 
    let candidates = get_candidates();
    let pos = (positions[side].0 as isize, positions[side].1 as isize);
    let rotation = get_rotation(dir);
    // check for connections
    'outer: for (steps, other_dir) in candidates {
        let mut new_pos = pos;
        for step in steps {
            new_pos = get_rotation(step).c_mul(rotation).c_add(new_pos);
            if new_pos.0 < 0 || new_pos.1 < 0 || !positions.contains(&(new_pos.0 as usize, new_pos.1 as usize)) {
                continue 'outer;
            }
        }
        let other_side = positions.iter()
                                  .position(|other_pos| other_pos == &(new_pos.0 as usize, new_pos.1 as usize))
                                  .unwrap();
        return (other_side, rotate_dir(dir, other_dir)); 
    }
    
    unreachable!()
}

fn rotate_dir(dir: u8, other_dir: u8) -> u8 {
    (other_dir + (dir + 2) % 4) % 4
}

fn get_rotation(dir: u8) -> (isize, isize) {
    match dir {
        0 => (-1, 0),
        1 => (0, 1),
        2 => (1, 0),
        _ => (0, -1),
    } 
}

fn get_candidates() -> [(Vec<u8>, u8); 18]{
    [
        (vec![2], 2), 
        (vec![3, 2], 3), (vec![1, 2], 1), 
        (vec![0, 1, 1], 0), (vec![0, 3, 3], 0), (vec![1, 1, 2], 0), (vec![3, 3, 2], 0), (vec![0, 0, 0], 2),
        (vec![3, 0, 3, 3], 1), (vec![1, 0, 1, 1], 3), (vec![1, 0, 0, 0], 1), (vec![3, 0, 0, 0], 3),
        (vec![0, 0, 1, 0], 3), (vec![0, 0, 3, 0], 1), (vec![0, 1, 0, 1], 3), (vec![0, 3, 0, 3], 1),
        (vec![1, 0, 0, 1, 0], 2), (vec![3, 0, 0, 3, 0], 2)
    ]
}

#[derive(Debug)]
struct Maze{
    tiles: Vec<Vec<Tile>>,
    player: Player,
}

impl Maze {
    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn take_steps(&mut self, n_steps: u64) {
        for _ in 0..n_steps {
            if !self.update_player_pos() {
                break;
            }
        }   
    }

    fn update_player_pos(&mut self) -> bool {
        let new_pos_opt = match self.player.facing {
            0 => self.calc_new_pos(-1, Self::move_vertically),  // up
            1 => self.calc_new_pos(1, Self::move_horizontally),             // right 
            2 => self.calc_new_pos(1, Self::move_vertically),              // down
            _ => self.calc_new_pos(-1, Self::move_horizontally),              // left
        };

        if let Some(new_pos) = new_pos_opt {
            self.player.pos = new_pos;
            return true;
        }

        false
    }

    fn calc_new_pos<F: Fn(&Maze, isize, (usize, usize)) -> (usize, usize )>(&mut self, steps: isize, mov: F) -> Option<(usize, usize)> {
        let mut new_pos = mov(&self, steps, self.player.pos);
        while self[new_pos] == Tile::None {
            new_pos = mov(&self, steps, new_pos)
        }
        if self[new_pos] == Tile::Floor {
            Some(new_pos)
        } else {
            None
        }
    }

    fn move_vertically(&self, steps: isize, pos: (usize, usize)) ->  (usize, usize) {
        (Self::update_single_coord(self.height(), pos.0, steps), pos.1)
    }

    fn move_horizontally(&self, steps: isize, pos: (usize, usize)) -> (usize, usize) {
        (pos.0, (Self::update_single_coord(self.width(), pos.1, steps)))   
    }

    fn update_single_coord(size: usize, pos: usize, steps: isize) -> usize {
        (pos + (size as isize + steps) as usize) % size
    }
}

// (row, col)
impl Index<(usize, usize)> for Maze {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.tiles[index.0][index.1]
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row, line) in self.tiles.iter().enumerate() {
            let _ = writeln!(f, "{}", line.iter().enumerate()
                .map(|(col, t)| 
                    if self.player.pos == (row, col) {
                        char::from(&self.player)
                    } else {
                        match t {
                            Tile::None => ' ',
                            Tile::Floor => '.',
                            Tile::Wall => '#',
                        }
                    }
                )
                .collect::<String>()
            );
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Player {
    pos: (usize, usize),
    facing: u8, // 0: up, 1: right, 2: down, 3: left
}

impl Player {
    fn new(pos: (usize, usize)) -> Self {
        Self { pos, facing: 1 }
    }
    
    fn turn(&mut self, dir: Turn) {
        self.facing = (self.facing + u8::from(dir)) % 4;
    } 
}

impl From<&Player> for char {
    fn from(player: &Player) -> Self {
        match player.facing {
            0 => '^',
            1 => '>',
            2 => 'v',
            _ => '<',
        }
    }    
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Right,
    Left,
}

impl From<Turn> for u8 {
    fn from(dir: Turn) -> Self{
        match dir {
            Turn::Right => 1,
            Turn::Left => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    None,
    Floor,
    Wall,
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Step(u64),
    Turn(Turn),
}

trait Dir {
    fn is_opposite(&self, other: Self) -> bool;
    fn adjust_side(&self, side_len: usize, pos: (usize, usize)) -> (usize, usize);
}

impl Dir for u8 {
    fn is_opposite(&self, other: Self) -> bool {
        match (self, other) {
            (0, 2) | (2, 0) | (1, 3) | (3, 1) => true,
            _ => false,
        }
    }

    fn adjust_side(&self, side_len: usize, pos: (usize, usize)) -> (usize, usize) {
        match self {
            0 => (side_len - 1, pos.1),
            1 => (pos.0, 0),
            2 => (0, pos.1),
            _ => (pos.0, side_len - 1),
        }
    }
}

trait Complex {
    fn c_add(&self, other: Self) -> Self;
    fn c_mul(&self, other: Self) -> Self;
    fn c_div(&self, other: Self) -> Self;
}

impl<T> Complex for (T, T) 
where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T> + Copy
{
    fn c_add(&self, other: Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }

    fn c_mul(&self, other: Self) -> Self {
        (self.0 * other.0 - self.1 * other.1, 
         self.0 * other.1 + self.1 * other.0)
    }

    fn c_div(&self, other: Self) -> Self {
        let divisor = self.0 * self.0 + self.1 * self.1;
        let real = self.0 * other.0 + self.1 * other.1;
        let complex = self.0 * other.1 - self.1 * other.0;
        ( real / divisor, complex / divisor)
    }
}

fn handle_instr(instr: Instr, maze: &mut Maze) {
    match instr {
        Instr::Step(n_steps) => maze.take_steps(n_steps),
        Instr::Turn(dir) => maze.player.turn(dir),
    }
}

fn execute_all(instructions: Vec<Instr>, maze: &mut Maze) -> (usize, usize, usize) {
    for instr in instructions {
        // println!("{}", maze);
        handle_instr(instr, maze);
    }

    (maze.player.pos.0 + 1, maze.player.pos.1 + 1, (maze.player.facing + 3) as usize % 4)
}

#[derive(Debug)]
struct Cube {
    sides: [Vec<Vec<Tile>>; 6],
    positions: [(usize, usize); 6],
    adj_sides: HashMap<(usize, u8), (usize, u8)>,
    player: Player,
    cur_side: usize,
}

impl Cube {
    fn execute_all(&mut self, instructions: Vec<Instr>) -> (usize, usize, usize) {
        for instr in instructions {
            self.execute_instr(instr);
        }

        self.calc_abs_position()
    }

    fn execute_instr(&mut self, instr: Instr) {
        match instr {
            Instr::Step(n_steps) => self.take_steps(n_steps),
            Instr::Turn(dir) => self.player.turn(dir),
        }
    }

    fn take_steps(&mut self, n_steps: u64) {
        for _ in 0..n_steps {
            if !self.step() {
                break;
            }
        }
    }

    fn step(&mut self) -> bool {
        let (new_pos, new_side, new_facing) = self.calc_new_pos();
        if self.sides[new_side][new_pos.0][new_pos.1] == Tile::Floor {
            self.player.pos = new_pos;
            self.cur_side = new_side;
            self.player.facing = new_facing;
            true
        } else {
            false
        }
    }

    fn calc_new_pos(&mut self) -> ((usize, usize), usize, u8) {
        let side_len = self.sides[0].len();
        match self.player.facing {
            0 if self.player.pos.0 == 0 => self.switch_side(),
            0 => ((self.player.pos.0 - 1, self.player.pos.1), self.cur_side, self.player.facing),
            1 if self.player.pos.1 == side_len - 1 => self.switch_side(),
            1 => ((self.player.pos.0, self.player.pos.1 + 1), self.cur_side, self.player.facing),
            2 if self.player.pos.0 == side_len - 1 => self.switch_side(),  
            2 => ((self.player.pos.0 + 1, self.player.pos.1), self.cur_side, self.player.facing),
            3 if self.player.pos.1 == 0 => self.switch_side(),
            _ => ((self.player.pos.0, self.player.pos.1 - 1), self.cur_side, self.player.facing),
        }

    }

    fn calc_abs_position(&self) -> (usize, usize, usize) {
        let (s_row, s_col) = self.positions[self.cur_side];
        let abs_pos = (s_row * self.sides[0].len() + self.player.pos.0, s_col * self.sides[0].len() + self.player.pos.1);
        (abs_pos.0 + 1, abs_pos.1 + 1, self.player.facing as usize)
    }

    // (new_pos, new_side, new_facing)
    fn switch_side(&mut self) -> ((usize, usize), usize, u8) {
        let (side, end) = *self.adj_sides.get(&(self.cur_side, self.player.facing)).unwrap();
        (self.translate(self.player.facing, end, self.player.pos), side, end)
    }

    fn translate(&self, start: u8, end: u8, pos: (usize, usize)) -> (usize, usize) {
        // if start is opposite of end, dont swap coords
        let invert = self.sides[0].len() - 1;
        let mut new_pos = match (start, end) {
            (0, 0) | (1, 1) | (2, 2) | (3, 3) => pos,
            (0, 2) | (2, 0) | (1, 3) | (3, 1) => (invert - pos.0, invert - pos.1), // opposite
            (0, 1) | (1, 0) | (2, 3) | (3, 2) => (pos.1, pos.0),
            _ =>(invert - pos.1, invert - pos.0), 
        };
        // the column, row, that we end up at, which is always the same
        new_pos = end.adjust_side(self.sides[0].len(), new_pos);
        // the column/row, that we end up at, which changes
        new_pos
    }
}

pub fn get_solution_1() -> usize {
    let (mut maze, instructions) = parse(INPUT);
    let (row, col, facing) = execute_all(instructions, &mut maze);
    1000 * row + 4 * col + facing
}

pub fn get_solution_2() -> usize {
    // all coordinates are stored relatively
    // store shape of cube as hashmap, by checking where a player ends up at,
    // store positions of sides in a hashmap also
    // if he leaves a certain side from a certain direction
    // switch between relative coordinates on cube sides and absolute coordinates
    // only work with relative position
    // at the last step, translate relative position to absolute
    // store cube sides like [s1, s2, s3, s4, s5, s6]
    let (mut cube, instructions) = parse_cube(INPUT);
    let (row, col ,facing) = cube.execute_all(instructions);
    1000 * row + 4 * col + (facing + 3) as usize % 4
}


#[test]
fn test_init_maze() {
    let (maze, instr) = parse(_TEST);
    assert_eq!(maze.player.pos, (0, 8));
    println!("{}", maze);
    println!("{:?}", instr);
}

#[test]
fn test_update_pos() {
    let (mut maze, _) = parse(_TEST);
    assert!(maze.update_player_pos());
    assert_eq!(maze.player.pos, (0, 9));

    maze.player.pos = (1, 8);
    assert!(!maze.update_player_pos());

    maze.player.pos = (3, 8);
    maze.player.facing = 3;
    assert!(maze.update_player_pos());
    assert_eq!(maze.player.pos, (3, 11));

    maze.player.pos = (4, 6);
    maze.player.facing = 0;
    assert!(maze.update_player_pos());
    assert_eq!(maze.player.pos, (7, 6));

    maze.player.pos = (4, 0);
    maze.player.facing = 3;
    assert!(!maze.update_player_pos());
    assert_eq!(maze.player.pos, (4, 0));
    println!("{}", maze);
}

#[test]
fn test_execute_all() {
    let (mut maze, instructions) = parse(INPUT);
    execute_all(instructions, &mut maze);
}

#[test]
fn test_complex_mul() {
    // test rotation
    let n = (2, 1);

    let ninety = (0, 1);
    assert_eq!(n.c_mul(ninety), (-1, 2));

    let one_eighty = (-1, 0);
    assert_eq!(n.c_mul(one_eighty), (-2, -1));

    let two_seventy = (0, -1);
    assert_eq!(n.c_mul(two_seventy), (1, -2));
}

#[test]
fn test_rotate_dir() {
    assert_eq!(rotate_dir(2, 0), 0);
    assert_eq!(rotate_dir(2, 1), 1);
    assert_eq!(rotate_dir(2, 2), 2);
    assert_eq!(rotate_dir(2, 3), 3);

    assert_eq!(rotate_dir(1, 0), 3);
    assert_eq!(rotate_dir(1, 1), 0);
    assert_eq!(rotate_dir(1, 2), 1);
    assert_eq!(rotate_dir(1, 3), 2);

    assert_eq!(rotate_dir(3, 0), 1);
    assert_eq!(rotate_dir(3, 1), 2);
    assert_eq!(rotate_dir(3, 2), 3);
    assert_eq!(rotate_dir(3, 3), 0);

    assert_eq!(rotate_dir(0, 0), 2);
    assert_eq!(rotate_dir(0, 1), 3);
    assert_eq!(rotate_dir(0, 2), 0);
    assert_eq!(rotate_dir(0, 3), 1);
}

#[test]
fn test_parse_cube() {
    println!("{}", get_solution_2());
}