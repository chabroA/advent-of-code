use crate::AoCData;
use std::{collections::HashMap, path::PathBuf};

pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn parse(input: &'a str) -> Self {
        Self(input)
    }

    fn part1(&self) -> String {
        let mut dir_sizes = HashMap::new();
        let mut current_path = Vec::new();

        for line in self.0.lines() {
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

        let res: u32 = dir_sizes.values().filter(|&&x| x <= 100_000).sum();

        res.to_string()
    }

    fn part2(&self) -> String {
        let mut dir_sizes = HashMap::new();
        let mut current_path = Vec::new();

        for line in self.0.lines() {
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

        let needed_space = 30_000_000 - (70_000_000 - dir_sizes.get(&PathBuf::from("/")).unwrap());

        let res = dir_sizes
            .values()
            .filter(|&&x| x >= needed_space)
            .min()
            .unwrap();

        res.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn part1() {
        let input = utils::get_sample_input(7);
        let data = Data::parse(&input);
        assert_eq!(data.part1(), "95437");
    }

    #[test]
    fn part2() {
        let input = utils::get_sample_input(7);
        let data = Data::parse(&input);
        assert_eq!(data.part2(), "24933642");
    }
}
