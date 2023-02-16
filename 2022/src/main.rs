use aoc2022::utils;
use std::env;

fn main() {
    let mut args = env::args();

    let day = {
        args.next();

        match args.next() {
            Some(arg) => arg.parse::<u8>().ok(),
            None => None,
        }
    };

    match day {
        Some(day) => print_day(day),
        None => {
            for day in 1..=utils::DAYS {
                print_day(day);
            }
        }
    }
}

fn print_day(day: u8) {
    println!("Day {day}");
    let input = utils::get_input(day);
    let solution = utils::run(day, input);
    println!("Part 1: {}", solution.part1);
    println!("Part 2: {}", solution.part2);
}
