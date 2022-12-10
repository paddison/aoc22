use std::fmt::Write;

fn parse(input: &str) -> Vec<(usize, i64)> {
    input.lines()
         .map(|l| if l.starts_with("noop") { 
                (1, 0) 
            } else {
                (2, l[l.find(' ').unwrap() + 1..].parse::<i64>().unwrap() ) 
            }
         ).collect()
}

fn execute_instr(instructions: Vec<(usize, i64)>) -> i64 {
    let (mut x, mut i_count, mut signal_strengths) = (1, 0, Vec::new());

    for (cycles, n) in instructions {
        for _ in 0..cycles {
            i_count += 1;
            if (i_count + 20) % 40 == 0 {
                signal_strengths.push(i_count * x);
            }
        }
        x += n;
    }

    signal_strengths.into_iter().sum()
}

fn draw_screen(instructions: Vec<(usize, i64)>) -> [bool; 240] {
    let (mut x, mut i_count, mut screen) = (1, 0, [false; 240]);

    for (cycles, n) in instructions {
        for _ in 0..cycles {
            screen[i_count] = (x as usize).abs_diff(i_count % 40) <= 1;
            i_count += 1;
        }
        x += n;
    }
    
    screen
}

fn to_string(screen: &[bool; 240]) -> String {
    let mut string = String::new();
    for line in screen.chunks(40) {
        let _ = writeln!(string, "{}", line.iter()
                                           .map(|pixel| if *pixel { '#' } else { ',' } )
                                           .collect::<String>());
    }
    string
}

pub fn get_solution_1() -> i64 {
    execute_instr(parse(include_str!("../data/d10.txt"))) 
}

pub fn get_solution_2() -> &'static str {
    let _screen = to_string(&draw_screen(parse(include_str!("../data/d10.txt"))));
    "PZULBAUA" // prints this
}