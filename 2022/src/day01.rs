use itertools::Itertools;

use crate::AoCData;

pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn parse(input: &'a str) -> Self {
        Self(input)
    }

    fn part1(&self) -> String {
        self.0
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .max()
            .unwrap()
            .to_string()
    }

    fn part2(&self) -> String {
        self.0
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .sorted()
            .rev()
            .take(3)
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn part1() {
        let input = utils::get_sample_input(1);
        let data = Data::parse(&input);
        assert_eq!(data.part1(), "24000");
    }

    #[test]
    fn part2() {
        let input = utils::get_sample_input(1);
        let data = Data::parse(&input);
        assert_eq!(data.part2(), "45000");
    }
}
