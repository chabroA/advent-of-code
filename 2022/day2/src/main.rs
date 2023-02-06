use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(lines: &Vec<String>) {
    let mut scores = HashMap::new();

    scores.insert("A", 1);
    scores.insert("B", 2);
    scores.insert("C", 3);

    scores.insert("X", 1);
    scores.insert("Y", 2);
    scores.insert("Z", 3);

    let mut my_total_score = 0;

    for line in lines {
        let line: Vec<&str> = line.split(' ').collect();

        let oponent_move = line[0];
        let my_move = line[1];

        let outcome = match oponent_move {
            "A" => match my_move {
                "X" => 3,
                "Y" => 6,
                "Z" => 0,
                _ => panic!("Invalid move"),
            },
            "B" => match my_move {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!("Invalid move"),
            },
            "C" => match my_move {
                "X" => 6,
                "Y" => 0,
                "Z" => 3,
                _ => panic!("Invalid move"),
            },
            _ => panic!("Invalid move"),
        };

        let my_score = scores[my_move] + outcome;

        my_total_score += my_score;
    }

    println!("My total score: {my_total_score}");
}

fn part2(lines: &Vec<String>) {
    let mut scores = HashMap::new();

    scores.insert("X", 0);
    scores.insert("Y", 3);
    scores.insert("Z", 6);

    let mut my_total_score = 0;

    for line in lines {
        let line: Vec<&str> = line.split(' ').collect();

        let oponent_move = line[0];
        let outcome = line[1];

        let my_move = match outcome {
            "X" => match oponent_move {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => panic!("Invalid move"),
            },
            "Y" => match oponent_move {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => panic!("Invalid move"),
            },
            "Z" => match oponent_move {
                "A" => 2,
                "B" => 3,
                "C" => 1,
                _ => panic!("Invalid move"),
            },
            _ => panic!("Invalid outcome"),
        };

        let my_score = scores[outcome] + my_move;

        my_total_score += my_score;
    }

    println!("My total score: {my_total_score}");
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    part1(&lines);
    part2(&lines);
}
