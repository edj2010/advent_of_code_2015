use advent_of_code::parse::{parsers, Parser};

use std::cmp::min;

fn parse(input: &str) -> impl Iterator<Item = (u32, u32, u32)> {
    parsers::number()
        .skip_tag("x")
        .and_then(parsers::number())
        .skip_tag("x")
        .and_then(parsers::number())
        .map(|((a, b), c)| (a, b, c))
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    parse(input)
        .map(|(a, b, c)| {
            let faces = vec![a * b, b * c, a * c];
            let extra = min(min(faces[0], faces[1]), faces[2]);
            faces.into_iter().sum::<u32>() * 2 + extra
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    parse(input)
        .map(|(a, b, c)| {
            let ribbon = min(min(a + b, a + c), b + c) * 2;
            a * b * c + ribbon
        })
        .sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "2x3x4
1x1x10
";
    const DAY: Day = Day::Day02;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 58 + 43);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 34 + 14);
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY
            )),
            1586300
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY
            )),
            3737498
        );
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(|| {
            part1(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY,
            ))
        });
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(|| {
            part2(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY,
            ))
        });
    }
}
