use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut elf_calories = vec![0];
    let mut elf_index = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let calories = line.trim().parse::<i32>().unwrap_or(0);
        if calories == 0 {
            elf_index += 1;
            elf_calories.push(0);
        } else {
            elf_calories[elf_index] += calories;
        }
    }

    let max_calories = elf_calories.iter().max().unwrap();
    println!("Max calories: {max_calories}");

    elf_calories.sort();
    let test: i32 = elf_calories.iter().rev().take(3).sum();
    println!("Top 3: {test}");
}
