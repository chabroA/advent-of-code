use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Providing any arg to the program will run part 2
    let is_part_1 = args.len() == 1;

    let marker = if is_part_1 { 4 } else { 14 };

    let file = File::open("inputs/day6.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    assert!(lines.len() == 1);

    let content = lines[0].clone();

    let chars = content.chars().collect::<Vec<_>>();

    let mut res = 0;

    for i in 0..chars.len() {
        let mut window = chars[i..i + marker].to_vec();

        window.sort();
        window.dedup();

        if window.len() == marker {
            res = i + marker;
            break;
        }
    }

    dbg!(res);
}
