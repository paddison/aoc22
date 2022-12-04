fn parse(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    let mut elf_ids = Vec::new();
    for line in input.lines() {
        let parts = line.split(',').flat_map(|range| range.split('-')).map(|p| p.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        elf_ids.push(((parts[0], parts[1]), (parts[2], parts[3])));
    }

    elf_ids
}

fn is_contained((lhs, rhs): ((usize, usize), (usize, usize))) -> bool {
    lhs.0 <= rhs.0 && lhs.1 >= rhs.1 ||
    rhs.0 <= lhs.0 && rhs.1 >= lhs.1
}

fn does_overlap((lhs, rhs): ((usize, usize), (usize, usize))) -> bool {
    lhs.1 >= rhs.0 && lhs.0 <= rhs.0 || 
    rhs.1 >= lhs.0 && rhs.0 <= lhs.0
}

pub fn get_solution_1() -> usize {
    parse(include_str!("../data/d04.txt")).into_iter().filter(|pair| is_contained(*pair)).count()
}

pub fn get_solution_2() -> usize {
    parse(include_str!("../data/d04.txt")).into_iter().filter(|pair| does_overlap(*pair)).count()
}

#[test]
fn test() {
    println!("{:?}", parse(include_str!("../data/d04.txt")));
}

#[test]
fn test_is_contained() {
    let elves = parse("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");

    assert_eq!(is_contained(elves[0]), false);
    assert_eq!(is_contained(elves[1]), false);
    assert_eq!(is_contained(elves[2]), false);
    assert_eq!(is_contained(elves[3]), true);
    assert_eq!(is_contained(elves[4]), true);
    assert_eq!(is_contained(elves[5]), false);
}

#[test]
fn test_does_overlap() {
    let elves = parse("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"); 

    assert_eq!(does_overlap(elves[0]), false);
    assert_eq!(does_overlap(elves[1]), false);
    assert_eq!(does_overlap(elves[2]), true);
    assert_eq!(does_overlap(elves[3]), true);
    assert_eq!(does_overlap(elves[4]), true);
    assert_eq!(does_overlap(elves[5]), true); 
}