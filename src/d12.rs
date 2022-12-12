use std::{collections::{HashMap, BinaryHeap, VecDeque, HashSet}, fmt::Display};

static INPUT: &str = include_str!("../data/d12.txt");
static _TEST: &str = include_str!("../data/d12_test.txt");

#[derive(Debug, Clone)]
struct Graph {
    start: Node,
    goal: Node,
    heights: Vec<i8>,
    dim: (usize, usize) // row, col
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.heights.chunks(self.dim.1) {
            let _ = writeln!(f, "{}", row.iter().map(|n| if *n != 35 { char::from_digit(*n as u32, 36).unwrap_or('.') } else { '.' }).collect::<String>());
        }

        Ok(())
    }
}

impl Graph {

    fn dist(from: &(usize, usize), to: &(usize, usize)) -> usize {
        from.0.abs_diff(to.0) + from.1.abs_diff(to.1) 
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        self.dim.1 * row + col
    }

    fn coords(&self, idx: usize) -> (usize, usize) {
        (idx / self.dim.1, idx % self.dim.1)
    }

    fn height(&self, row: usize, col: usize) -> Option<i8> {
        if row >= self.dim.0 || col >= self.dim.1 {
            return None;
        } 
        Some(self.heights[self.idx(row, col)])
    }

    // return only neighbours which are same, or one level higher
    fn get_neighbours(&self, node: &Node) -> Vec<Node> {
        let (row, col) = node.pos;
        let mut nb = Vec::new();
        let neighbours_pos = [(row, col.overflowing_sub(1).0), (row, col + 1), (row.overflowing_sub(1).0, col), (row + 1, col)];

        for (r, c) in neighbours_pos {
            if let Some(height) = self.height(r, c) {
                if height - node.height < 2 {
                    let n = Node { height, pos: (r, c), dist: Self::dist(&(r, c), &self.goal.pos), cost: node.cost + 1 };
                    nb.push(n);
                }
            }
        }
        nb
    }

    fn walk(&mut self) -> Option<usize> {
        let mut queue = BinaryHeap::new();
        queue.push(self.start);
 
        while !queue.is_empty() {
            println!("{}", self);   
            let cur = queue.pop().unwrap();
            for nb in self.get_neighbours(&cur) {
                if nb == self.goal {
                    return Some(cur.cost);
                }
                let idx = self.idx(nb.pos.0, nb.pos.1);
                self.heights[idx] = i8::MAX;
                queue.push(nb);
            }
        }

        None
    }

    fn hike(g: Self) -> usize {
        let starting_positions = g.heights.iter()
                                        .enumerate()
                                        .filter(|(_, n)| **n <= 1)
                                        .map(|(idx, _)| g.coords(idx))
                                        .collect::<Vec<(usize, usize)>>();
        let mut paths = Vec::new();
        let n_threads = 4;
        let chunk_size = starting_positions.len() / n_threads;
        let mut handles = Vec::new();
        
        for id in 0..n_threads {
            let g_local = g.clone();

            let mut slice = Vec::new();

            for pos in &starting_positions[id * chunk_size..id * chunk_size + chunk_size] {
                slice.push(*pos);
            }

            let handle = std::thread::spawn(move || {
                let mut paths = Vec::new();
                let mut g_local_1 = g_local.clone();
                for pos in slice {
                    g_local_1.start = Node { height: 0, pos, dist: Self::dist(&pos, &g_local_1.goal.pos), cost: 0 };
                    if let Some(steps) = g_local_1.walk() {
                        paths.push(steps)
                    }
                }
                paths
            });

            handles.push(handle);
        }

        for handle in handles {
            paths.push(handle.join().map(|v| *v.iter().min().unwrap_or(&usize::MAX)).unwrap());
        }

        *paths.iter().min().unwrap()
    }
}

#[derive(Eq, Hash, Clone, Copy, Debug)]
struct Node {
    height: i8,
    dist: usize,
    cost: usize,
    pos: (usize, usize), // row, col
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
    let mut start = Node { height: 0, pos: (0, 0), dist: 0, cost: 0};
    let mut goal = Node { height: 27, pos: (0, 0), dist: 0, cost: usize::MAX };

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'S' => { start.pos = (row, col); nodes.push(0) },
                'E' => { goal.pos = (row, col); nodes.push(26) },
                c => nodes.push((c.to_digit(36).unwrap() - 10) as i8),
            }
        }
    }

    start.dist = Graph::dist(&start.pos, &goal.pos);
    Graph { start, goal, heights: nodes, dim: (rows, cols) }
}

pub fn get_solution_1() -> usize {
    let mut g = parse(INPUT);
    g.walk().unwrap()
}

pub fn get_solution_2() -> usize {
    let g = parse(INPUT);
    Graph::hike(g)
}

#[test]
fn test() {
    let mut g = parse(INPUT);
    g.walk().unwrap(); 
}