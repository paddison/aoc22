fn parse(input: &'static str) -> Vec<Rucksack> {
    input.lines()
         .map(|line| Rucksack { 
            items: line, 
         })
         .collect()
}

fn divide_into_group(rucksacks: Vec<Rucksack>) -> Vec<[Rucksack; 3]> {
    let mut all_groups = Vec::new();

    for chunk in rucksacks.chunks(3) {
        all_groups.push([chunk[0], chunk[1], chunk[2]]);
    }
 
    all_groups
}

#[derive(Clone, Copy, Debug)]
struct Rucksack {
    items: &'static str,
}

impl Rucksack {
    fn get_double_item(&self) -> char {
        for l_item in self.items[..self.items.len() / 2].chars() {
            for r_item in self.items[self.items.len() / 2..].chars() {
                if l_item == r_item {
                    return l_item
                }
            }
        }
        unreachable!()
    }
    
    fn get_triple(group: [Rucksack; 3]) -> char {
        for item_1 in group[0].items.chars() {
            for item_2 in group[1].items.chars() {
                for item_3 in group[2].items.chars() {
                    if item_1 == item_2 && item_1 == item_3 {
                        return item_1
                    }
                } 
            }
        }
        unreachable!()
    }
}

fn get_score(ch: char) -> u32 {
    if ch.is_lowercase() {
        ch as u32 - 96
    } else {
        ch as u32 - 38
    }
}

pub fn get_solution_1() -> u32 {
    parse(include_str!("../data/d03.txt"))
        .into_iter()
        .map(|rs| get_score(rs.get_double_item()))
        .sum()
}

pub fn get_solution_2() -> u32 {
    divide_into_group(parse(include_str!("../data/d03.txt")))
        .into_iter()
        .map(|g| get_score(Rucksack::get_triple(g)))
        .sum()
}