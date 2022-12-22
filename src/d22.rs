use std::fmt::Display;
use std::ops::{Add, Sub, Index};

static INPUT: &str = include_str!("../data/d22.txt");
static _TEST: &str = include_str!("../data/d22_test.txt");

fn parse(input: &str) -> (Maze, Vec<Instr>)  {
    let len = input.len();
    let tiles = input.lines().take_while(|l| !l.is_empty())
        .map(|l| l.chars()
            .map(|c| match c {
                '.' => Tile::Floor,
                '#' => Tile::Wall,
                _ => Tile::None,
            })
            .collect())
        .collect::<Vec<Vec<Tile>>>();
    
    let mut instructions = Vec::new();
    let line = input.lines().last().unwrap(); 
    let steps: Vec<Instr> = line.split(|c: char| c.is_alphabetic()).map(|n| Instr::Step(n.parse().unwrap())).collect();
    let dirs: Vec<Instr> = line.replace(|c: char| c.is_numeric(), "").chars().map(|s| Instr::Turn(if s == 'R' { Dir::Right } else { Dir::Left })).collect();
    let size = steps.len().max(dirs.len());
    for i in 0..size {
        if let Some(step) = steps.get(i) {
            instructions.push(*step);
        }
        if let Some(dir) = dirs.get(i) {
            instructions.push(*dir);
        }
    }
    
    let pos = (0, tiles[0].iter().position(|t| t == &Tile::Floor).unwrap());
    (Maze { tiles, player: Player::new(pos) }, instructions)
}

#[derive(Debug)]
struct Maze{
    tiles: Vec<Vec<Tile>>,
    player: Player,
}

impl Maze {
    fn update_player_pos(&mut self) -> bool {
        match self.player.facing {
            0 => self.move_up(),
            1 => self.move_right(),
            2 => self.move_down(),
            _ => self.move_left(),
        }
    }

    fn move_up(&mut self) -> bool {
        let mut pos = self.player.pos;
        let size = self.height();
        pos.0 = Self::update_single_coord(self.height(), pos.0, -1);
        while self[pos] == &Tile::None {
            pos.0 = Self::update_single_coord(self.height(), pos, steps)
        }

        false
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
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
    
    fn turn(&mut self, dir: Dir) {
        self.facing = (match dir {
            Dir::Right => u8::add,
            Dir::Left => u8::sub,
        })(self.facing, 1) % 4 
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
enum Dir {
    Right,
    Left,
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
    Turn(Dir),
}

#[test]
fn test_init_maze() {
    let (maze, instr) = parse(_TEST);
    assert_eq!(maze.player.pos, (0, 8));
    println!("{}", maze);
    println!("{:?}", instr);
}
