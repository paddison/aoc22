use std::{collections::{HashMap, HashSet, BinaryHeap}, fmt::Display};

static INPUT: &str = include_str!("../data/d12.txt");
static _TEST: &str = include_str!("../data/d12_test.txt");

#[derive(Debug)]
struct Graph {
    start: Node,
    goal: Node,
    nodes: Vec<Node>,
    dim: (usize, usize) // row, col
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.nodes.chunks(self.dim.1) {
            let _ = writeln!(f, "{}", row.iter().map(|n| if n.val != 35 { char::from_digit(n.val as u32, 36).unwrap() } else { '.' }).collect::<String>());
        }

        Ok(())
    }
}

impl Graph {

    fn walk(&mut self) -> (Vec<(usize, usize)>, usize) {
        let mut queue = BinaryHeap::new();
        queue.push(self.start);
        let mut costs = HashMap::from([(self.start.pos, 0_usize)]);
        let mut path = HashMap::from([(self.start.pos, None)]);
        // let mut visited = HashSet::new();
 
        while !queue.is_empty() {
            let cur =  queue.pop().unwrap();
            if cur == self.goal {
                
                return (Self::build_path(self.goal.pos, path), *costs.get(&cur.pos).unwrap());
            }
            let cost = *costs.get(&cur.pos).unwrap();
            for mut nb in self.get_neighbours(&cur) {
                let new_cost = cost + 1;
                nb.h = new_cost + self.dist(&nb);
                if !costs.contains_key(&nb.pos) {
                    costs.insert(nb.pos, cost + 1);
                    path.insert(nb.pos, Some(cur.pos));
                    queue.push(nb);
                } else {
                    let old_cost = costs.get_mut(&nb.pos).unwrap();
                    if *old_cost > new_cost {
                        *old_cost = new_cost;
                        queue.push(nb);
                    }
                }
            }
        }

        (Vec::new(), 0)
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

    fn dist(&self, n: &Node) -> usize {
        n.pos.0.abs_diff(self.goal.pos.0) + n.pos.1.abs_diff(self.goal.pos.1)
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Node> {
        if row >= self.dim.0 || col >= self.dim.1 {
            None
        } else {
            let idx = self.idx(row, col);
            Some(&mut self.nodes[idx])
        }
    }

    // return only neighbours which are same, or one level higher
    fn get_neighbours(&mut self, node: &Node) -> Vec<Node> {
        let (row, col) = node.pos;
        let mut nb = Vec::new();

        let others_pos = [(row, col.overflowing_sub(1).0), (row, col + 1), (row.overflowing_sub(1).0, col), (row + 1, col)];

        for (r, c) in others_pos {
            if let Some(other) = self.get_mut(r, c) {
                if other.val - node.val < 2 {
                    nb.push(*other);
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
    h: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.h.cmp(&other.h).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.h.partial_cmp(&other.h).map(|ord| ord.reverse())
    }
}

fn parse(input: &str) -> Graph {
    let mut nodes = Vec::new();
    let cols = input.find('\n').unwrap();
    let rows = input.len() / cols;
    let mut start = Node { val: 0, pos: (0, 0), h: 0 };
    let mut goal = Node { val: 27, pos: (0, 0), h: 0 };
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'S' => { start.pos = (row, col); nodes.push(start) },
                'E' => { goal.pos = (row, col); nodes.push(goal) },
                c => nodes.push(Node { val: (c.to_digit(36).unwrap() - 9) as i8, pos: (row, col), h: 0 }),
            }
        }
    }

    Graph { start, goal, nodes, dim: (rows, cols) }
}

pub fn get_solution_1() -> usize {
    let mut g = parse(INPUT);
    g.walk().1 
}


#[test]
fn test_do_digit() {
    println!("{}", 'z'.to_digit(36).unwrap() - 10);
}

#[test]
fn test_walk() {
    let mut g = parse(INPUT);
    let (path, c) = g.walk();
    for (row, col) in path {
        let n = g.get_mut(row, col).unwrap();
        n.val = 35;
    }
    println!("{}", g);
    println!("{}", c);
}