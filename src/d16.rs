use core::time;
use std::collections::{HashMap, HashSet};

type Valves = HashMap<&'static str, Valve>;

static INPUT: &str = include_str!("../data/d16.txt");
static _TEST: &str = include_str!("../data/d16_test.txt"); 

#[derive(Hash, PartialEq, Eq, Clone)]
struct Valve {
    rate: u64,
    is_closed: bool,
    neighbours: Vec<&'static str>,
}

fn parse(input: &'static str) -> Valves {
    let mut valves = HashMap::new();
    for parts in input.lines().map(|l| l.split_whitespace().collect::<Vec<&str>>()) {
        match &parts[..] {
            [_, id, _, _, rate, _, _, _, _, others @ ..] => {
                let rate = rate.trim_matches(|c: char| !c.is_numeric()).parse().unwrap();
                let neighbours = others.iter().map(|v| v.trim_matches(|c: char| !c.is_ascii_alphabetic())).collect();
                let valve = Valve {
                    rate,
                    is_closed: rate != 0,
                    neighbours
                };
                valves.insert(*id, valve);
            },
            _ => unreachable!()
        }
    }
    valves
}

fn open_valve(id: &str, mut valves: Valves) -> Valves {
    let valve = valves.get_mut(id).unwrap();
    valve.is_closed = false;
    valves 
}

#[inline(always)]
fn is_done(time_left: u64, valves: &Valves) -> bool {
    time_left == 0 || valves.values().all(|v| !v.is_closed)
}

fn do_step(cur: &'static str, time_left: u64, valves: &Valves, score: u64, max: &mut u64, states: &mut HashMap<(&str, u64), u64>) {
    if is_done(time_left, valves){
        if score > *max {
            *max = score;
        }
        return;
    }
    // check if we open valve
    let v = valves.get(&cur).unwrap();
    try_open_valve(v, cur, time_left - 1, valves, score, max, states);

    for n in &v.neighbours {
        // only continue, if we reached the same node before with less time left
        let best_time_left = states.entry((n, score)).or_insert(0);
        if *best_time_left < time_left {
            *best_time_left = time_left;
            do_step(n, time_left - 1, valves, score, max, states);
        }
    }
}

fn try_open_valve(v: &Valve, cur: &'static str, time_left: u64, valves: &Valves, score: u64, max: &mut u64, states: &mut HashMap<(&str, u64), u64>) {
    if !v.is_closed {
        return;
    }
    let new_score = score + v.rate * time_left;
    let best_time_left = states.entry((cur, new_score)).or_insert(0);
    if *best_time_left < time_left {
        *best_time_left = time_left;
        let new_valves = open_valve(cur, valves.clone());
        do_step(cur, time_left, &new_valves, new_score, max, states)
    }
}

fn do_step_eleph(cur: &'static str, e_cur: &'static str, time_left: u64, valves: &Valves, score: u64, max: &mut u64, states: &mut HashMap<(&str, &str, u64), u64>) {
    if is_done(time_left, valves) {
        if score > *max {
            *max = score;
        }
        return;
    }

    let (v, e_v) = (valves.get(&cur).unwrap(), valves.get(&e_cur).unwrap());
    if v.is_closed && e_v.is_closed && cur != e_cur {
        open_both(cur, e_cur, time_left - 1, valves, score, max, states)
    }

    if v.is_closed {
        open_player(cur, e_cur, time_left - 1, valves, score, max, states)
    }

    if e_v.is_closed && cur != e_cur {
        open_eleph(cur, e_cur, time_left - 1, valves, score, max, states)
    }

    go_to_next(cur, e_cur, time_left - 1, valves, score, max, states)
}

fn go_to_next(cur: &'static str, e_cur: &'static str, time_left: u64, valves: &Valves, score: u64, max: &mut u64, states: &mut HashMap<(&str, &str, u64), u64>) {
    let (v, e_v) = (valves.get(&cur).unwrap(), valves.get(&e_cur).unwrap());
    for n in &v.neighbours {
        for e_n in &e_v.neighbours {
            let best_time_left = states.entry((n, e_n, score)).or_insert(0);
            if *best_time_left < time_left {
                *best_time_left = time_left;
                do_step_eleph(n, e_n, time_left, valves, score, max, states)
            }
        }
    }
    do_step_eleph(cur, e_cur, time_left, valves, score, max, states);
}

fn open_player(cur: &'static str, e_cur: &'static str, time_left: u64, valves: &Valves, score: u64, max: &mut u64, states: &mut HashMap<(&str, &str, u64), u64>) {
    // one opens, the other goes to neighbour
    let new_score = score + valves.get(cur).unwrap().rate * time_left as u64;
    for n in &valves.get(e_cur).unwrap().neighbours {
        let best_time_left = states.entry((cur, n, new_score)).or_insert(0);
        if *best_time_left < time_left {
            *best_time_left = time_left;
            let new_valves = open_valve(cur, valves.clone()); 
            do_step_eleph(cur, n, time_left, &new_valves, new_score, max, states)
        }
    } 
}

fn open_eleph(cur: &'static str, e_cur: &'static str, time_left: u64, valves: &Valves, score: u64, max: &mut u64, states: &mut HashMap<(&str, &str, u64), u64>) {
    // one opens, the other goes to neighbour
    let new_score = score + valves.get(e_cur).unwrap().rate * time_left as u64;
    for n in &valves.get(cur).unwrap().neighbours {
        let best_time_left = states.entry((n, e_cur, new_score)).or_insert(0);
        if *best_time_left < time_left {
            *best_time_left = time_left;
            let new_valves = open_valve(e_cur, valves.clone()); 
            do_step_eleph(n, e_cur, time_left, &new_valves, new_score, max, states)
        }
    } 
}

fn open_both(cur: &'static str, e_cur: &'static str, time_left: u64, valves: &Valves, score: u64, max: &mut u64, states: &mut HashMap<(&str, &str, u64), u64>) {
    // one opens, the other goes to neighbour
    let new_score = score + (valves.get(cur).unwrap().rate + valves.get(e_cur).unwrap().rate) * time_left;
    let best_time_left = states.entry((cur, e_cur, new_score)).or_insert(0);
    if *best_time_left < time_left {
        *best_time_left = time_left;
        let new_valves = open_valve(e_cur, open_valve(cur, valves.clone())); 
        do_step_eleph(cur, e_cur, time_left, &new_valves, new_score, max, states)
    }
}

fn max_pressure(valves: &Valves) -> u64 {
    let mut max = 0;
    do_step("AA", 30, valves, 0, &mut max, &mut HashMap::from([(("AA", 0), 30)]));
    max
}

fn max_pressure_eleph(valves: &Valves) -> u64 {
    let mut max = 0;
    do_step_eleph("AA", "AA", 26, valves, 0, &mut max, &mut HashMap::from([(("AA", "AA", 0), 26)]));
    max
}

pub fn get_solution_1() -> u64 {
    let valves = parse(INPUT);
    max_pressure(&valves) 
}

pub fn get_solution_2() -> u64 {
    let valves = parse(INPUT);
    max_pressure_eleph(&valves) 
}

#[test]
fn test() {
    get_solution_2();
}