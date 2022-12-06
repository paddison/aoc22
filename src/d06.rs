fn find_first_packet_marker(input: &str) -> usize {
    for (i, (((ch1, ch2), ch3), ch4)) in input.chars().zip(input[1..].chars()).zip(input[2..].chars()).zip(input[3..].chars()).enumerate() {
        if ch1 != ch2 && ch1 != ch3 && ch1 != ch4 &&
           ch2 != ch3 && ch2 != ch4 &&
           ch3 != ch4 {
            return i + 4
           }
    }

    unreachable!()
}

fn find_first_msg_marker(input: &str) -> usize {
    let mut uniques = Vec::new();
    for (i, ch) in input.chars().enumerate() {
        if uniques.contains(&ch) {
            uniques = Vec::new();
        }
        uniques.push(ch);
        if uniques.len() == 14 {
            return i + 1;
        }
    }
    
    unreachable!()
}

pub fn get_solution_1() -> usize {
    find_first_packet_marker(include_str!("../data/d06.txt"))
}

pub fn get_solution_2() -> usize {
    find_first_msg_marker(include_str!("../data/d06.txt"))
}