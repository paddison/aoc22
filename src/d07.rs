fn parse(input: &'static str) -> Dir {
    add_dir(&input.lines().map(|line| line.split_whitespace().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>(), &mut 0)
}

fn add_dir(inputs: &Vec<Vec<&'static str>>, cursor: &mut usize) -> Dir {
    let cd_cmd = &inputs[*cursor];
    assert_ne!(cd_cmd[2], "..");
    let mut dir = Dir::new(cd_cmd[2]);
    *cursor += 1;

    assert_eq!(inputs[*cursor][1], "ls");
    *cursor += 1;

    add_files(inputs, &mut dir, cursor);
    if *cursor >= inputs.len() {
        return dir;
    }
    while let Some(cmd) = inputs.get(*cursor) {
        if cmd[2] == ".." {
            break;
        }
        dir.add_dir(add_dir(inputs, cursor));
        *cursor += 1;
    } 

    dir
}

fn add_files(inputs: &Vec<Vec<&str>>, dir: &mut Dir, cursor: &mut usize) {
    while inputs[*cursor][0] != "$" {
        match inputs[*cursor][0] {
            "dir" => (),
            n => dir.add_file(n.parse().unwrap()),
        }
        *cursor += 1;
        if *cursor >= inputs.len() {
            break;
        }
    }
}

#[derive(Debug)]
struct Dir {
    _name: &'static str,
    dirs: Vec<Box<Dir>>,
    files: Vec<usize>,
}

impl Dir {
    fn new(name: &'static str) -> Self {
        Self { _name: name, dirs: Vec::new(), files: Vec::new() }
    }

    fn add_dir(&mut self, dir: Dir) {
        self.dirs.push(Box::new(dir));
    }

    fn add_file(&mut self, file: usize) {
        self.files.push(file);
    }

    fn get_size(&self) -> usize {
        let mut size = self.files.iter().sum();
        for dir in &self.dirs {
            size += dir.get_size();
        }
        size
    } 

    fn record_all_sizes(&self, sizes: &mut Vec<usize>) {
        let mut size = self.files.iter().sum();
        for dir in &self.dirs {
            size += dir.get_size();
            dir.record_all_sizes(sizes);
        }

        sizes.push(size);
    }
}

pub fn get_solution_1() -> usize {
    let fs = parse(include_str!("../data/d07.txt"));
    let mut sizes = Vec::new();
    fs.record_all_sizes(&mut sizes);
    sizes.into_iter().filter(|s| *s <= 100000).sum()
}

pub fn get_solution_2() -> usize {
    let fs = parse(include_str!("../data/d07.txt"));
    let mut sizes = Vec::new();
    fs.record_all_sizes(&mut sizes);
    let actual_size = *sizes.iter().max().unwrap();
    let required_space = 30000000 - (70000000 - actual_size);
    sizes.into_iter().filter(|s| *s >= required_space).min().unwrap()
}