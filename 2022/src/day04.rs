use crate::AoCData;

pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn parse(input: &'a str) -> Self {
        Self(input)
    }

    fn part1(&self) -> String {
        let mut contains = 0;

        for line in self.0.lines() {
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
        }

        contains.to_string()
    }

    fn part2(&self) -> String {
        let mut overlaps = 0;

        for line in self.0.lines() {
            let bounds = line
                .split(',')
                .flat_map(|s| {
                    s.split('-')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<i32>>();

            if bounds[0] <= bounds[3] && bounds[2] <= bounds[1] {
                overlaps += 1;
            }
        }

        overlaps.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn part1() {
        let input = utils::get_sample_input(4);
        let data = Data::parse(&input);
        assert_eq!(data.part1(), "2");
    }

    #[test]
    fn part2() {
        let input = utils::get_sample_input(4);
        let data = Data::parse(&input);
        assert_eq!(data.part2(), "4");
    }
}
