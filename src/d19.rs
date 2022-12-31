use std::collections::{HashMap, HashSet, BinaryHeap, VecDeque};

// initial algorithm:
// for each step, build the best bot that can be build right now, or wait.
// then recurse
static INPUT: &str = include_str!("../data/d19.txt");
static _TEST: &str = include_str!("../data/d19_test.txt");

static mut BP: Option<BluePrint> = None;

// (ore, clay, obsidian)
type BluePrint = [Entry; 4];
type Entry = (Robot, (u32, u32, u32));
// state are the resources and active bots
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

// impl PartialOrd for State {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         (self.geode + self.rem_steps).partial_cmp(&(other.geode + other.rem_steps))
//     }
// }

impl State {
    fn new(rem_steps: u32) -> Self {
        Self { ore: 0, clay: 0, obsidian: 0, geode: 0, r_ore: 1, r_clay: 0, r_obsidian: 0, r_geode: 0, rem_steps }
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
        let bp = [
            (Robot::Ore, (parts[1][3].parse().unwrap(), 0, 0)),
            (Robot::Clay, (parts[2][3].parse().unwrap(), 0, 0)),
            (Robot::Obsidian, (parts[3][3].parse().unwrap(), parts[3][6].parse().unwrap(), 0)),
            (Robot::Geode, (parts[4][3].parse().unwrap(), 0, parts[4][6].parse().unwrap())),
        ];
        bps.push(bp);      
    }
    bps
}

fn run(mut state: State, bp: BluePrint, max: &mut u32, visited: &mut HashSet<State>, id: usize) {
    // if state.r_geode == 0 && state.rem_steps < 11 {
    //     return;
    // }
    if !visited.insert(state) {
        // println!("v: {:?}", state);
        return;
    }

    let max_ore = bp.iter().map(|(_, (ore, _, _))| ore).max().unwrap();
    if state.ore >= state.rem_steps * max_ore - state.r_ore * (state.rem_steps - 1) {
        state.ore = state.rem_steps * max_ore - state.r_ore * (state.rem_steps - 1); 
    }
    if state.clay >= state.rem_steps * bp[1].1.1 - state.r_clay * (state.rem_steps - 1) {
        state.clay = state.rem_steps * bp[1].1.1 - state.r_clay * (state.rem_steps - 1); 
    } 
    
    if state.obsidian >= state.rem_steps * bp[2].1.2 - state.r_obsidian * (state.rem_steps - 1) {    
        state.obsidian = state.rem_steps * bp[2].1.2 - state.r_obsidian * (state.rem_steps - 1);
    }  
    // if visited.len() > 10000000 {
    //     println!("drain");
    //     visited.drain();
    // }
    let next = state.get_available_robots(&bp);
    state.update_resources();
    if state.rem_steps == 1 {
        if state.geode > *max {
            *max = state.geode;
            println!("id: {}, max: {}", id, max);
            println!("\t{:?}", state);
        }
        return;
    }

    state.rem_steps -= 1;
    
    run(state, bp, max, visited, id);
    if let Some(entry) = next.iter().find(|(r, _)| r == &Robot::Geode) {
        let mut new_s = state;
        new_s.build_robot(*entry);
        run(new_s, bp, max, visited, id);
    } else {
        for (robot, cost) in next.into_iter() {
            let should_build = match get_max_resource(&bp, robot) {
                Some(max_resource) => match robot {
                    Robot::Ore => state.r_ore < max_resource,
                    Robot::Clay => state.r_clay < max_resource,
                    Robot::Obsidian => state.r_obsidian < max_resource,
                    _ => unreachable!(),
                },
                None => false,
            };
            if should_build {
                let mut new_s = state;
                new_s.build_robot((robot, cost));
                run(new_s, bp, max, visited, id);
            }
        }
    }
}

fn search(rem_steps: u32, bp: &BluePrint, id: usize) {
    let state = State::new(rem_steps);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut max = 0;
    let mut c = 0_u128;
    queue.push_back(state);
    while !queue.is_empty() {
        let mut state = queue.pop_front().unwrap();
        c += 1;
        if c % 1_000_000 == 0 {
            println!("queue: {}, visited: {}", queue.len(), visited.len());
        }

        if state.rem_steps == 1 {
            if state.geode > max {
                max = state.geode;
                println!("id: {}, max: {}", id, max);
                println!("\t{:?}", state);
                println!("queue: {}", queue.len());
            }
            continue;
        }

        
        // set resources to values in order to build a robot for every remaining round
        if state.ore >= state.rem_steps * state.ore - state.r_ore * (state.rem_steps - 1) {
            state.ore = state.rem_steps * state.ore - state.r_ore * (state.rem_steps - 1); 
        }
        if state.clay >= state.rem_steps * state.clay - state.r_clay * (state.rem_steps - 1) {
            state.clay = state.rem_steps * state.clay - state.r_clay * (state.rem_steps - 1); 
        } 
        
        if state.obsidian >= state.rem_steps * state.obsidian - state.r_obsidian * (state.rem_steps - 1) {
            state.obsidian = state.rem_steps * state.obsidian - state.r_obsidian * (state.rem_steps - 1);
        } 

        if !visited.insert(state) {
            continue;
        }

        let next = state.get_available_robots(&bp);
        state.update_resources();

        state.rem_steps -= 1;
        
        queue.push_back(state);
        for (robot, cost) in next.into_iter() {
            let should_build = match get_max_resource(&bp, robot) {
                Some(max_resource) => match robot {
                    Robot::Ore => state.r_ore < max_resource,
                    Robot::Clay => state.r_clay < max_resource,
                    Robot::Obsidian => state.r_obsidian < max_resource,
                    Robot::Geode => unreachable!(),
                },
                None => true,
            };
            if should_build {
                let mut new_s = state;
                new_s.build_robot((robot, cost));
                queue.push_back(new_s);
            }
        } 

    }
}

fn get_max_resource(bp: &BluePrint, robot: Robot) -> Option<u32> {
    match robot {
        Robot::Ore => bp.iter().map(|entry| entry.1.0).max(),
        Robot::Clay => bp.iter().map(|entry| entry.1.1).max(),
        Robot::Obsidian => bp.iter().map(|entry| entry.1.2).max(),
        Robot::Geode => return None,
    }
}

pub fn get_solution_1() -> usize {
    let bps = parse(INPUT);
    // let mut quality_levels = Vec::new();
    // for (id, bp) in bps.iter().enumerate() {
    //     let state = State::new(24);
    //     let mut max = 0;
    //     run(state, *bp, &mut max, &mut HashSet::new());
    //     println!("id: {id}, max: {max}");
    //     quality_levels.push((id + 1) * max as usize);

    // }
    // println!("{:?}", quality_levels);
    // quality_levels.iter().sum::<usize>()
    1962
}

pub fn get_solution_2() -> u32 {
    let bps = parse(INPUT);
    let mut handles = Vec::new();
    for id in 0..3 {
        let state = State::new(32);
        let mut max = 0;
        let bp = bps[id];
        let handle = std::thread::spawn(move || {
            run(state, bp, &mut max, &mut HashSet::new(), id);
            // search(32, &bp, id);
            println!("max: {max}");
            return max;
        });
        handles.push(handle);
    }
    let mut product = 1;
    for handle in handles {
        product *= handle.join().unwrap();
    }
    // println!("{:?}", handles);
    product
}

#[test]
fn test_run() {
    get_solution_2();
    // let mut scores = HashSet::new();
    // println!("{}", scores.len());
    // println!("{:?}", scores.iter().max());
}