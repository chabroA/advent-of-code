use crate::AoCData;
use itertools::Itertools;

pub struct Data(Vec<Vec<u32>>);

pub fn directions(grid: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = grid[y].clone();
    let col: Vec<u32> = grid.iter().map(|r| r[x]).collect();

    let (up, down) = col.split_at(y);
    let (left, right) = row.split_at(x);

    let up = up.iter().rev().copied().collect();
    let down = down[1..].to_vec();
    let left = left.iter().rev().copied().collect();
    let right = right[1..].to_vec();

    [up, down, left, right]
}

impl<'a> AoCData<'a> for Data {
    fn parse(input: &'a str) -> Self {
        Self(
            input
                .lines()
                .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        )
    }

    fn part1(&self) -> String {
        let grid = &self.0;
        let len = grid.len();

        ((1..len - 1)
            .cartesian_product(1..len - 1)
            .map(|(y, x)| {
                let heigh = grid[y][x];
                directions(grid, x, y)
                    .iter()
                    .map(|dir| dir.iter().all(|h| *h < heigh))
                    .any(|visible| visible)
            })
            .filter(|visible| *visible)
            .count()
            + (len - 1) * 4)
            .to_string()
    }

    fn part2(&self) -> String {
        let grid = &self.0;
        let len = grid.len();

        (1..len - 1)
            .cartesian_product(1..len - 1)
            .map(|(y, x)| {
                let heigh = grid[y][x];
                directions(grid, x, y)
                    .iter()
                    .map(|dir| {
                        dir.iter()
                            .position(|h| *h >= heigh)
                            .map(|p| p + 1)
                            .unwrap_or_else(|| dir.len())
                    })
                    .product::<usize>()
            })
            .max()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn part1() {
        let input = utils::get_sample_input(8);
        let data = Data::parse(&input);
        assert_eq!(data.part1(), "21");
    }

    #[test]
    fn part2() {
        let input = utils::get_sample_input(8);
        let data = Data::parse(&input);
        assert_eq!(data.part2(), "8");
    }
}
