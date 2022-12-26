use std::{fmt::Display, collections::{BinaryHeap, HashSet}};

static INPUT: &str = include_str!("../data/d24.txt");
static _TEST: &str = include_str!("../data/d24_test.txt");
static _TEST_MIN: &str = include_str!("../data/d24_test_min.txt");

fn parse(input: &str) -> State {
    let mut winds = Vec::new();
    let mut map = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let mut map_row: Vec<bool> = Vec::new();
        for (col, c) in line.chars().enumerate() {
            match c {
                '.' => map_row.push(true),
                '#' => map_row.push(false),
                c => {
                    map_row.push(false);
                    let dir = match c {
                        '^' => (-1, 0),
                        '>' => (0, 1),
                        'v' => (1, 0),
                        _ => (0, -1),
                    };
                    winds.push(Wind { dir, pos: (row, col) });
                }
            }
        }
        map.push(map_row);
    }
    let goal = (map.len() - 1, map[0].len() - 2);
    State { winds, map, player: (0, 1), goal, dist: manhattan((0, 1), goal), steps: 0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Wind {
    dir: (isize, isize),
    pos: (usize, usize),
}

impl Wind {
    fn update(&mut self, width: usize, height: usize) {
        let mut new_pos = (
            (self.pos.0 as isize + self.dir.0 as isize) as usize, 
            (self.pos.1 as isize + self.dir.1 as isize) as usize
        );
        if new_pos.0 == 0 { new_pos.0 = height - 2 }
        if new_pos.0 == height - 1 { new_pos.0 = 1 }
        if new_pos.1 == 0 { new_pos.1 = width - 2 }
        if new_pos.1 == width - 1 { new_pos.1 = 1 }
        self.pos = new_pos;
    }
}

#[derive(Debug, Clone, Hash)]
struct State {
    winds: Vec<Wind>,
    map: Vec<Vec<bool>>,
    player: (usize, usize),
    goal: (usize, usize),
    dist: usize,
    steps: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player && self.steps == other.steps
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist).map(|o| o.reverse())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

impl State {
    fn update(&mut self) {
        for wind in &mut self.winds {
            wind.update(self.map[0].len(), self.map.len());
        }
        let height = self.map.len();
        let width = self.map[0].len();
        for (row, line) in self.map[..height - 1].iter_mut().enumerate().skip(1){
            for (col, cell) in line[..width - 1].iter_mut().enumerate().skip(1) {
                *cell = self.winds.iter().find(|wind| wind.pos == (row, col)).is_none();
            }
        }
        self.steps += 1;
    }

    fn neighbours(&self) -> Vec<Self> {
        let mut neighbours = Vec::new();
        let mut new_state = self.clone();
        new_state.update();
        // let filterd = 
        for m in new_state.filter_moves(new_state.get_moves()) {
            let mut n = new_state.clone();
            let from_start = manhattan((0, 1), m);
            let to_goal = manhattan(m, self.goal);
            n.player = m;
            n.dist = n.steps; // add 1 for current node?
            neighbours.push(n)
        }

        neighbours
    }

    fn get_moves(&self) -> [(usize, usize); 5] {
        let (down, right, stay, up, left) = ((1, 0), (0, 1), (0, 0), (-1, 0), (0, -1));
        let new_pos = [down, right, stay, up, left].into_iter().map(|(r, c)| (self.player.0 as isize + r, c + self.player.1 as isize)).collect::<Vec<(isize, isize)>>();
        let mut moves = [(0, 0); 5];

        for (i, dir) in [down, right, stay, up, left].iter().enumerate() {
            let new_pos = (self.player.0 as isize + dir.0, self.player.1 as isize + dir.1);
            if new_pos.0 >= 0 && new_pos.1 >= 0 {
                let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                moves[i] = new_pos;
               
            }
        }
        moves
    }

    fn filter_moves(&self, moves: [(usize, usize); 5]) -> Vec<(usize, usize)> {
        let mut filtered_moves = Vec::new();
        for new_pos in moves.iter() {
            if let Some(true) = self.map.get(new_pos.0).map(|r| r.get(new_pos.1)).flatten() {
                filtered_moves.push(*new_pos);
            }
        }
        filtered_moves
    }

    fn is_goal(&self) -> bool {
        self.player == self.goal
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = self.map.iter()
                            .map(|row| row
                                .iter()
                                .map(|c| if *c { '.' } else { '#' })
                                .collect()
                            )
                            .collect::<Vec<Vec<char>>>();
        
        for wind in &self.winds {
            let c = match wind.dir {
                (-1, 0) => '^',
                (1, 0) => 'v',
                (0, 1) => '>',
                _ => '<',
            };
            let cur = chars.get_mut(wind.pos.0).unwrap().get_mut(wind.pos.1).unwrap();
                match &cur {
                    '#' => *cur = c,
                    n if n.is_numeric() => {
                        let mut n = n.to_digit(10).unwrap();
                        n += 1;
                        *cur = char::from_digit(n, 10).unwrap();
                    },
                    _ => *cur = '2', 
                }
            
        }

        chars[self.player.0][self.player.1] = 'E';

        for line in chars {
            let _ = writeln!(f, "{}", line.iter().collect::<String>());
        }

        Ok(())
    }
}

fn manhattan(start: (usize, usize), goal: (usize, usize)) -> usize {
    start.0.abs_diff(goal.0) + start.1.abs_diff(goal.1)
}

// only calculate winds once
// winds are not part of state, but are calculated separately
// store winds in hashmap
fn a_star(state: State) {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::from([(state.steps, state.player)]);
    queue.push(state);
    while !queue.is_empty() {
        let state = queue.pop().unwrap();
        println!("{}", state.steps);
        println!("{:?}", state.player);
        if state.is_goal() {
            println!("{}", state.steps);
            return;
        }
        let neigh = state.neighbours();
        
        for n in neigh {
            if !visited.contains(&(n.steps, n.player)) {
                visited.insert((n.steps, n.player));
                queue.push(n);
            }
        }
        
        // visited.insert((state.steps, state.player));
    }

    unreachable!();
}

pub fn get_solution_1() -> usize {
    let state = parse(INPUT);
    a_star(state);
    0
}

#[test]
fn test_parse() {
    let mut state = parse(_TEST);
    println!("{}", state);
    state.update();
    println!("{}", state);
    state.update();
    println!("{}", state);
    state.update();
    println!("{}", state);
    state.update();
    println!("{}", state);
    state.update();
    println!("{}", state);
}

#[test]
fn test_neighbours() {
    let state = parse(INPUT);
    println!("{:?}", a_star(state));
}