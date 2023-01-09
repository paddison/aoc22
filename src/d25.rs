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

fn to_base_5(mut n: u64) -> Vec<u32> {
    let mut n_string = Vec::new();

    while n > 0 {
        n_string.push((n % 5) as u32);
        n /= 5;
    }

    n_string
}

fn to_snafu(mut n_base_5: Vec<u32>) -> String {
    // determine largest exp
    let mut snafu = Vec::new();

    for i in 0..n_base_5.len() {
        match n_base_5[i] {
            5 => {
                n_base_5[i + 1] += 1;
                snafu.push('0');
            }
            4 => {
                n_base_5[i + 1] += 1;
                snafu.push('-');
            },
            3 => {
                n_base_5[i + 1] += 1;
                snafu.push('=');
            },
            n => snafu.push(char::from_digit(n, 10).unwrap()),
        }
    }

    snafu.iter().rev().collect()
}

pub fn get_solution_1() -> String {
    to_snafu(to_base_5(parse(INPUT).into_iter().map(to_dec).sum()))
}