use std::cmp::Ordering;

static INPUT: &str = include_str!("../data/d13.txt");
static _TEST: &str = include_str!("../data/d13_test.txt");

#[derive(Debug)]
enum Packet {
    List(Vec<Packet>),
    Value(u64),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            _ => false
        }
    }
}

impl Packet {
    fn compare(lhs: &Self, rhs: &Self) -> Ordering {
        use self::Packet::*;

        match (lhs, rhs) {
            (Value(low), Value(high)) => low.cmp(&high),
            (Value(low), rhs) => Self::compare(&List(vec![Value(*low)]), rhs),
            (lhs, Value(high)) => Self::compare(lhs, &List(vec![Value(*high)])),
            (List(l), List(r)) => {
                for (l, r) in l.iter().zip(r) {
                    let cmp = Self::compare(l, r);
                    if !cmp.is_eq() {
                        return cmp;
                    }
                }
                l.len().cmp(&r.len())
            },
        }
    }
}

struct PacketParser {
    tokens: std::vec::IntoIter<&'static str>,
}

impl PacketParser {
    fn new(input: &'static str) -> Self {
        Self { tokens: Self::tokenize(input) }
    }

    fn tokenize(input: &'static str) -> std::vec::IntoIter<&'static str> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().enumerate().peekable();
        while let Some((i, c)) = chars.next() {
            let tk = match c {
                '[' => "[",
                ']' => "]",
                ',' => continue,
                _ => { 
                    let end = loop {
                        match chars.peek() {
                            Some((j, c)) if !c.is_numeric() => break *j,
                            _ => chars.next(),
                        };
                    };
                    &input[i..end]
                },
            };

            tokens.push(tk);
        }
        tokens.into_iter()
    }

    fn parse(&mut self) -> Packet {
        assert_eq!(self.tokens.next(), Some("["));
        Packet::List(self.parse_list())
    }

    fn parse_list(&mut self) -> Vec<Packet> {
        let mut list = Vec::new();
        while let Some(tk) = self.tokens.next() {
            match tk {
                "]" => return list,
                "[" => list.push(Packet::List(self.parse_list())),
                n => list.push(Packet::Value(n.parse::<u64>().unwrap())),
            }
        }

        list
    }
}

fn parse(input: &'static str) -> Vec<(Packet, Packet)> {
    let mut packets = Vec::new();
    for chunk in input.lines().collect::<Vec<&str>>().chunks(3) {
        packets.push((PacketParser::new(chunk[0]).parse(), PacketParser::new(chunk[1]).parse()));
    }
    packets
}

fn sum_indices(pairs: Vec<(Packet, Packet)>) -> usize {
    let mut sum = 0;
    for (i, (lhs, rhs)) in pairs.into_iter().enumerate() {
        if let Ordering::Less = Packet::compare(&lhs, &rhs) {
            sum += i + 1;
        }
    }

    sum
}

fn flatten_pairs(pairs: Vec<(Packet, Packet)>) -> Vec<Packet> {
    pairs.into_iter().flat_map(|(lhs, rhs)| [lhs, rhs]).collect()
}

fn add_divider_packets(packets: &mut Vec<Packet>) {
    packets.push(Packet::List(vec![Packet::List(vec![Packet::Value(2)])]));
    packets.push(Packet::List(vec![Packet::List(vec![Packet::Value(6)])]));
}

fn get_divider_indices(packets: &[Packet]) -> Vec<usize> {
    let mut indices = Vec::new();
    
    for (i, packet) in packets.iter().enumerate() {
        if let Packet::List(l) = packet {
            if let [Packet::List(l)] = &l[..] {
                if let [Packet::Value(2 | 6)] = l[..] {
                    indices.push(i + 1);
                }
            }
        }
    }

    indices
}

pub fn get_solution_1() -> usize {
    sum_indices(parse(INPUT))
}

pub fn get_solution_2() -> usize {
    let mut packets = flatten_pairs(parse(INPUT));
    add_divider_packets(&mut packets);
    packets.sort_by(|lhs, rhs| Packet::compare(lhs, rhs));
    get_divider_indices(&packets).into_iter().product()
}