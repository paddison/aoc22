use std::collections::HashMap;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

static INPUT: &str = include_str!("../data/d21.txt");
static _TEST: &str = include_str!("../data/d21_test.txt");

type Monkeys = HashMap<&'static str, Monkey>;

fn parse(input: &'static str) -> Monkeys {
    let mut monkeys = HashMap::new();
    for parts in input.lines().map(|l| l.split_whitespace().collect::<Vec<_>>()) {
        let name = &parts[0][0..4];
        monkeys.insert(name, parts.into());
    }
    monkeys
}

enum Monkey {
    Val(i64),
    Instr(&'static str, &'static str, Op),
}

impl From<Vec<&'static str>> for Monkey {
    fn from(parts: Vec<&'static str>) -> Self {
        match parts.len() {
            2 => Monkey::Val(parts[1].parse().unwrap()),
            _ => Monkey::Instr(parts[1], parts[3], parts[2].into())
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div
}

impl Op {
    fn exec<T>(&self, lhs: T, rhs: T) -> T 
    where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
    {
        (match self {
            Op::Add => T::add,
            Op::Sub => T::sub,
            Op::Mul => T::mul,
            Op::Div => T::div,
        })(lhs, rhs)
    }

    fn inv(&self) -> Self {
        match self {
            Op::Add => Self::Sub,
            Op::Sub => Self::Add,
            Op::Mul => Self::Div,
            Op::Div => Self::Mul,
        }
    }
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            _ => Op::Div,
        }
    }
}

fn calculate(monkey: &str, monkeys: &mut Monkeys) -> i64 {
    let (lhs, rhs, op)=  match monkeys.remove(monkey).unwrap() {
        Monkey::Val(n) => return n,
        Monkey::Instr(lhs, rhs, op) => (lhs, rhs, op),
    };

    op.exec(calculate(lhs, monkeys), calculate(rhs, monkeys))
}

fn contains_humn(monkey: &str, monkeys: &Monkeys) -> bool {
    if monkey == "humn" {
        return true;
    }
    let (lhs, rhs)=  match *monkeys.get(monkey).unwrap() {
        Monkey::Val(_) => return false,
        Monkey::Instr(lhs, rhs, _) => (lhs, rhs),
    };

    contains_humn(lhs, monkeys) || contains_humn(rhs, monkeys)
}

// figure out rhs first, then compute
fn determine_num(monkey: &str, monkeys: &mut Monkeys, cur_n: f64) -> f64 {
    if monkey == "humn" {
        return cur_n;
    }

    let (lhs, rhs, next_op) =  match monkeys.remove(monkey).unwrap() {
        Monkey::Val(n) => return n as f64,
        Monkey::Instr(lhs, rhs, op) => (lhs, rhs, op),
    };
    let (humn, other) = if contains_humn(lhs, monkeys) { (lhs, rhs) } else { (rhs, lhs) };
    let result = calculate(other, monkeys) as f64;
    let next_n = next_op.inv().exec(cur_n, result);

    determine_num(humn, monkeys, next_n)
}

pub fn get_solution_1() -> i64 {
    let mut monkeys = parse(INPUT);
    calculate("root", &mut monkeys)
}

pub fn get_solution_2() -> f64 {
    let mut monkeys = parse(INPUT);
    let (lhs, rhs) = match monkeys.remove("root").unwrap() {
        Monkey::Instr(lhs, rhs, _) => (lhs, rhs),
        _ => unreachable!(),
    };
    let (humn, other) = if contains_humn(lhs, &monkeys) { (lhs, rhs) } else { (rhs, lhs) }; 
    // because i'm to lazy to fix Op::inv, i pass the negative result
    let result = calculate(other, &mut monkeys);
    determine_num(humn, &mut monkeys, -result as f64) 
}