use std::{collections::BinaryHeap, fmt::Display, sync::Arc};

static INPUT: &str = include_str!("../data/d12.txt");
static _TEST: &str = include_str!("../data/d12_test.txt");

#[derive(Debug, Clone)]
struct Graph {
    start: Node,
    goal: Node,
    height_map: Vec<i8>,
    dim: (usize, usize) // row, col
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.height_map.chunks(self.dim.1) {
            let _ = writeln!(f, "{}", row.iter()
                                         .map(|n| match *n { 
                                            36 => '.',
                                            n => char::from_digit(n as u32, 36).unwrap_or('#'),  
                                         })
                                         .collect::<String>());
        }

        Ok(())
    }
}

impl Graph {
    fn idx(&self, (row, col): (usize, usize)) -> usize {
        self.dim.1 * row + col
    }

    fn coords(&self, idx: usize) -> (usize, usize) {
        (idx / self.dim.1, idx % self.dim.1)
    }

    fn height(&self, (row, col): (usize, usize)) -> Option<i8> {
        if row >= self.dim.0 || col >= self.dim.1 {
            return None;
        } 
        Some(self.height_map[self.idx((row, col))])
    }

    // return only neighbours which are same, or one level higher
    fn neighbours(&self, node: &Node) -> Vec<Node> {
        let (row, col) = node.pos;
        let mut nb = Vec::new();
        let others_pos = [(row, col.overflowing_sub(1).0), (row, col + 1), (row.overflowing_sub(1).0, col), (row + 1, col)];

        for pos in others_pos {
            match self.height(pos) {
                Some(other) if other - node.height < 2 => nb.push(Node { height: other, pos, cost: node.cost + 1 }),
                _ => continue,
            }
        }
        nb
    }
    
    fn walk(&mut self) -> Option<usize> {
        let mut queue = BinaryHeap::new();
        queue.push(self.start);
 
        while !queue.is_empty() {
            let cur =  queue.pop().unwrap();
            for nb in self.neighbours(&cur) {
                if nb.pos == self.goal.pos {
                    return Some(nb.cost);
                }
                let idx = self.idx(nb.pos);
                self.height_map[idx] = i8::MAX;        
                queue.push(nb);
            }
        }

        None
    }

    fn hike(g: Self) -> usize {
        let n_threads = 4;
        let starting_positions = Arc::new(g.height_map.iter()
                                        .enumerate()
                                        .filter(|(_, n)| **n == 0)
                                        .map(|(idx, _)| g.coords(idx))
                                        .collect::<Vec<(usize, usize)>>());
        let chunk_size = starting_positions.len() / n_threads;
        let mut paths = Vec::new(); 
        let mut handles = Vec::new();
        
        for id in 0..n_threads {
            let g_thread = g.clone();
            let thread_positions = Arc::clone(&starting_positions);
            let handle = std::thread::spawn(move || {
                let mut paths = Vec::new();
                for pos in &thread_positions[id * chunk_size..id * chunk_size + chunk_size] {
                    let mut g_iter = g_thread.clone();
                    g_iter.start = Node { height: 0, pos: *pos, cost: 0 };
                    if let Some(steps) = g_iter.walk() {
                        paths.push(steps)
                    }
                }
                paths
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Some(min_cost) = handle.join().unwrap().into_iter().min() {
                paths.push(min_cost);
            }
        }
        
        paths.into_iter().min().unwrap()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node {
    height: i8,
    pos: (usize, usize), // row, col
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost).map(|ord| ord.reverse())
    }
}

fn parse(input: &str) -> Graph {
    let mut nodes = Vec::new();
    let cols = input.find('\n').unwrap();
    let rows = input.len() / cols;
    let mut start = Node { height: 0, pos: (0, 0), cost: 0};
    let mut goal = Node { height: 26, pos: (0, 0), cost: usize::MAX };

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'S' => { start.pos = (row, col); nodes.push(0) },
                'E' => { goal.pos = (row, col); nodes.push(26) },
                c => nodes.push((c.to_digit(36).unwrap() - 10) as i8),
            }
        }
    }
    Graph { start, goal, height_map: nodes, dim: (rows, cols) }
}

pub fn get_solution_1() -> usize {
    let mut g = parse(INPUT);
    g.walk().unwrap()
}

pub fn get_solution_2() -> usize {
    let g = parse(INPUT);
    Graph::hike(g)
}