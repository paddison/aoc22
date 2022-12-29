use std::{fmt::Write, collections::{BinaryHeap, HashSet}};

static INPUT: &str = include_str!("../data/d24.txt");
static _TEST: &str = include_str!("../data/d24_test.txt");
static _TEST_MIN: &str = include_str!("../data/d24_test_min.txt");

type Winds = Vec<Vec<Wind>>;

fn parse(input: &str) -> (State, Winds) {
    let mut winds = Vec::new();
    let width = input.find('\n').unwrap();
    let height = input.len() / width;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let dir = match c {
                '^' => (-1, 0),
                '>' => (0, 1),
                'v' => (1, 0),
                '<' => (0, -1),
                _ => continue
            };
                winds.push(Wind { dir, pos: (row, col) });
            }
        }
    
    let goal = (height - 1, width - 2);
    (State { player: (0, 1), goal, dist: manhattan((0, 1), goal), steps: 0, dim: (height, width) }, vec![winds])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Wind {
    dir: (isize, isize),
    pos: (usize, usize),
}

impl Wind {
    fn update(wind: &Wind, width: usize, height: usize) -> Self {
        let mut pos = (
            (wind.pos.0 as isize + wind.dir.0 as isize) as usize, 
            (wind.pos.1 as isize + wind.dir.1 as isize) as usize
        );
        if pos.0 == 0 { pos.0 = height - 2 }
        if pos.0 == height - 1 { pos.0 = 1 }
        if pos.1 == 0 { pos.1 = width - 2 }
        if pos.1 == width - 1 { pos.1 = 1 }
        Wind { dir: wind.dir, pos}
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct State {
    player: (usize, usize),
    goal: (usize, usize),
    dist: usize,
    steps: usize,
    dim: (usize, usize),
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
    fn neighbours(&self, winds: &mut Winds) -> Vec<Self> {
        let mut neighbours = Vec::new();

        if winds.len() - 1  == self.steps {
            let new_winds = winds[self.steps].iter()
            .map(|w| Wind::update(w, self.dim.1, self.dim.0))
            .collect();
            winds.push(new_winds);
        }

        for m in Self::filter_moves(&self.get_moves(), &winds[self.steps + 1]) {
            let mut n = *self;
            let to_goal = manhattan(m, n.goal);
            n.steps += 1;
            n.player = m;
            n.dist = to_goal + n.steps;
            neighbours.push(n)
        }

        neighbours
    }

    fn get_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();

        for dir in [(1, 0), (0, 1), (0, 0), (-1, 0), (0, -1)].iter() {
            let new_pos = (self.player.0 as isize + dir.0, self.player.1 as isize + dir.1);
            if (new_pos.0 > 0 && new_pos.1 > 0 && new_pos.0 < (self.dim.0 - 1) as isize && new_pos.1 < (self.dim.1 - 1) as isize)
                || new_pos == (0, 1) 
                || new_pos == ((self.dim.0 - 1) as isize, (self.dim.1 - 2) as isize) 
            {
                let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                moves.push(new_pos);
            }
        }
        moves
    }

    fn filter_moves(moves: &[(usize, usize)], winds: &[Wind]) -> Vec<(usize, usize)> {
        let mut filtered_moves = Vec::new();
        for new_pos in moves {
            if winds.iter().find(|w| &w.pos == new_pos).is_none() {
                filtered_moves.push(*new_pos);
            }
        }
        filtered_moves
    }

    fn is_goal(&self) -> bool {
        self.player == self.goal
    }

    fn _to_string(&self, wind_states: &Winds) -> String {
        let winds = &wind_states[self.steps];
        let mut chars = Vec::new();
        let mut s = String::new();
        for row in 0..self.dim.0 {
            let mut line = Vec::new();
            for col in 0..self.dim.1 {
                if col == 0 || col == self.dim.1 - 1 || row == 0 || row == self.dim.0 - 1 {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            chars.push(line);
        }

        for wind in winds {
            chars[wind.pos.0][wind.pos.1] = match wind.dir {
                (-1, 0) => '^',
                (1, 0) => 'v',
                (0, -1) => '<',
                _ => '>',
            };
        }

        chars[0][1] = '.';
        chars[self.goal.0][self.goal.1] = '.';
        chars[self.player.0][self.player.1] = 'E';

        for row in chars {
            let _ = writeln!(s, "{}", row.iter().collect::<String>());
        }

        s
    }
}

fn manhattan(start: (usize, usize), goal: (usize, usize)) -> usize {
    start.0.abs_diff(goal.0) + start.1.abs_diff(goal.1)
}

// only calculate winds once
// winds are not part of state, but are calculated separately
// store winds in hashmap
fn a_star(state: State, winds: &mut Winds) -> State {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::from([(state.steps, state.player)]);

    queue.push(state);
    while !queue.is_empty() {
        let state = queue.pop().unwrap();
        
        if state.is_goal() {
            return state;
        }
        
        for n in state.neighbours(winds) {
            if !visited.contains(&(n.steps, n.player)) {
                visited.insert((n.steps, n.player));
                queue.push(n);
            }
        }
    }

    unreachable!();
}

pub fn get_solution_1() -> usize {
    let (state, mut winds) = parse(INPUT);
    a_star(state, &mut winds).steps
}

pub fn get_solution_2() -> usize {
    let (mut state, mut winds) = parse(INPUT);
    // go to exit
    state = a_star(state, &mut winds);
    // go back
    state.goal = (0, 1);
    state.dist = manhattan(state.player, state.goal) + state.steps;
    state = a_star(state, &mut winds);
    // go to exit again
    state.goal = (state.dim.0 - 1, state.dim.1 - 2);
    a_star(state, &mut winds).steps
}