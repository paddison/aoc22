use std::{collections::{HashMap, BinaryHeap}, fmt::Display};

static INPUT: &str = include_str!("../data/d12.txt");
static _TEST: &str = include_str!("../data/d12_test.txt");

#[derive(Debug)]
struct Graph {
    start: Node,
    goal: Node,
    nodes: Vec<i8>,
    dim: (usize, usize) // row, col
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.nodes.chunks(self.dim.1) {
            let _ = writeln!(f, "{}", row.iter().map(|n| if *n != 35 { char::from_digit(*n as u32, 36).unwrap() } else { '.' }).collect::<String>());
        }

        Ok(())
    }
}

impl Graph {

    fn walk(&self) -> (Vec<(usize, usize)>, usize) {
        let mut queue = BinaryHeap::new();
        queue.push(self.start);
        let mut costs = HashMap::from([(self.start.pos, 0_usize)]);
        let mut path = HashMap::from([(self.start.pos, None)]);
 
        while !queue.is_empty() {
            let cur =  queue.pop().unwrap();
            if cur == self.goal {
                return (Self::build_path(self.goal.pos, path), *costs.get(&cur.pos).unwrap());
            }
            for nb in self.get_neighbours(&cur) {
                if !costs.contains_key(&nb.pos) {
                    costs.insert(nb.pos, nb.cost);
                    path.insert(nb.pos, Some(cur.pos));
                    queue.push(nb);
                }
            }
        }

        (Vec::new(), 0)
    }

    fn hike(&mut self) -> usize {
        let starting_positions = self.nodes.iter().enumerate().filter(|(_, n)| **n <= 1).map(|(idx, _)| self.rev_idx(idx)).collect::<Vec<(usize, usize)>>();
        let mut paths = Vec::new();
        for pos in starting_positions {
            let mut new_start = Node { val: 0, pos, dist: 0, cost: 0 };
            new_start.dist = self.dist(&new_start);
            self.start = new_start;
            paths.push(self.walk().1);
        }
        *paths.iter().filter(|n| **n != 0).min().unwrap()
    }

    fn build_path(goal: (usize, usize), path: HashMap::<(usize, usize), Option<(usize, usize)>>) -> Vec<(usize, usize)> {
        let mut reconstructed_path = vec![goal];
        let mut cur = goal;
        while let Some(Some(n)) = path.get(&cur) {
            reconstructed_path.push(*n);
            cur = *n;
        }

        reconstructed_path
    }

    fn rev_idx(&self, idx: usize) -> (usize, usize) {
        (idx / self.dim.1, idx % self.dim.1)
    }

    fn dist(&self, n: &Node) -> usize {
        n.pos.0.abs_diff(self.goal.pos.0) + n.pos.1.abs_diff(self.goal.pos.1)
    }

    fn get_mut(&self, row: usize, col: usize) -> Option<i8> {
        if row >= self.dim.0 || col >= self.dim.1 {
            None
        } else {
            let idx = self.idx(row, col);
            Some(self.nodes[idx])
        }
    }

    // return only neighbours which are same, or one level higher
    fn get_neighbours(&self, node: &Node) -> Vec<Node> {
        let (row, col) = node.pos;
        let mut nb = Vec::new();

        let others_pos = [(row, col.overflowing_sub(1).0), (row, col + 1), (row.overflowing_sub(1).0, col), (row + 1, col)];

        for (r, c) in others_pos {
            if let Some(other) = self.get_mut(r, c) {
                if other - node.val < 2 {
                    let n = Node { val: other, pos: (r, c), dist: self.dist(&self.goal), cost: node.cost + 1 };
                    nb.push(n);
                }
            }
        }
    
        nb
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        self.dim.1 * row + col
    }
}

#[derive(Eq, Hash, Clone, Copy, Debug)]
struct Node {
    val: i8,
    pos: (usize, usize), // row, col
    dist: usize,
    cost: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.dist + self.cost).cmp(&(other.dist + other.cost)).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.dist + self.cost).partial_cmp(&(other.dist + other.cost)).map(|ord| ord.reverse())
    }
}

fn parse(input: &str) -> Graph {
    let mut nodes = Vec::new();
    let cols = input.find('\n').unwrap();
    let rows = input.len() / cols;
    let mut start = Node { val: 0, pos: (0, 0), dist: 0, cost: 0};
    let mut goal = Node { val: 27, pos: (0, 0), dist: 0, cost: usize::MAX };
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'S' => { start.pos = (row, col); nodes.push(0) },
                'E' => { goal.pos = (row, col); nodes.push(26) },
                c => nodes.push((c.to_digit(36).unwrap() - 10) as i8),
            }
        }
    }
    start.dist = start.pos.0.abs_diff(goal.pos.0) + start.pos.1.abs_diff(goal.pos.1);
    Graph { start, goal, nodes, dim: (rows, cols) }
}

pub fn get_solution_1() -> usize {
    let g = parse(INPUT);
    g.walk().1 
}

pub fn get_solution_2() -> usize {
    let mut g = parse(INPUT);
    g.hike()
}


#[test]
fn test_do_digit() {
    println!("{}", 'z'.to_digit(36).unwrap() - 10);
}

#[test]
fn test_walk() {
    let mut g = parse(INPUT);
    let (path, c) = g.walk();
    println!("{}", path.len());
    for (row, col) in path {
        let idx = g.idx(row, col);
        let n = &mut g.nodes[idx];
        *n = 35;
    }
    println!("{}", g);
    println!("{}", c);
}