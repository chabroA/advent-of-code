use std::{collections::HashMap, path::PathBuf};

fn main() {
    let input = std::fs::read_to_string("inputs/day7.txt").unwrap();

    let mut dir_sizes = HashMap::new();
    let mut current_path = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();

        match parts[..] {
            ["$", "cd", ".."] => {
                current_path.pop();
            }
            ["$", "cd", dir] => {
                current_path.push(dir);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();

                for i in 0..current_path.len() {
                    let path = PathBuf::from_iter(&current_path[..=i]);
                    *dir_sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => (),
        }
    }

    let res1: u32 = dir_sizes.values().filter(|&&x| x <= 100_000).sum();

    dbg!(res1);

    let needed_space = 30_000_000 - (70_000_000 - dir_sizes.get(&PathBuf::from("/")).unwrap());

    let res2 = dir_sizes
        .values()
        .filter(|&&x| x >= needed_space)
        .min()
        .unwrap();

    dbg!(res2);
}
