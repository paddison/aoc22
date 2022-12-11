use std::cell::RefCell;

static INPUT: &str = include_str!("../data/d11.txt");
static _TEST: &str = include_str!("../data/d11_test.txt");

struct Monkey {
    items: RefCell<Vec<u64>>,
    op: Op,
    test: u64,
    nb: [usize; 2],
}

impl Monkey {
    fn new(items: RefCell<Vec<u64>>, op: Op, test: u64, nb: [usize; 2]) -> Self {
        Self { items, op, test, nb }
    }

    fn throw_items(&self, others: &[Monkey], count: &mut u64, m: Option<u64>) {
        while !self.items.borrow().is_empty() {
            let cur_worry_lvl = self.items.borrow_mut().remove(0);
            let new_worry_lvl = match m {
                Some(m) => self.op.exec(cur_worry_lvl) % m,
                None => self.op.exec(cur_worry_lvl) / 3,
            };
            let idx = if new_worry_lvl % self.test == 0 { self.nb[0] } else { self.nb[1] };
            others[idx].items.borrow_mut().push(new_worry_lvl);
            *count += 1;
        }
    }
}

fn do_round(monkeys: &[Monkey], inspected: &mut [u64; 8], m: Option<u64>) {
    for (i, monkey) in monkeys.iter().enumerate() {
        monkey.throw_items(monkeys, inspected.get_mut(i).unwrap(), m);
    }
}

fn do_n_rounds(n: u64, with_mod: bool) -> u64 {
    let monkeys = parse(INPUT);
    let mut inspected = [0; 8];
    let m = if with_mod {
            Some(monkeys.iter().map(|m| m.test).product())
        } else {
            None
        };
    
    for _ in 0..n {
        do_round(&monkeys, &mut inspected, m);
    }

    inspected.sort();
    inspected.reverse();
    inspected[0..2].iter().product() 
}

pub fn get_solution_1() -> u64{
    do_n_rounds(20, false)
}

pub fn get_solution_2() -> u64{
    do_n_rounds(10000, true)
}

enum Op {
    Mul(u64),
    Add(u64),
    Sqr,
}

impl Op {
    fn exec(&self, val: u64) -> u64 {
        match self {
            Op::Mul(n) => val * n,
            Op::Add(n) => val + n,
            Op::Sqr => val * val,
        }
    }
}

fn parse(input: &'static str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    for parts in input.lines().collect::<Vec<&str>>().chunks(7) {
        let items = parts[1][parts[1].find(':').unwrap() + 2..].split(',')
                                                               .map(|n| n.trim().parse().unwrap())
                                                               .collect();
        
        let op = match parts[2].split_whitespace().collect::<Vec<&str>>()[..] {
            [_, _, _, _, op, n] => match (op, n) {
                ("*", "old") => Op::Sqr,
                ("*", n) => Op::Mul(n.parse().unwrap()),
                ("+", n) => Op::Add(n.parse().unwrap()),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        let test = parts[3][parts[3].find("by").unwrap() + 3..].parse().unwrap();
        let nb_true = parts[4][parts[4].len() - 1..].parse().unwrap();
        let nb_false = parts[5][parts[5].len() - 1..].parse().unwrap();

        monkeys.push(Monkey::new(RefCell::new(items), op, test, [nb_true, nb_false]))
    }

    monkeys
}