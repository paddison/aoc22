// instructions are (amount, from, to)
fn parse(input: &str) -> Vec<(usize, usize, usize)> {
    let mut instructions = Vec::new();

    for line in input.lines().filter(|l| l.starts_with("move")) {
        let parts = line.split_whitespace().filter_map(|l| l.parse::<usize>().ok()).collect::<Vec<usize>>();
        instructions.push((parts[0], parts[1] - 1, parts[2] - 1)) // decrement by one for indexing
    }

    instructions
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    for line in input.lines().filter(|l| !l.starts_with("move") && !l.is_empty()).rev() {
        for (i, ch) in line.chars().skip(1).step_by(4).enumerate() {
            if stacks.len() == i {
                stacks.push(Vec::new());
            }
            if ch.is_alphabetic() {
                stacks[i].push(ch)
            }
        }
    }

    stacks
}

fn execute_instruction_9000((amount, from, to): (usize, usize, usize), crates: &mut Vec<Vec<char>>) {
    for _ in 0..amount {
        if let Some(c) = crates[from].pop() {
            crates[to].push(c)
        }
    }
}

fn execute_instruction_9001((amount, from, to): (usize, usize, usize), crates: &mut Vec<Vec<char>>) {
    let mut moved_crates = Vec::new();
    for _ in 0..amount {
        if let Some(c) = crates[from].pop() {
            moved_crates.push(c)
        }
    }

    moved_crates.reverse();
    crates[to].append(&mut moved_crates);
}

fn get_top_crates(crates: Vec<Vec<char>>) -> String {
    let mut result = String::new();
    for mut stack in crates {
        if let Some(c) = stack.pop() {
            result.push(c);
        }
    }
    result
}

pub fn get_solution_1() -> String {
    let input = include_str!("../data/d05.txt");
    let instructions = parse(input);
    let mut crates = parse_stacks(input);
    for instruction in instructions {
        execute_instruction_9000(instruction, &mut crates);
    }

    get_top_crates(crates)
}

pub fn get_solution_2() -> String {
    let input = include_str!("../data/d05.txt");
    let instructions = parse(input);
    let mut crates = parse_stacks(input);
    for instruction in instructions {
        execute_instruction_9001(instruction, &mut crates);
    }

    get_top_crates(crates)
}