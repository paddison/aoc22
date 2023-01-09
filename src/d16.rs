use std::collections::{HashMap, BinaryHeap, HashSet};

type Valves = HashMap<&'static str, (u32, Vec<&'static str>)>;
type ValveDistances =  HashMap<(&'static str, &'static str), u32>;

static INPUT: &str = include_str!("../data/d16.txt");
static _TEST: &str = include_str!("../data/d16_test.txt"); 

fn parse(input: &'static str) -> (Valves, u64, ValveTable) {
    let mut valves = HashMap::new();
    let mut valve_table = Vec::new();
    let opened = 0;
    let mut mask = 1_u64; 
    input.lines().for_each(|l| {
        match &l.split_whitespace().collect::<Vec<&str>>()[..] {
            [_, name, _, _, flow_rate, _, _, _, _, rest @..] => {
                let flow_rate = flow_rate.trim_matches(|c: char| !c.is_numeric()).parse::<u32>().unwrap();
                // valve is already open
                if flow_rate != 0 {
                    valve_table.push((*name, mask));
                    // opened |= mask;
                    mask <<= 1;
                }
                valves.insert(*name, (flow_rate, rest.iter().map(|v| v.trim_matches(',')).collect()));
            },
            _ => unreachable!(),
        }
    });

    (valves, opened, ValveTable::new(valve_table))
}

struct ValveTable {
    _inner: Vec<(&'static str, u64)>,
    len: usize,
}

impl ValveTable {
    fn new(entries: Vec<(&'static str, u64)>) -> Self {
        let len = entries.len();
        Self { _inner: entries, len }
    }

    fn get_mask(&self, valve: &'static str) -> Option<u64> {
        self._inner.iter().find(|(valve_entry, _)| *valve_entry == valve).map(|(_, mask)| *mask)
    }

    fn get_valve(&self, mask: u64) -> Option<&'static str> {
        self._inner.iter().find(|(_, mask_entry)| *mask_entry == mask).map(|(valve, _)| *valve)
    }

    fn get_closed(&self, valves: u64) -> Vec<&'static str> {
        let mut closed = Vec::new();
        let mut mask = 1;
        while mask < 2_u64.pow(self.len as u32) {
            if valves & mask == 0 {
                closed.push(self.get_valve(mask).unwrap());
            }
            mask <<= 1;
        }

        closed
    }

    fn all_open(&self, opened: u64) -> bool {
        opened == 2_u64.pow(self.len as u32) - 1
    }
    
    fn are_disjoint_valves(&self, mut opened_human: u64, mut opened_elephant: u64) -> bool {
        while opened_human != 0 && opened_elephant != 0 {
            if opened_human % 2 == 1 && opened_elephant % 2 == 1 {
                return false;
            }
            opened_human >>= 1;
            opened_elephant >>= 1;
        }
        return true;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    valve: &'static str,
    opened: u64,
    time: u32,
    pressure: u32,
}

impl State {
    fn new(valve: &'static str, time: u32, pressure: u32, opened: u64) -> Self {
        Self { valve, time, pressure, opened }
    }

    fn open_valve(&mut self, valve: &'static str, valve_table: &ValveTable) {
        let mask = valve_table.get_mask(valve).unwrap();
        self.opened |= mask;
    }
}

pub fn get_solution_1() -> u32 {
    let (valves, opened, valve_table) = parse(INPUT);
    let state = State::new("AA", 30, 0, opened);
    let min_distances = minimum_distance_valves(&valves);
    find_max(state, min_distances, &valves, &valve_table)
}

pub fn get_solution_2() -> u32 {
    let (valves, opened, valve_table) = parse(INPUT);
    let min_distances = minimum_distance_valves(&valves);
    let all_states = record_all_states(State::new("AA", 26, 0, opened), min_distances, &valves, &valve_table);
    get_best_combination(all_states, &valve_table)
}

#[derive(Eq, PartialEq, Hash)]
struct Node {
    cost: u32,
    name: &'static str,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost).map(|ordering| ordering.reverse())
    }
}

fn shortest_path(from: &'static str, to: &'static str, valves: &Valves) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let node = Node { cost: 0, name: from };
    queue.push(node);
    while let Some(Node { cost, name }) = queue.pop() {
        if name == to {
            return cost;
        }
        if !visited.insert(name) {
            continue;
        }
        for neighbour in &valves.get(name).unwrap().1 {
            let neighbour_node = Node { cost: cost + 1, name: neighbour };
            queue.push(neighbour_node);
        }
    }
    u32::MAX
}

fn minimum_distance_valves(valves: &Valves) -> ValveDistances {
    let mut valve_distances = HashMap::new();
    let pressure_valves = valves.iter().filter_map(|(valve, (pressure, _))| 
        if pressure >= &0 { 
            Some(*valve) 
        } else { 
            None }
        ).collect::<Vec<&str>>();
    
    for from in &pressure_valves {
        for to in &pressure_valves {
            if from == to {
                continue;
            }
            valve_distances.insert((*from, *to), shortest_path(from, to, valves));
        }
    }
    valve_distances
}

fn find_max(state: State, min_distances: ValveDistances, valves: &Valves, valve_table: &ValveTable) -> u32 {
    let mut queue = vec![state];
    let mut max = 0;
    while let Some(State { valve, opened, time, pressure }) = queue.pop() {
        if time == 0 || valve_table.all_open(opened) {
            max = max.max(pressure);
            continue; 
        }
        for closed in valve_table.get_closed(opened) {
            // open valve + 1
            let distance = min_distances.get(&(valve, closed)).unwrap();
            if time < distance + 1 {
                max = max.max(pressure);
                continue;
            }
            let new_time = time - *distance - 1;
            let new_pressure = valves.get(&closed).unwrap().0 * new_time + pressure;
            let mut new_state = State::new(closed, new_time, new_pressure, opened);

            new_state.open_valve(closed, valve_table);
            queue.push(new_state);
        }
    }
    
    max 
}

fn record_all_states(state: State, min_distances: ValveDistances, valves: &Valves, valve_table: &ValveTable) -> HashMap<u64, u32> { // (valves, pressure)
    let mut all_states = HashMap::new();
    let mut queue = vec![state];

    while let Some(state) = queue.pop() {
        all_states.entry(state.opened)
            .and_modify(|pressure| *pressure = std::cmp::max(*pressure, state.pressure))
            .or_insert(state.pressure);

        for closed in valve_table.get_closed(state.opened) {
            let distance = min_distances.get(&(state.valve, closed)).unwrap();
            // if there is not enough time to reach a valve
            if state.time < distance + 1 {
                continue;
            }
            let new_time = state.time - distance - 1;
            let new_pressure = valves.get(&closed).unwrap().0 * new_time + state.pressure;
            let mut new_state = State::new(closed, new_time, new_pressure, state.opened);
            new_state.open_valve(closed, valve_table);
            queue.push(new_state);
        }
    }

    all_states
}

fn get_best_combination(all_states: HashMap<u64, u32>, valve_table: &ValveTable) -> u32 {
    let mut max = 0;
    for (opened_human, pressure_human) in all_states.iter() {
        for (opened_elephant, pressure_elephant) in all_states.iter() {
            if valve_table.are_disjoint_valves(*opened_human, *opened_elephant) {
                max = std::cmp::max(max, pressure_human + pressure_elephant);
            }
        }
    }
    max
}