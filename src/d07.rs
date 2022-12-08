use std::{fmt::{ Display, Write }, collections::HashMap, path::PathBuf};

fn parse(input: &'static str) -> Dir {
    add_dir(&input.lines().map(|line| line.split_whitespace().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>(), &mut 0)
}

fn add_dir(inputs: &Vec<Vec<&'static str>>, cursor: &mut usize) -> Dir {
    let cd_cmd = &inputs[*cursor];
    let mut dir = Dir::new(cd_cmd[2]);
    *cursor += 1;
    *cursor += 1; // skip past ls

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

fn add_files(inputs: &Vec<Vec<&'static str>>, dir: &mut Dir, cursor: &mut usize) {
    while let Some(cmd) = inputs.get(*cursor) {
        match cmd[0] {
            "$" => break,
            "dir" => (),
            n => dir.add_file(File::new(cmd[1], n.parse().unwrap())),
        }
        *cursor += 1;
    }
}

#[derive(Debug)]
struct File {
    name: &'static str,
    size: usize,
}

impl File {
    fn new(name: &'static str, size: usize) -> Self {
        Self { name, size }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.name, self.size)
    }
}

#[derive(Debug)]
struct Dir {
    _name: &'static str,
    dirs: Vec<Box<Dir>>,
    files: Vec<File>,
}

impl Dir {
    fn new(name: &'static str) -> Self {
        Self { _name: name, dirs: Vec::new(), files: Vec::new() }
    }

    fn add_dir(&mut self, dir: Dir) {
        self.dirs.push(Box::new(dir));
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn record_all_sizes(&self, sizes: &mut Vec<usize>) -> usize {
        let mut size = self.files.iter().map(|f| f.size).sum();

        for dir in &self.dirs {
            size += dir.record_all_sizes(sizes);
        }

        sizes.push(size);
        size
    }

    fn get_string(&self, indent_lvl: usize) -> String {
        let width = indent_lvl * 2;
        let mut string = String::new();
        let _ = writeln!(string, "{:width$}{}:", "", self._name);
        for f in &self.files {
            let _ = writeln!(string, "{:width$}  {}", "", f);
        }

        for sub_dir in &self.dirs {
            let _ = write!(string, "{}", sub_dir.get_string(indent_lvl + 1));
        }

        string
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_string(0))
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

// implement it via hashmap
type _FileSystem = HashMap<PathBuf, (Vec<usize>, Vec<PathBuf>)>;

fn _parse_hash_map(input: &'static str) -> _FileSystem {
    let mut fs = HashMap::new();
    let mut path = PathBuf::new();
    for parts in input.lines().map(|l| l.split_whitespace().collect::<Vec<&str>>()) {
        match parts[..] {
            ["$", "cd", "..", ..] => { path.pop(); },
            ["$", "cd", dir, ..] => path.push(dir),
            ["$", "ls", ..] => continue,
            [..] => { 
                match parts[..] {
                    ["dir", dir_name, ..] => {
                        let mut new_dir = path.clone();
                        new_dir.push(dir_name);
                        let (_, sub_dirs) = fs.entry(path.clone()).or_insert((Vec::new(), Vec::new()));
                        sub_dirs.push(new_dir)
                    }
                    [size, ..] => {
                        let (files, _) = fs.entry(path.clone()).or_insert((Vec::new(), Vec::new()));
                        files.push(size.parse().unwrap())
                    },
                    [..] => continue,
                }
            },
        }
    }

    fs
}

fn _get_sizes<'file>(fs: &'file _FileSystem) -> HashMap<&'file PathBuf, usize> {
    let mut sizes = HashMap::new();
    for k in fs.keys() {
        if !sizes.contains_key(k) {
            let size = _calculate_size(fs, k, &mut sizes);
            sizes.insert(k, size);
        }
    }
    sizes
}

fn _calculate_size<'file>(fs: &'file _FileSystem, k: &PathBuf, sizes: &mut HashMap<&'file PathBuf, usize>) -> usize {
    let (files, children) = fs.get(k).unwrap();
    let mut size = files.iter().sum();

    for child in children {
        if sizes.contains_key(child) {
            size += sizes.get(child).unwrap();
        } else {
            size += _calculate_size(fs, child, sizes);
        }
    }

    size
}

fn _get_solution_map_1() -> usize {
    let fs = _parse_hash_map(include_str!("../data/d07.txt"));
    _get_sizes(&fs).values().filter(|v| **v <= 100000).sum()
}

fn _get_solution_map_2() -> usize {
    let fs = _parse_hash_map(include_str!("../data/d07.txt"));
    let sizes = _get_sizes(&fs);
    let max = sizes.get(&PathBuf::from("/")).unwrap();
    let required_space = 30000000 - (70000000 - max);
    *sizes.values().filter(|v| **v >= required_space).min().unwrap()
}