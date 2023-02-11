use itertools::Itertools;

fn parse() -> Vec<Vec<u32>> {
    std::fs::read_to_string("inputs/day8.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn directions(grid: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
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

fn part1() -> usize {
    let grid = parse();
    let len = grid.len();

    (1..len - 1)
        .cartesian_product(1..len - 1)
        .map(|(y, x)| {
            let heigh = grid[y][x];
            directions(&grid, x, y)
                .iter()
                .map(|dir| dir.iter().all(|h| *h < heigh))
                .any(|visible| visible)
        })
        .filter(|visible| *visible)
        .count()
        + (len - 1) * 4
}

fn part2() -> usize {
    let grid = parse();
    let len = grid.len();

    (1..len - 1)
        .cartesian_product(1..len - 1)
        .map(|(y, x)| {
            let heigh = grid[y][x];
            directions(&grid, x, y)
                .iter()
                .map(|dir| {
                    dir.iter()
                        .position(|h| *h >= heigh)
                        .map(|p| p + 1)
                        .unwrap_or_else(|| dir.len())
                })
                .product()
        })
        .max()
        .unwrap()
}

fn main() {
    let res1 = part1();
    dbg!(res1);

    let res2 = part2();
    dbg!(res2);
}
