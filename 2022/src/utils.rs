use std::fs;

pub const DAYS: u8 = 11;

pub struct Solution {
    pub part1: String,
    pub part2: String,
}

fn format_day(day: u8) -> String {
    if day < 10 {
        format!("0{day}")
    } else {
        day.to_string()
    }
}

pub fn get_input(day: u8) -> String {
    let day = format_day(day);
    fs::read_to_string(format!("inputs/day{day}.txt")).unwrap()
}

pub fn get_sample_input(day: u8) -> String {
    let day = format_day(day);
    fs::read_to_string(format!("inputs/day{day}-sample.txt")).unwrap()
}

pub trait AoCData<'a> {
    fn parse(input: &'a str) -> Self
    where
        Self: Sized;

    fn solve(self) -> Solution
    where
        Self: Sized,
    {
        Solution {
            part1: self.part1(),
            part2: self.part2(),
        }
    }

    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

pub fn run_day<'a, T: AoCData<'a>>(input: &'a str) -> Solution {
    T::parse(input).solve()
}

pub fn run(day: u8, input: String) -> Solution {
    match day {
        1 => run_day::<crate::day01::Data>(&input),
        2 => run_day::<crate::day02::Data>(&input),
        3 => run_day::<crate::day03::Data>(&input),
        4 => run_day::<crate::day04::Data>(&input),
        5 => run_day::<crate::day05::Data>(&input),
        6 => run_day::<crate::day06::Data>(&input),
        7 => run_day::<crate::day07::Data>(&input),
        8 => run_day::<crate::day08::Data>(&input),
        9 => run_day::<crate::day09::Data>(&input),
        10 => run_day::<crate::day10::Data>(&input),
        11 => run_day::<crate::day11::Data>(&input),
        _ => panic!("Day {day} not implemented"),
    }
}
