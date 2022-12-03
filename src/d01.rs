fn parse(input: &str) -> Vec<usize> {
    let mut elves = Vec::new();
    let mut elf = Vec::new();
    
    for line in input.lines() {
        if line.len() == 0 {
            elves.push(elf.iter().sum::<usize>());
            elf = Vec::new();
        } else {
            elf.push(line.parse::<usize>().unwrap());
        }
    }

    elves.sort_by(|a, b| a.cmp(&b).reverse());
    elves
} 

pub fn get_solution_1() -> usize {
    parse(include_str!("../data/d01.txt"))[0]
}

pub fn get_solution_2() -> usize {
    parse(include_str!("../data/d01.txt"))[..3].iter().sum()
}