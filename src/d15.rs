use std::{ops::Range, collections::{HashMap, HashSet}, sync::{Arc, mpsc}};

static INPUT: &str = include_str!("../data/d15.txt");
static _TEST: &str = include_str!("../data/d15_test.txt");

fn parse(input: &'static str) -> Vec<Sensor> {
    let mut sensors = Vec::new();

    for line in input.lines() {
        let nums = line.split_whitespace()
                       .filter(|part| part.contains('='))
                       .map(|part| {
                        let start = part.find('=').unwrap() + 1;
                        let end = part.len() - if part.contains([',', ':']) { 1 } else { 0 };
                        part[start..end].parse::<i64>().unwrap()
                       })
                       .collect::<Vec<i64>>();
        
        sensors.push(Sensor { pos: (nums[0], nums[1]), beacon: (nums[2], nums[3]) });
    }

    sensors
}

#[derive(Debug)]
struct Sensor {
    pos: (i64, i64), // (x, y)
    beacon: (i64, i64)
}


impl Sensor {
    
    fn beacon_dist(&self) -> i64 {
        self.manhattan(self.beacon)
    }

    fn manhattan(&self, other: (i64, i64)) -> i64 {
        (self.pos.0.abs_diff(other.0) + self.pos.1.abs_diff(other.1)) as i64 
    }

    fn contains(&self, other: (i64, i64)) -> bool {
        self.beacon_dist() >= self.manhattan(other)
    }

    // assumes that sensor covers this row
    fn insert_row(&self, row: i64, pos_map: &mut HashMap<i64, Vec<Range<i64>>>) {
        let dist = self.beacon_dist();
        // calculate sensor distance to row
        let row_dist = (row - self.pos.1).abs();
        if row_dist >= dist {
            return;
        }

        let offset = dist - row_dist;
        let beacon_range = self.pos.0 - offset..self.pos.0 + offset + 1;
        Self::add_to_map(beacon_range, row, pos_map)
    }

    fn add_to_map(mut beacon_range: Range<i64>, y: i64, pos_map: &mut HashMap<i64, Vec<Range<i64>>>) {
        if pos_map.contains_key(&y) {
            for range in pos_map.get_mut(&y).unwrap() {
                // check if range is already contained in beacon_range
                if beacon_range.contains(&range.start) && beacon_range.contains(&(range.end - 1)) {
                    range.end = range.start
                }
                // if range contanis start, move current start to end
                if range.contains(&beacon_range.start) {
                    beacon_range.start = range.end;
                }
                if range.contains(&beacon_range.end) {
                    beacon_range.end = range.start
                }
            }
        }
        if !beacon_range.is_empty() {
            let entry = pos_map.entry(y).or_insert(Vec::new());
            entry.push(beacon_range);
        }
    }
}

fn count_y(y: i64, sensors: &[Sensor], pos_map: &HashMap<i64, Vec<Range<i64>>>) -> i64 {
    let mut count = 0;
    // calculate all positions where a beacon cannot be
    for range in pos_map.get(&y).unwrap() {
        count += range.end - range.start;
    }
    // subtract beacons and sensor which are at y
    let mut others = HashSet::new();
    for sensor in sensors {
        if sensor.pos.1 == y {
            others.insert(sensor.pos);
        }
        if sensor.beacon.1 == y {
            others.insert(sensor.beacon);
        }
    }

    count - others.len() as i64
}

pub fn get_solution_1() -> i64 {
    let sensors = parse(INPUT);
    let row = 2_000_000;
    let filtered = sensors.into_iter().filter(|s| ((s.pos.1 - s.beacon_dist())..(s.pos.1 + s.beacon_dist() + 1)).contains(&row)).collect::<Vec<Sensor>>();
    let mut pos_map = HashMap::new();
    for s in &filtered {
        s.insert_row(row, &mut pos_map);
    } 
    count_y(row, &filtered, &pos_map)
}

pub fn get_solution_2() -> i64 {
    let sensors = Arc::new(parse(INPUT));
    let n_threads = 4;
    let chunk_size = 4_000_000 / n_threads;
    let mut handles = Vec::new();
    let (send, recv) = mpsc::channel();
    for i in 0..n_threads {
        let t_sensors = Arc::clone(&sensors);
        let t_send = send.clone();
        let handle = std::thread::spawn(move || {
            let mut pos_map = HashMap::new();
            for row in i * chunk_size..(i + 1) * chunk_size {
                for s in &*t_sensors {
                    s.insert_row(row, &mut pos_map)
                }
                let mut ranges = pos_map.get(&row).unwrap().iter().filter(|r| !r.is_empty()).collect::<Vec<&Range<i64>>>();
                ranges.sort_by(|a, b| a.start.cmp(&b.start));
                if let Some(col) = contains_hole(&ranges) {
                    let _ = t_send.send(col * 4000000 + row);
                }
            }
        });
        handles.push(handle);
    }

    recv.recv().unwrap()
}

fn contains_hole(ranges: &[&Range<i64>]) -> Option<i64> {
    if ranges.len() < 2 {
        return None;
    }

    for (a, b) in ranges.iter().zip(&ranges[1..]) { 
        if b.start - a.end == 1 {
            return Some(a.end)
        }
    }

    None
}