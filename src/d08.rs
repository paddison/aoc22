static _USIZE_WIDTH: usize = std::mem::size_of::<usize>() * 8;
static _TREE_WIDTH: usize = 4;
static _TREES_PER_USIZE: usize = _USIZE_WIDTH / _TREE_WIDTH;
static _SHIFT_MASK: usize = 15;

// trees need 4 bits to be stored
// that means one usize can store USIZE_WIDTH / 4 trees
// we can store 16 trees in a usize (if its 64 bits)
// extract tree by shifting it then logical and with 15
// calculate index:
// usually index is calculated by y * dim_x + x
// this yields the number as if it was stored in a normal vec
// since we store 16 trees in one number, we need to calculate
// index / 16 -> gives the index in the vec, index % 16 the position in the number
// index 0 means the 4 most significant bits, so we need to shift by 64 - 4
// for index 1 shift by 64 - 8. index 15 means 64 - 64. 
// so shifts are calculated by USIZE_WIDTH - (i + 1) * 4
struct _Grid4Bit {
    trees: Vec<usize>,
    dim: (usize, usize), // (x, y)
}

impl _Grid4Bit {
    fn _get(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.dim.0 || y >= self.dim.1 {
            return None;
        }
        let (vec_idx, n_idx) = self._actual_idx(x, y);
        let n = self.trees[vec_idx];
        Some(n >> _USIZE_WIDTH - (n_idx + 1) * 4 & _SHIFT_MASK)
    }

    fn _actual_idx(&self, x: usize, y: usize) -> (usize, usize) {
        let idx = self._idx(x, y);
        (idx / _TREES_PER_USIZE, idx % _TREES_PER_USIZE)
    }

    fn _idx(&self, x: usize, y: usize) -> usize {
        x + y * self.dim.0
    }
    
    fn _is_visible(&self, x: usize, y: usize) -> bool {
        let tree_height = self._get(x, y).unwrap();

        self._check_line(true, y, 0, x, tree_height) ||
        self._check_line(true, y, x + 1, self.dim.0, tree_height) ||
        self._check_line(false, x, 0, y, tree_height) ||
        self._check_line(false, x, y + 1, self.dim.1, tree_height)
    }

    // is_x inidicates if we go through a row (true), or column (false)
    fn _check_line(&self, is_x: bool, other_coord: usize, start: usize, finish: usize, height: usize) -> bool {
        for coord in start..finish {
            let other_height = match is_x {
                true => self._get(coord, other_coord),
                false => self._get(other_coord, coord),
            }.unwrap();
            if other_height >= height {
                return false;
            }
        }

        true
    }

    fn _count_visible_trees(&self) -> usize {
        let mut visibles = self.dim.0 * 2 + self.dim.1 * 2 - 4;

        for x in 1..self.dim.0 - 1 {
            for y in 1..self.dim.1 - 1 {
                if self._is_visible(x, y) {
                    visibles += 1;
                }
            }
        }

        visibles
    }

    fn _compute_scenic_score(&self, x: usize, y: usize) -> usize {
        let tree_height = self._get(x, y).unwrap();

        self._viewing_distance(true, y, 0, x, tree_height) *
        self._viewing_distance(true, y, x + 1, self.dim.0, tree_height) *
        self._viewing_distance(false, x, 0, y, tree_height) *
        self._viewing_distance(false, x, y + 1, self.dim.1, tree_height) 
    }

    fn _viewing_distance(&self, is_x: bool, other_coord: usize, start: usize, finish: usize, height: usize) -> usize {
        let iter = if start == 0 {
            (start..finish).rev().collect::<Vec<usize>>()
        } else {
            (start..finish).collect::<Vec<usize>>()
        };

        for (i, coord) in iter.into_iter().enumerate() {
            let other_height = match is_x {
                true => self._get(coord, other_coord),
                false => self._get(other_coord, coord),
            }.unwrap();

            if other_height >= height {
                return i + 1;
            }
        }

        finish - start
    }

    fn _max_scenic_score(&self) -> usize {
        let mut scores = Vec::new();

        for x in 1..self.dim.0 - 1{
            for y in 1..self.dim.1 - 1 {
                scores.push(self._compute_scenic_score(x, y));
            }
        }
        
        *scores.iter().max().unwrap()
    }
}

fn _parse_4_bit(input: &'static str) -> _Grid4Bit {
    let dim_x = input.find('\n').unwrap();
    let dim_y = input.len() / dim_x;

    let numbers = input.lines()
                       .flat_map(|l| l.chars())
                       .map(|c| c.to_digit(10).unwrap())
                       .collect::<Vec<u32>>();
    let trees = _create_bitmap(numbers);

    _Grid4Bit { trees, dim: (dim_x, dim_y) }

}

fn _create_bitmap(numbers: Vec<u32>) -> Vec<usize> {
    let mut bitmap = Vec::new();

    let mut bm_n = 0;
    let mut count = 0;

    for n in numbers.into_iter() {
        if count % _TREES_PER_USIZE == 0 && count != 0 {
            bitmap.push(bm_n);
            bm_n = 0;
        }
        bm_n <<= _TREE_WIDTH;
        bm_n += n as usize;
        count += 1;
    }
    let remaining_shift = (_TREES_PER_USIZE - count % _TREES_PER_USIZE) % _TREES_PER_USIZE;
    bm_n <<= _TREE_WIDTH * remaining_shift;
    bitmap.push(bm_n);

    bitmap
}

pub fn _get_solution_1_4_bit() -> usize {
    let g = _parse_4_bit(include_str!("../data/d08.txt"));
    g._count_visible_trees()
}

pub fn _get_solution_2_4_bit() -> usize {
    let g = _parse_4_bit(include_str!("../data/d08.txt"));
    g._max_scenic_score()
}

