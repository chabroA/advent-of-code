use crate::AoCData;

pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn parse(input: &'a str) -> Self {
        Self(input)
    }

    fn part1(&self) -> String {
        let lines = self.0;
        let mut sum = 0;

        for line in lines.lines() {
            let (first, second) = line.split_at(line.len() / 2);

            let mut first = String::from(first);
            first.retain(|c| second.contains(c));

            let c = first.chars().next().unwrap();

            if ('a'..='z').contains(&c) {
                sum += c as i32 % 96;
            } else if ('A'..='Z').contains(&c) {
                sum += c as i32 % 64 + 26;
            }
        }

        sum.to_string()
    }

    fn part2(&self) -> String {
        let lines = self.0.lines().collect::<Vec<_>>();
        let mut sum = 0;

        for x in (0..lines.len()).step_by(3) {
            let mut line = String::from(lines[x]);
            line.retain(|c| lines[x + 1].contains(c) && lines[x + 2].contains(c));

            let c = line.chars().next().unwrap();

            if ('a'..='z').contains(&c) {
                sum += c as i32 % 96;
            } else if ('A'..='Z').contains(&c) {
                sum += c as i32 % 64 + 26;
            }
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn part1() {
        let input = utils::get_sample_input(3);
        let data = Data::parse(&input);
        assert_eq!(data.part1(), "157");
    }

    #[test]
    fn part2() {
        let input = utils::get_sample_input(3);
        let data = Data::parse(&input);
        assert_eq!(data.part2(), "70");
    }
}
