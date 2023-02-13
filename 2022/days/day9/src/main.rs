use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

fn part1() -> usize {
    let input = std::fs::read_to_string("inputs/day9.txt").unwrap();
    let start = Coord { x: 0, y: 0 };
    let mut head = start;
    let mut tail = start;
    let mut positions = HashSet::new();
    positions.insert(tail);

    for line in input.lines() {
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
    positions.len()
}

fn part2() -> usize {
    let input = std::fs::read_to_string("inputs/day9.txt").unwrap();
    let start = Coord { x: 0, y: 0 };
    let mut rope = vec![start; 10];
    let mut positions = HashSet::new();
    positions.insert(start);

    for line in input.lines() {
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

    positions.len()
}

fn main() {
    let res1 = part1();
    let res2 = part2();

    dbg!(res1);
    dbg!(res2);
}
