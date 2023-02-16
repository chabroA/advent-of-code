use crate::AoCData;
use std::collections::VecDeque;

pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn parse(input: &'a str) -> Self {
        Self(input)
    }

    fn part1(&self) -> String {
        let lines = self.0.lines().collect::<Vec<_>>();

        let mut i = 0;
        let mut stacks: Vec<VecDeque<String>> = Vec::new();

        let nb_stacks = (lines[0].len() + 1) / 4;

        for _ in 0..nb_stacks {
            stacks.push(VecDeque::new());
        }

        while lines[i].contains('[') {
            let line = lines[i];
            for x in (0..line.len()).step_by(4) {
                let element: String = line
                    .get(x..x + 3)
                    .unwrap()
                    .trim()
                    .replace(|c| c == '[' || c == ']', "");

                if !element.is_empty() {
                    stacks[x / 4].push_back(element);
                }
            }

            i += 1;
        }
        i += 2;

        while i < lines.len() {
            let line = lines[i];

            let mut parts = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap_or_default())
                .collect::<Vec<_>>();

            parts.retain(|&x| x != 0);

            for _ in 0..parts[0] {
                let el = stacks[parts[1] - 1].pop_front().unwrap();
                stacks[parts[2] - 1].push_front(el);
            }

            i += 1;
        }

        let mut res = String::new();

        for mut stack in stacks {
            res += &stack.pop_front().unwrap();
        }

        res
    }

    fn part2(&self) -> String {
        let lines = self.0.lines().collect::<Vec<_>>();

        let mut i = 0;
        let mut stacks: Vec<VecDeque<String>> = Vec::new();

        let nb_stacks = (lines[0].len() + 1) / 4;

        for _ in 0..nb_stacks {
            stacks.push(VecDeque::new());
        }

        while lines[i].contains('[') {
            let line = lines[i];
            for x in (0..line.len()).step_by(4) {
                let element: String = line
                    .get(x..x + 3)
                    .unwrap()
                    .trim()
                    .replace(|c| c == '[' || c == ']', "");

                if !element.is_empty() {
                    stacks[x / 4].push_back(element);
                }
            }

            i += 1;
        }
        i += 2;

        while i < lines.len() {
            let line = lines[i];

            let mut parts = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap_or_default())
                .collect::<Vec<_>>();

            parts.retain(|&x| x != 0);

            let mut els = stacks[parts[1] - 1].drain(0..parts[0]).collect::<Vec<_>>();
            els.reverse();

            for el in els {
                stacks[parts[2] - 1].push_front(el);
            }

            i += 1;
        }

        let mut res = String::new();

        for mut stack in stacks {
            res += &stack.pop_front().unwrap();
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn part1() {
        let input = utils::get_sample_input(5);
        let data = Data::parse(&input);
        assert_eq!(data.part1(), "CMZ");
    }

    #[test]
    fn part2() {
        let input = utils::get_sample_input(5);
        let data = Data::parse(&input);
        assert_eq!(data.part2(), "MCD");
    }
}
