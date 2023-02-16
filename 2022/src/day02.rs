use crate::AoCData;

pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn parse(input: &'a str) -> Self {
        Self(input)
    }

    fn part1(&self) -> String {
        self.0
            .lines()
            .map(|line| {
                // transform A B C and X Y Z to 0 1 2 respectively by using their ASCII order
                let bytes = line.as_bytes();
                let left = (bytes[0] - b'A') as i8;
                let right = (bytes[2] - b'X') as i8;

                // 0 for rock, 1 for paper, 2 for scissors
                // 0 for loss, 1 for draw, 2 for win
                let oponent_shape = left;
                let my_shape = right;
                let outcome = (my_shape - oponent_shape + 1).rem_euclid(3);

                let shape_score = my_shape + 1;
                let outcome_score = 3 * outcome;
                (shape_score + outcome_score) as u32
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.0
            .lines()
            .map(|line| {
                // transform A B C and X Y Z to 0 1 2 respectively by using their ASCII order
                let bytes = line.as_bytes();
                let left = (bytes[0] - b'A') as i8;
                let right = (bytes[2] - b'X') as i8;

                // 0 for rock, 1 for paper, 2 for scissors
                // 0 for loss, 1 for draw, 2 for win
                let oponent_shape = left;
                let outcome = right;
                let my_shape = (oponent_shape - 1 + outcome).rem_euclid(3);

                let shape_score = my_shape + 1;
                let outcome_score = 3 * outcome;
                (shape_score + outcome_score) as u32
            })
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn part1() {
        let input = utils::get_sample_input(2);
        let data = Data::parse(&input);
        assert_eq!(data.part1(), "15");
    }

    #[test]
    fn part2() {
        let input = utils::get_sample_input(2);
        let data = Data::parse(&input);
        assert_eq!(data.part2(), "12");
    }
}
