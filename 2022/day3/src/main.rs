use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let (first, second) = line.split_at(line.len() / 2);

        let mut first = String::from(first);
        first.retain(|c| second.contains(c));

        let c = first.chars().next().unwrap();

        if ('a'..='z').contains(&c) {
            sum += c as i32 % 96;
        }

        if ('A'..='Z').contains(&c) {
            sum += c as i32 % 64 + 26;
        }
    }

    println!("Sum: {sum}");
}
