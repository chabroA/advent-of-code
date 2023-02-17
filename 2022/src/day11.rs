use crate::AoCData;
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
}

#[derive(Debug, Clone)]
enum Operation {
    Mul(Value),
    Add(Value),
}

impl Operation {
    fn compute(&self, old: u64) -> u64 {
        match self {
            Operation::Mul(val) => old * val.num(old),
            Operation::Add(val) => old + val.num(old),
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    Constant(u64),
    Old,
}

impl Value {
    fn num(&self, old: u64) -> u64 {
        match self {
            Value::Constant(x) => *x,
            Value::Old => old,
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    divisor: u64,
    true_new_monkey_index: u64,
    false_new_monkey_index: u64,
}

impl Test {
    fn get_monkey_index(&self, value: u64) -> u64 {
        if value % self.divisor == 0 {
            self.true_new_monkey_index
        } else {
            self.false_new_monkey_index
        }
    }
}

pub struct Data(Vec<Monkey>);

impl<'a> AoCData<'a> for Data {
    fn parse(input: &'a str) -> Self {
        let monkeys = input
            .split("\n\n")
            .map(|monkey_desc| {
                let mut iter = monkey_desc.lines().skip(1);

                let items = iter
                    .next()
                    .unwrap()
                    .replace("Starting items: ", "")
                    .split(',')
                    .map(|val| val.trim().parse::<u64>().unwrap())
                    .collect::<VecDeque<_>>();

                let operation_tokens = iter
                    .next()
                    .unwrap()
                    .replace("Operation: new = ", "")
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();

                let operation = match operation_tokens
                    .iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<_>>()[..]
                {
                    ["old", "*", value] => {
                        let value = value.parse::<u64>().ok();

                        let operation: Operation = match value {
                            Some(val) => Operation::Mul(Value::Constant(val)),
                            None => Operation::Mul(Value::Old),
                        };

                        operation
                    }
                    ["old", "+", value] => {
                        let value = value.parse::<u64>().ok();

                        let operation: Operation = match value {
                            Some(val) => Operation::Add(Value::Constant(val)),
                            None => Operation::Add(Value::Old),
                        };

                        operation
                    }
                    _ => panic!("Unknown operation"),
                };

                let test_args = iter
                    .map(|s| s.split_whitespace().last().unwrap().parse::<u64>().unwrap())
                    .collect::<Vec<_>>();

                if test_args.len() != 3 {
                    panic!("Invalid test args: {test_args:?}");
                }

                let test = Test {
                    divisor: test_args[0],
                    true_new_monkey_index: test_args[1],
                    false_new_monkey_index: test_args[2],
                };

                Monkey {
                    items,
                    operation,
                    test,
                }
            })
            .collect();

        Self(monkeys)
    }

    fn part1(&self) -> String {
        let monkeys = &mut self.0.clone();

        let mut inspected = vec![0; monkeys.len()];

        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let monkey = monkeys[i].clone();

                for item in &monkey.items {
                    inspected[i] += 1;
                    let item = monkey.operation.compute(*item) / 3;
                    let new_monkey_index = monkey.test.get_monkey_index(item);
                    let new_monkey = &mut monkeys[new_monkey_index as usize];
                    new_monkey.items.push_back(item);
                    monkeys[i].items.pop_front();
                }
            }
        }

        inspected
            .iter()
            .sorted()
            .rev()
            .take(2)
            .product::<u64>()
            .to_string()
    }

    fn part2(&self) -> String {
        let monkeys = &mut self.0.clone();

        let mut inspected = vec![0; monkeys.len()];

        let common_divisor: u64 = monkeys.iter().map(|monkey| monkey.test.divisor).product();

        for _ in 0..10_000 {
            for i in 0..monkeys.len() {
                let monkey = monkeys[i].clone();

                for item in &monkey.items {
                    inspected[i] += 1;
                    let item = monkey.operation.compute(*item) % common_divisor;
                    let new_monkey_index = monkey.test.get_monkey_index(item);
                    let new_monkey = &mut monkeys[new_monkey_index as usize];
                    new_monkey.items.push_back(item);
                    monkeys[i].items.pop_front();
                }
            }
        }

        inspected
            .iter()
            .sorted()
            .rev()
            .take(2)
            .product::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn part1() {
        let input = utils::get_sample_input(11);
        let data = Data::parse(&input);

        assert_eq!(data.part1(), "10605");
    }

    #[test]
    fn part2() {
        let input = utils::get_sample_input(11);
        let data = Data::parse(&input);

        assert_eq!(data.part2(), "2713310158");
    }
}
