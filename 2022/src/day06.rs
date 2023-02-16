use crate::AoCData;

pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn parse(input: &'a str) -> Self {
        Self(input)
    }

    fn part1(&self) -> String {
        const MARKER: usize = 4;

        let lines = self.0.lines().collect::<Vec<_>>();

        assert!(lines.len() == 1);

        let content = lines[0];

        let chars = content.chars().collect::<Vec<_>>();

        let mut res = 0;

        for i in 0..chars.len() {
            let mut window = chars[i..i + MARKER].to_vec();

            window.sort();
            window.dedup();

            if window.len() == MARKER {
                res = i + MARKER;
                break;
            }
        }

        res.to_string()
    }

    fn part2(&self) -> String {
        const MARKER: usize = 14;

        let lines = self.0.lines().collect::<Vec<_>>();

        assert!(lines.len() == 1);

        let content = lines[0];

        let chars = content.chars().collect::<Vec<_>>();

        let mut res = 0;

        for i in 0..chars.len() {
            let mut window = chars[i..i + MARKER].to_vec();

            window.sort();
            window.dedup();

            if window.len() == MARKER {
                res = i + MARKER;
                break;
            }
        }

        res.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn part1() {
        let input = utils::get_sample_input(6);
        let data = Data::parse(&input);
        assert_eq!(data.part1(), "11");
    }

    #[test]
    fn part2() {
        let input = utils::get_sample_input(6);
        let data = Data::parse(&input);
        assert_eq!(data.part2(), "26");
    }
}
