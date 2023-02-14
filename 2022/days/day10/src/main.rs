fn part1() -> i32 {
    let mut cycles = vec![1];

    let input = std::fs::read_to_string("inputs/day10.txt").unwrap();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();

        let register = cycles[cycles.len() - 1];

        match parts[..] {
            ["noop"] => cycles.push(register),
            ["addx", value] => {
                let new_value = register + value.parse::<i32>().unwrap();

                cycles.push(register);
                cycles.push(new_value);
            }
            _ => panic!("Invalid instruction"),
        }
    }

    (59..cycles.len())
        .step_by(40)
        .map(|i| cycles[i] * i32::try_from(i + 1).unwrap())
        .sum::<i32>()
        + cycles[20 - 1] * 20
}

fn part2() -> Vec<String> {
    let mut cycles = vec![1];

    let input = std::fs::read_to_string("inputs/day10.txt").unwrap();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();

        let register = cycles[cycles.len() - 1];

        match parts[..] {
            ["noop"] => cycles.push(register),
            ["addx", value] => {
                let new_value = register + value.parse::<i32>().unwrap();

                cycles.push(register);
                cycles.push(new_value);
            }
            _ => panic!("Invalid instruction"),
        }
    }

    cycles
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let i = i32::try_from(i).unwrap() % 40;

            match (x - 1..=x + 1).contains(&i) {
                true => '#',
                _ => ' ',
            }
        })
        .collect::<Vec<_>>()
        .chunks(40)
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<String>>()
}

fn main() {
    let res1 = part1();
    let res2 = part2();

    dbg!(res1);
    dbg!(res2);
}
