use std::{collections::HashSet, sync::Arc};

// initial algorithm:
// for each step, build the best bot that can be build right now, or wait.
// then recurse
static INPUT: &str = include_str!("../data/d19.txt");
static _TEST: &str = include_str!("../data/d19_test.txt");

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

impl State {
    fn new(rem_steps: u32) -> Self {
        Self { ore: 0, clay: 0, obsidian: 0, geode: 0, r_ore: 1, r_clay: 0, r_obsidian: 0, r_geode: 0, rem_steps }
    }

    fn get_available_robots(&self, bp: &BluePrint) -> Vec<Entry> {
        if self.can_be_built(bp[3].1) {
            vec![bp[3]]
        } else {
            bp[0..3].iter()
                .filter(|entry| self.can_be_built(entry.1) && self.should_be_built(entry.0, bp))
                .cloned()
                .collect()
        }
    }

    fn can_be_built(&self, (ore, clay, obsidian): (u32, u32, u32)) -> bool {
        self.ore >= ore && self.clay >= clay && self.obsidian >= obsidian
    }

    fn should_be_built(&self, robot: Robot, bp: &BluePrint) -> bool {
        match robot {
            Robot::Ore => self.r_ore < *bp.iter().map(|(_, (ore, _, _))| ore).max().unwrap(),
            Robot::Clay => self.r_clay < bp[2].1.1,
            Robot::Obsidian => self.r_obsidian < bp[3].1.2,
            Robot::Geode => true,
        }
    }

    fn update_resources(&mut self) {
        self.clay += self.r_clay;
        self.ore += self.r_ore;
        self.obsidian += self.r_obsidian;
        self.geode += self.r_geode;
    }

    fn optimize_resources(&mut self, bp: &BluePrint) {
        let max_ore = bp.iter().map(|(_, (ore, _, _))| ore).max().unwrap();
        // only store enough resources, so that a robot could be built in every step
        if self.ore >= self.rem_steps * max_ore - self.r_ore * (self.rem_steps - 1) {
            self.ore = self.rem_steps * max_ore - self.r_ore * (self.rem_steps - 1); 
        }
    
        if self.clay >= self.rem_steps * bp[1].1.1 - self.r_clay * (self.rem_steps - 1) {
            self.clay = self.rem_steps * bp[1].1.1 - self.r_clay * (self.rem_steps - 1); 
        } 
        
        if self.obsidian >= self.rem_steps * bp[2].1.2 - self.r_obsidian * (self.rem_steps - 1) {    
            self.obsidian = self.rem_steps * bp[2].1.2 - self.r_obsidian * (self.rem_steps - 1);
        }  
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

    fn can_reach_cur_max(&self, max: &u32) -> bool {
        self.rem_steps * self.r_geode + self.geode + self.rem_steps * (self.rem_steps + 1) / 2 > *max
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

fn run(mut state: State, bp: BluePrint, max: &mut u32, visited: &mut HashSet<State>) {
    if state.rem_steps == 0 {
        if state.geode > *max {
            *max = state.geode;
        }
        return;
    }

    if !visited.insert(state) {
        return;
    }

    if !state.can_reach_cur_max(max) {
        return;
    }

    let next = state.get_available_robots(&bp);

    state.optimize_resources(&bp);
    state.update_resources();
    state.rem_steps -= 1;
    
    for (robot, cost) in next.into_iter() {
        let mut new_s = state;
        new_s.build_robot((robot, cost));
        run(new_s, bp, max, visited);
    }
    run(state, bp, max, visited);
}

pub fn get_solution_1() -> usize {
    let bps = Arc::new(parse(INPUT));
    let mut quality_levels = Vec::new();
    let n_threads = 4;
    let chunk_size = bps.len() / n_threads;
    let mut handles = Vec::new();

    for i in 0..n_threads {
        let t_bps = Arc::clone(&bps);
        let start = i * chunk_size;
        let end = if i == n_threads - 1 { bps.len() } else { i * chunk_size + chunk_size };

        let handle = std::thread::spawn(move || {
            let mut t_quality_levels = Vec::new();
            for (id, bp) in t_bps[start..end].iter().enumerate() {
                let mut max = 0;
                let state = State::new(24);
                run(state, *bp, &mut max, &mut HashSet::new());
                t_quality_levels.push((id + i * chunk_size + 1) * max as usize);
            }
            return t_quality_levels;
        });

        handles.push(handle);
    }

    for handle in handles {
        quality_levels.append(&mut handle.join().unwrap());
    }

    quality_levels.iter().sum::<usize>()
}

pub fn get_solution_2() -> u32 {
    let bps = parse(INPUT);
    let mut handles = Vec::new();
    for id in 0..3 {
        let state = State::new(32);
        let mut max = 0;
        let bp = bps[id];
        let handle = std::thread::spawn(move || {
            run(state, bp, &mut max, &mut HashSet::new());
            return max;
        });
        handles.push(handle);
    }
    let mut product = 1;
    for handle in handles {
        product *= handle.join().unwrap();
    }
    product
}