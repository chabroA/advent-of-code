use crate::AoCData;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Data<'a>(&'a str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl<'a> AoCData<'a> for Data<'a> {
    fn parse(input: &'a str) -> Self {
        Self(input)
    }

    fn part1(&self) -> String {
        let start = Coord { x: 0, y: 0 };
        let mut head = start;
        let mut tail = start;
        let mut positions = HashSet::new();
        positions.insert(tail);

        for line in self.0.lines() {
            let (dir, steps) = line.split_once(' ').unwrap();
            let steps: u8 = steps.parse().unwrap();

            for _ in 0..steps {
                match dir {
                    "U" => head.y += 1,
                    "D" => head.y -= 1,
                    "R" => head.x += 1,
                    "L" => head.x -= 1,
                    _ => unreachable!("invalid direction"),
                }

                let diff = Coord {
                    x: head.x - tail.x,
                    y: head.y - tail.y,
                };

                if diff.x.abs() > 1 || diff.y.abs() > 1 {
                    tail.x += diff.x.signum();
                    tail.y += diff.y.signum();
                    positions.insert(tail);
                }
            }
        }
        positions.len().to_string()
    }

    fn part2(&self) -> String {
        let start = Coord { x: 0, y: 0 };
        let mut rope = vec![start; 10];
        let mut positions = HashSet::new();
        positions.insert(start);

        for line in self.0.lines() {
            let (dir, steps) = line.split_once(' ').unwrap();
            let steps: u8 = steps.parse().unwrap();

            for _ in 0..steps {
                match dir {
                    "U" => rope[0].y += 1,
                    "D" => rope[0].y -= 1,
                    "R" => rope[0].x += 1,
                    "L" => rope[0].x -= 1,
                    _ => unreachable!("invalid direction"),
                }

                for (head_idx, tail_idx) in (0..rope.len()).tuple_windows() {
                    let diff = Coord {
                        x: rope[head_idx].x - rope[tail_idx].x,
                        y: rope[head_idx].y - rope[tail_idx].y,
                    };

                    if diff.x.abs() > 1 || diff.y.abs() > 1 {
                        rope[tail_idx].x += diff.x.signum();
                        rope[tail_idx].y += diff.y.signum();
                        if tail_idx == rope.len() - 1 {
                            positions.insert(rope[tail_idx]);
                        }
                    }
                }
            }
        }

        positions.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn part1() {
        let input = utils::get_sample_input(9);
        let data = Data::parse(&input);
        assert_eq!(data.part1(), "13");
    }

    #[test]
    fn part2() {
        let input = utils::get_sample_input(9);
        let data = Data::parse(&input);
        assert_eq!(data.part2(), "1");
    }
}
