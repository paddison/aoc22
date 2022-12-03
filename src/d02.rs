fn parse(input: &str) -> Vec<(Shape, Shape)> {
    input.lines().map(|l| l.chars().collect::<Vec<char>>()).map(|v| (v[0].into(), v[2].into())).collect()
}

#[derive(Clone, Copy)]
enum Shape {
    Rock, // win
    Paper, // loose
    Scissors, // draw
}

impl Shape {
    fn score(player: Self, elf: Self) -> usize {
        match elf {
            Self::Rock => match player {
                Self::Rock => 1 + 3,
                Self::Paper => 2 + 6,
                Self::Scissors => 3,
            },
            Self::Paper => match player {
                Self::Rock => 1,
                Self::Paper => 2 + 3,
                Self::Scissors => 3 + 6,
            },
            Self::Scissors => match player {
                Self::Rock => 1 + 6,
                Self::Paper => 2,
                Self::Scissors => 3 + 3, 
            },
        }
    }

    fn score_2(player: Self, elf: Self) -> usize {
        match elf {
            Self::Rock => match player {
                Self::Rock => 3 + 0,        // play scissor
                Self::Paper => 1 + 3,       // play rock
                Self::Scissors => 2 + 6,    // play paper
            },
            Self::Paper => match player {
                Self::Rock => 1 + 0,        // play rock
                Self::Paper => 2 + 3,       // play paper
                Self::Scissors => 3 + 6,    // play scissor
            },
            Self::Scissors => match player {
                Self::Rock => 2 + 0,        // play paper
                Self::Paper => 3 + 3,       // play scissors
                Self::Scissors => 1 + 6,    // play rock
            },
        }
    }
}

impl From<char> for Shape {
    fn from(input: char) -> Self {
        match input {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

pub fn get_solution_1() -> usize {
    parse(include_str!("../data/d02.txt"))
        .into_iter()
        .map(|(elf, player)| Shape::score(player, elf))
        .sum()
}

pub fn get_solution_2() -> usize {
    parse(include_str!("../data/d02.txt"))
        .into_iter()
        .map(|(elf, player)| Shape::score_2(player, elf))
        .sum()
}