struct Grid {
    trees: Vec<u8>,
    dim: (usize, usize),
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.dim.0 || y >= self.dim.1 {
            return None;
        }

        self.trees.get(self.idx(x, y)).cloned()
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        self.dim.0 * y + x
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let tree_height = self.get(x, y).unwrap();

        self.check_line(true, y, 0, x, tree_height) ||
        self.check_line(true, y, x + 1, self.dim.0, tree_height) ||
        self.check_line(false, x, 0, y, tree_height) ||
        self.check_line(false, x, y + 1, self.dim.1, tree_height)
    }

    // is_x inidicates if we go through a row (true), or column (false)
    fn check_line(&self, is_x: bool, other_coord: usize, start: usize, finish: usize, height: u8) -> bool {
        for coord in start..finish {
            let other_height = match is_x {
                true => self.get(coord, other_coord),
                false => self.get(other_coord, coord),
            }.unwrap();
            if other_height >= height {
                return false;
            }
        }

        true
    }


    fn count_visible_trees(&self) -> usize {
        let mut visibles = self.dim.0 * 2 + self.dim.1 * 2 - 4;

        for x in 1..self.dim.0 - 1 {
            for y in 1..self.dim.1 - 1 {
                if self.is_visible(x, y) {
                    visibles += 1;
                }
            }
        }

        visibles
    }
    
    // is_x inidicates if we go through a row (true), or column (false)
    fn compute_scenic_score(&self, x: usize, y: usize) -> usize {
        let tree_height = self.get(x, y).unwrap();

        self.viewing_distance(true, y, 0, x, tree_height) *
        self.viewing_distance(true, y, x + 1, self.dim.0, tree_height) *
        self.viewing_distance(false, x, 0, y, tree_height) *
        self.viewing_distance(false, x, y + 1, self.dim.1, tree_height) 
    }

    fn viewing_distance(&self, is_x: bool, other_coord: usize, start: usize, finish: usize, height: u8) -> usize {
        let iter = if start == 0 {
            (start..finish).rev().collect::<Vec<usize>>()
        } else {
            (start..finish).collect::<Vec<usize>>()
        };

        for (i, coord) in iter.into_iter().enumerate() {
            let other_height = match is_x {
                true => self.get(coord, other_coord),
                false => self.get(other_coord, coord),
            }.unwrap();

            if other_height >= height {
                return i + 1;
            }
        }

        finish - start
    }

    fn max_scenic_score(&self) -> usize {
        let mut scores = Vec::new();

        for x in 1..self.dim.0 - 1{
            for y in 1..self.dim.1 - 1 {
                scores.push(self.compute_scenic_score(x, y));
            }
        }
        
        *scores.iter().max().unwrap()
    }
}

fn parse(input: &str) -> Grid {    
    let dim_x = input.find('\n').unwrap();
    let dim_y = input.len() / dim_x;
    let trees = input.lines().flat_map(|l| l.chars()).map(|c| c.to_digit(10).unwrap() as u8).collect();

    Grid { trees, dim: (dim_x, dim_y)}
}

pub fn get_solution_1() -> usize {
    let g = parse(include_str!("../data/d08.txt"));
    g.count_visible_trees()
}

pub fn get_solution_2() -> usize {
    let g = parse(include_str!("../data/d08.txt"));
    g.max_scenic_score()
}

#[test]
fn test_shift() {
    // trees are             9    5    4    6    3    2    6    7    8    0    1    2    6    4    1    1    2    2    3    8
    let trees: usize = 0b_1001_0101_0100_0110_0011_0010_0110_0111_1000_0000_0001_0010_0110_0100_0001_0001;//_0010_0010_0011_1000;
    let idx = 0 * 99 + 5; // y, x
    let n_idx = idx % (_USIZE_WIDTH / _TREE_WIDTH);
    assert_eq!(n_idx, 5);
    assert_eq!(trees >> _USIZE_WIDTH - (n_idx + 1) * 4 & 15, 2);
}

#[test]
fn test_grid_idx() {
    // trees are                     9    5    4    6    3    2    6    7    8    0    1    2    6    4    1    1    2    2    3    8
    let g = _Grid4Bit { trees: vec![0b_1001_0101_0100_0110_0011_0010_0110_0111_1000_0000_0001_0010_0110_0100_0001_0001], dim: (4, 4) };
    assert_eq!(g._get(0, 0), Some(9));
    assert_eq!(g._get(3, 3), Some(1));
    assert_eq!(g._get(1, 2), Some(0));
    assert!(g._get(3, 4).is_none());
    assert!(g._get(4, 3).is_none());
    assert!(g._get(4, 4).is_none());
}

#[test]
fn test_create_bitmap() {
    let numbers = vec![9, 5, 4, 6, 3, 2, 6, 7, 8, 0, 1, 2, 6, 4, 1, 1, 8];
    let bm = _create_bitmap(numbers);
    assert_eq!(bm[0], 0b_1001_0101_0100_0110_0011_0010_0110_0111_1000_0000_0001_0010_0110_0100_0001_0001);
    assert_eq!(bm[1], 2_usize.pow(63));
}

#[test]
fn test_count_visibles() {
    let g = _parse_4_bit("30373
25512
65332
33549
35390");
    assert_eq!(g._count_visible_trees(), 21);
}

#[test]
fn test_compute_scenic_score() {
    let g = _parse_4_bit("30373
25512
65332
33549
35390"); 
    assert_eq!(g._compute_scenic_score(2, 3), 8);
}

#[test]
fn test_max_scenic_score() {
    let g = _parse_4_bit("30373
25512
65332
33549
35390");
    assert_eq!(g._max_scenic_score(), 8);
}