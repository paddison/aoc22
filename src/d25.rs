static INPUT: &str = include_str!("../data/d25.txt");
static _TEST: &str = include_str!("../data/d25_test.txt");

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.trim().chars().collect()).collect()
}

fn to_dec(snafu: Vec<char>) -> u64 {
    let mut exp = snafu.len() - 1;
    let mut n = 0;
    for c in snafu {
        match c {
            '1' => n += 5_u64.pow(exp as u32),
            '2' => n += 2 * 5_u64.pow(exp as u32),
            '-' => n -= 5_u64.pow(exp as u32),
            '=' => n -= 2 * 5_u64.pow(exp as u32),
            _ => (),
        }
        if exp == 0 {
            return n;
        }
        exp -= 1;
    }

    unreachable!();
}

fn sum_snafus(snafus: Vec<Vec<char>>) -> u64 {
    snafus.into_iter().map(|snafu| to_dec(snafu)).sum()
}

fn to_snafu(n: u64) -> String {
    // determine largest exp
    let mut exp = 0;
    let cmp = 0;
    loop {
        let new = cmp + 2 * 5_u64.pow(exp);
        if new > n {
            break exp;
        }
        exp += 1;

    };
    "".to_owned()
}

#[test]
fn test_calc_num() {
    let snafus = parse(_TEST);
    println!("{}", sum_snafus(snafus));
}