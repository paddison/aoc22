const LINE: [[bool; 4]; 1] = [[true, true, true, true]];
const CROSS: [[bool; 3]; 3] = [[false, true, false],
                                [true, true, true],
                                [false, true, false]];
const L: [[bool; 3]; 3] = [[false, false, false],
                            [false, false, true],
                            [false, true, false]];

const STICK: [[bool; 1]; 4] = [[true],
                                [true],
                                [true],
                                [true]];

const BLOCK: [[bool; 2]; 2] = [[true, true],
                                [true, true]];

const N_SHAPES: usize = 5;
static SHAPE_ORDER: [Shape; N_SHAPES] = [Shape::Line, Shape::Cross, Shape::L, Shape::Stick, Shape::Stick];

/// positions are: (x, y), where 0, 0 is top left of the grid
/// new rows are added at the top
struct Chamber {
    grid: Vec<[bool;7]>,
    cur_shape: usize,
}

impl Chamber {
    fn new() -> Self {
        Self { grid: vec![[false; 7]; 3], cur_shape: 0 }
    }

    /// Adds the necessary rows to the grid for the specified rock
    fn add_rows(&mut self, rock: &Rock) {
        // determine how many rows need to be added
        let n_rows = self.height() - self.get_highest() + rock.height();
        for _ in 0..n_rows {
            self.grid.insert(0, [false; 7]);
        }
    }
    /// Gets highest position where a rock is
    fn get_highest(&self) -> usize {
        self.grid.iter().enumerate().find(|(_, r)| r.iter().any(|r| *r)).map(|(i, _)| i).unwrap_or(self.grid.len())
    } 

    /// return height of the grid
    #[inline(always)]
    fn height(&self) -> usize {
        self.grid.len()
    }

    fn collides(rock: &Rock) -> bool {
        false
    }
}

fn start_next(chamber: &mut Chamber) -> Rock {
    let rock = chamber.next().unwrap();
    chamber.add_rows(&rock);
    rock
}

impl Iterator for Chamber {
    type Item = Rock ;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.cur_shape;
        self.cur_shape = (self.cur_shape + 1) % N_SHAPES;
        Some((&SHAPE_ORDER[idx]).into())
    }
}

struct Rock {
    pos: (usize, usize),
    shape: Vec<Vec<bool>>,
}

impl Rock {
    fn height(&self) -> usize {
       self.shape.len()
    }
}

impl From<&Shape> for Rock {
    fn from(shape: &Shape) -> Self {
        Self { pos: (0, 0), shape: shape.to_vec() }
    }
}

enum Shape {
    Line,
    Cross,
    L,
    Stick,
    Block
}

impl Shape {
    fn to_vec(&self) -> Vec<Vec<bool>> {
        match self {
            Shape::Line => shape_to_vec(LINE),
            Shape::Cross => shape_to_vec(CROSS),
            Shape::L => shape_to_vec(L),
            Shape::Stick => shape_to_vec(STICK),
            Shape::Block => shape_to_vec(BLOCK),
        }
    }
}

fn shape_to_vec<S, I, T>(shape: S) -> Vec<Vec<T>> 
where S: IntoIterator<Item = I>,
      I: IntoIterator<Item = T>,
{
    shape.into_iter().map(|inner| inner.into_iter().collect()).collect()
}