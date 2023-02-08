use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Providing any arg to the program will run part 2
    let is_part_1 = args.len() == 1;

    let file = File::open("inputs/day5.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let mut i = 0;
    let mut stacks: Vec<VecDeque<String>> = Vec::new();

    let nb_stacks = (lines[0].len() + 1) / 4;

    for _ in 0..nb_stacks {
        stacks.push(VecDeque::new());
    }

    while lines[i].contains('[') {
        let line = lines[i].clone();
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
        let line = lines[i].clone();

        let mut parts = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap_or_default())
            .collect::<Vec<_>>();

        parts.retain(|&x| x != 0);

        if is_part_1 {
            // Part 1
            for _ in 0..parts[0] {
                let el = stacks[parts[1] - 1].pop_front().unwrap();
                stacks[parts[2] - 1].push_front(el);
            }
        } else {
            // Part 2
            let mut els = stacks[parts[1] - 1].drain(0..parts[0]).collect::<Vec<_>>();
            els.reverse();

            for el in els {
                stacks[parts[2] - 1].push_front(el);
            }
        }

        i += 1;
    }

    let mut res = String::new();

    for mut stack in stacks {
        res += &stack.pop_front().unwrap();
    }

    dbg!(res);
}
