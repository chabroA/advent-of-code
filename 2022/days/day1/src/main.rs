use itertools::Itertools;

fn part1() -> u32 {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();

    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .filter_map(|s| s.parse::<u32>().ok())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

fn part2() -> u32 {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();

    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .filter_map(|s| s.parse::<u32>().ok())
                .sum::<u32>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
