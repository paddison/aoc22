static KEY: i64 = 811589153;
static INPUT: &str = include_str!("../data/d20.txt");
static _TEST: &str = include_str!("../data/d20_test.txt");

fn parse(input: &str) -> Vec<(i64, usize)>{
	input.lines().enumerate().map(|(i, n)| (n.parse().unwrap(), i)).collect()
}

fn calc_idx(n: i64, idx: i64) -> i64 {
	idx + n
}

fn calc_shift(ring_size: i64, idx: i64) -> i64 {
	(idx % ring_size + ring_size) % ring_size
}

fn shift(nums: &mut Vec<(i64, usize)>, idx: usize) {
	let entry = nums.remove(idx);
	let ring_size = nums.len() as i64;
	let new_idx = calc_shift(ring_size, calc_idx(entry.0, idx as i64)) as usize;
	if new_idx == 0 {
		nums.push(entry);
	} else {
		nums.insert(new_idx, entry);
	}
}

fn determine_idx(nums: &mut [(i64, usize)], pos: usize) -> usize {
    nums.iter().position(|(_, p)| pos == *p).unwrap()
}

fn mix_file(nums: &mut Vec<(i64, usize)>) {
    for pos in 0..nums.len() {
        let idx = determine_idx(nums, pos);
        shift(nums, idx);
    }
}
 
fn coordinates(nums: &Vec<(i64, usize)>) -> i64 {
    let zero = nums.iter().position(|(n, _)| *n == 0).unwrap();
    let ring_size = nums.len();
    let fst = (zero + 1000) % ring_size;
    let snd = (zero + 2000) % ring_size;
    let thd = (zero + 3000) % ring_size;
    
    nums[fst].0 + nums[snd].0 + nums[thd].0
}

fn add_key(nums: &mut [(i64, usize)], key: i64) {
    for entry in nums {
        entry.0 *= key;
    }
}

fn decrypt(nums: &mut Vec<(i64, usize)>) {
    for _ in 0..10 {
        mix_file(nums);
    }
}

pub fn get_solution_1() -> i64 {
    let mut nums = parse(INPUT);
    mix_file(&mut nums);
    coordinates(&nums)
}        
                        
pub fn get_solution_2() -> i64 {
    let mut nums = parse(INPUT);
    add_key(&mut nums, KEY);
    decrypt(&mut nums);
    coordinates(&nums)
}