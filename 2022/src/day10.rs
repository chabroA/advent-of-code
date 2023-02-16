use crate::AoCData;

pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn parse(input: &'a str) -> Self {
        Self(input)
    }

    fn part1(&self) -> String {
        let mut cycles = vec![1];

        for line in self.0.lines() {
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

        ((59..cycles.len())
            .step_by(40)
            .map(|i| cycles[i] * i32::try_from(i + 1).unwrap())
            .sum::<i32>()
            + cycles[20 - 1] * 20)
            .to_string()
    }

    fn part2(&self) -> String {
        let mut cycles = vec![1];

        for line in self.0.lines() {
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
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn part1() {
        let input = utils::get_sample_input(10);
        let data = Data::parse(&input);
        assert_eq!(data.part1(), "13140");
    }

    #[test]
    fn part2() {
        let input = utils::get_sample_input(10);
        let data = Data::parse(&input);
        assert_eq!(data.part2(), "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  \n###   ###   ###   ###   ###   ###   ### \n####    ####    ####    ####    ####    \n#####     #####     #####     #####     \n######      ######      ######      ####\n#######       #######       #######     \n ");
    }
}
