use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(lines: &Vec<String>) {
    let mut sum = 0;

    for line in lines {
        let (first, second) = line.split_at(line.len() / 2);

        let mut first = String::from(first);
        first.retain(|c| second.contains(c));

        let c = first.chars().next().unwrap();

        if ('a'..='z').contains(&c) {
            sum += c as i32 % 96;
        } else if ('A'..='Z').contains(&c) {
            sum += c as i32 % 64 + 26;
        }
    }

    dbg!(sum);
}

fn part2(lines: &Vec<String>) {
    let mut sum = 0;

    for x in (0..lines.len()).step_by(3) {
        let mut line = String::from(&lines[x]);
        line.retain(|c| lines[x + 1].contains(c) && lines[x + 2].contains(c));

        let c = line.chars().next().unwrap();

        if ('a'..='z').contains(&c) {
            sum += c as i32 % 96;
        } else if ('A'..='Z').contains(&c) {
            sum += c as i32 % 64 + 26;
        }
    }

    dbg!(sum);
}

fn main() {
    let file = File::open("inputs/day3.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    part1(&lines);
    part2(&lines);
}
