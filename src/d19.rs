use std::collections::{HashMap, HashSet, BinaryHeap};

// initial algorithm:
// for each step, build the best bot that can be build right now, or wait.
// then recurse
static INPUT: &str = include_str!("../data/d19.txt");
static _TEST: &str = include_str!("../data/d19_test.txt");

// (ore, clay, obsidian)
type BluePrint = [Entry; 4];
type Entry = (Robot, (u32, u32, u32));
// state are the resources and active bots
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq,)]
struct State {
    r_ore: u32,
    r_clay: u32,
    r_obsidian: u32,
    r_geode: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    rem_steps: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score().cmp(&other.score()))
    }
}

impl State {
    fn new() -> Self {
        Self { ore: 0, clay: 0, obsidian: 0, geode: 0, r_ore: 1, r_clay: 0, r_obsidian: 0, r_geode: 0, rem_steps: 24 }
    }

    fn get_best_robot<'bp>(&self, bp: &'bp BluePrint) -> Option<&'bp Entry> {
        for entry in bp {
            if self.can_be_built(entry.1) {
                return Some(entry)
            }
        }
        None 
    }

    fn get_available_robots(&self, bp: &BluePrint) -> Vec<Entry> {
        bp.iter()
            .filter(|entry| self.can_be_built(entry.1))
            .cloned()
            .collect()
    }

    fn can_be_built(&self, (ore, clay, obsidian): (u32, u32, u32)) -> bool {
        self.ore >= ore && self.clay >= clay && self.obsidian >= obsidian
    }

    fn update_resources(&mut self) {
        self.clay += self.r_clay;
        self.ore += self.r_ore;
        self.obsidian += self.r_obsidian;
        self.geode += self.r_geode;
    }

    fn build_robot(&mut self, (robot, (ore, clay, obsidian)): Entry) {
            self.ore -= ore;
            self.clay -= clay;
            self.obsidian -= obsidian;
            match robot {
                Robot::Ore => self.r_ore += 1,
                Robot::Clay => self.r_clay += 1,
                Robot::Obsidian => self.r_obsidian += 1,
                Robot::Geode => self.r_geode += 1, 
            }
    }

    fn estimate(&self, rem_steps: u32) -> u32 {
        self.r_geode * (rem_steps + 1) + self.geode + rem_steps * (rem_steps + 1)
    }

    fn score(&self) -> u32 {
        let sum = self.r_ore + self.ore +
            (self.r_clay + self.clay) * 10 +
            (self.r_obsidian + self.obsidian) * 100 +
            (self.r_geode + self.geode) * 1000 + self.rem_steps;
        // println!("{}", sum);
        sum

    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

fn parse(input: &str) -> Vec<BluePrint> {
    let mut bps = Vec::new();
    for parts in input.lines()
                        .map(|line| line.split("Each").map(|robot| robot.split_whitespace().collect::<Vec<&str>>())
                        .collect::<Vec<Vec<&str>>>()) {
        let mut bp = [
            (Robot::Ore, (parts[1][3].parse().unwrap(), 0, 0)),
            (Robot::Clay, (parts[2][3].parse().unwrap(), 0, 0)),
            (Robot::Obsidian, (parts[3][3].parse().unwrap(), parts[3][6].parse().unwrap(), 0)),
            (Robot::Geode, (parts[4][3].parse().unwrap(), 0, parts[4][6].parse().unwrap())),
        ];
        bp.reverse();
        bps.push(bp);      
    }
    bps
}

fn run(mut state: State, bp: BluePrint, max: &mut u32, rem_steps: u32) {
    let next = state.get_available_robots(&bp);
 
    state.update_resources();
    // check if current max can still be reached:
    if rem_steps == 1 {
        if state.geode > *max {
            *max = state.geode;
            println!("{}", max);
        }
        return;
    }

    // always try to build the best robot
    for entry in next {
        let mut new_s = state;
        new_s.build_robot(entry);
        run(new_s, bp, max, rem_steps - 1);
    }
    run(state, bp, max, rem_steps - 1);
}

fn search(state: State, bp: BluePrint) {
    let mut queue = BinaryHeap::new();
    queue.push(state);
    while !queue.is_empty() {
        let mut state = queue.pop().unwrap();
        if state.rem_steps == 0 {
            println!("{}", state.geode);
            continue;
        }
        let next = state.get_available_robots(&bp);
        state.update_resources();
        state.rem_steps -= 1;
        for entry in next {
            let mut new_s = state;
            new_s.build_robot(entry);
            queue.push(new_s);
        }
        queue.push(state);
    }
}

pub fn get_solution_1() -> u32 {
    let state = State::new();
    let bps = parse(_TEST);
    let mut max = 0;
    run(state, bps[0], &mut max, 24);
    max 
}

#[test]
fn test_parse() {
    let bps = parse(_TEST);
    assert_eq!(bps.len(), 2);
    assert_eq!(bps[0][3], (Robot::Ore, (4, 0, 0)));
    assert_eq!(bps[1][1], (Robot::Obsidian, (3, 8, 0)));
    assert_eq!(bps[1][0], (Robot::Geode, (3, 0, 12)));
}

#[test]
fn test_state_can_be_build() {
    let mut state = State::new();
    assert!(!state.can_be_built((4, 0, 0)));
    state.ore = 3;
    state.clay = 8;
    assert!(state.can_be_built((3, 8, 0)));
}

#[test]
fn test_state_update_resources() {
    let mut state = State::new();
    state.r_obsidian = 1;
    state.r_clay = 3;
    state.update_resources();
    assert_eq!(state.ore, 1);
    assert_eq!(state.obsidian, 1);
    assert_eq!(state.clay, 3);
}

#[test]
fn test_state_build_robot() {
    let mut state = State::new();
    state.ore = 4;
    state.obsidian = 12;
    state.build_robot((Robot::Geode, (3, 0, 12)));
    assert_eq!(state.ore, 1);
    assert_eq!(state.obsidian, 0);
    assert_eq!(state.r_geode, 1);
}

#[test]
fn test_run() {
    let state = State::new();
    let bps = parse(_TEST);
    let mut max = 0;
    search(state, bps[1])
    // let mut scores = HashSet::new();
    // println!("{}", scores.len());
    // println!("{:?}", scores.iter().max());
}