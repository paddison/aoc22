use std::collections::HashMap;

type Dirs = HashMap<&'static str, (Vec<(&'static str, usize)>, Vec<&'static str>)>; // (files, other dirs)

fn parse(input: &'static str) -> Dirs {
    let mut path = Vec::new();
    let mut dirs = HashMap::from([("/", (Vec::new(), Vec::new()))]);
    for instr in input.lines() {
        let parts = instr.split_whitespace().collect::<Vec<&str>>();
        match (parts[0], parts[1]) {
            ("$", "cd") => match parts[2] {
                ".." => { path.pop(); },
                dir => path.push(dir),
            },
            ("$", "ls") => continue,
            ("dir", new_dir) => {
                let cur_dir = path.last().unwrap();
                if !dirs.contains_key(new_dir) {
                    dirs.insert(new_dir, (Vec::new(), Vec::new()));
                    assert!(dirs.contains_key(cur_dir));
                    let (_, other_dirs) = dirs.get_mut(cur_dir).unwrap();
                    other_dirs.push(new_dir);
                };
            },
            (f_size, f_name) => {
                let cur_dir = path.last().unwrap();
                let size = f_size.parse().unwrap();
                assert!(dirs.contains_key(cur_dir));
                let (files, _) = dirs.get_mut(cur_dir).unwrap();
                files.push((f_name, size))
            },
        }
    }

    dirs
}

fn calculate_sizes(dirs: &Dirs) -> HashMap<&'static str, usize> {
    let mut dir_sizes = HashMap::new();
    for (cur_dir, _) in dirs {
        if !dir_sizes.contains_key(cur_dir) {
            let size = calculate_size(cur_dir, dirs);
            dir_sizes.insert(*cur_dir, size);
        }
    }

    dir_sizes
}

fn calculate_size(dir_name: &'static str, dirs: &Dirs) -> usize {
    let (files, children) = dirs.get(dir_name).unwrap();
    let mut size = files.iter().map(|(_, size)| size).sum();
    for child in children {
        size += calculate_size(child, dirs);
    }

    size
}

fn get_less_than_100000(sizes: HashMap<&'static str, usize>) -> usize {
    sizes.into_iter().map(|(_, size)| size).filter(|s| s <= &100000).sum()
}

#[test]
fn test_parse() {
    let input = include_str!("../data/d07.txt");
    let dirs = parse(input);
    println!("{:?}", dirs);
}

#[test]
fn test() {
    let input = include_str!("../data/d07.txt");
    let dirs = parse(input);
    let sizes = calculate_sizes(&dirs); 
    println!("{}", get_less_than_100000(sizes));
}