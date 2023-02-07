use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputs/day4.txt").unwrap();
    let reader = BufReader::new(file);

    let mut contains = 0;
    let mut overlaps = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let bounds = line
            .split(',')
            .flat_map(|s| {
                s.split('-')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<i32>>();

        if (bounds[0] <= bounds[2] && bounds[1] >= bounds[3])
            || (bounds[0] >= bounds[2] && bounds[1] <= bounds[3])
        {
            contains += 1;
        }

        if bounds[0] <= bounds[3] && bounds[2] <= bounds[1] {
            overlaps += 1;
        }
    }

    dbg!(contains);
    dbg!(overlaps);
}
