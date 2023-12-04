use advent_of_code::{
    hash::md5,
    parse::{parsers, Parser},
};

fn parse(input: &str) -> String {
    parsers::many_chars(|c| c != '\n')
        .skip(parsers::tag("\n").maybe())
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let max = u128::MAX >> 20;
    let base = parse(input);
    (0..)
        .find(|extra| {
            let s = if *extra == 0 {
                base.clone()
            } else {
                base.clone() + &extra.to_string()
            };
            md5::hash_string(&s) < max
        })
        .unwrap()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let max = u128::MAX >> 24;
    let base = parse(input);
    (0..)
        .find(|extra| {
            let s = if *extra == 0 {
                base.clone()
            } else {
                base.clone() + &extra.to_string()
            };
            md5::hash_string(&s) < max
        })
        .unwrap()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "";
    const DAY: Day = Day::Day04;

    #[test]
    fn advent_of_code_2015_q4() {
        assert_eq!(
            md5::hash_string("abcdef609043"),
            0x000001dbbfa3a5c83a2d506429c7b00e
        );

        assert_eq!(
            md5::hash_string("abcdef116"),
            0x3ca681913010ed8d8b40930ca058b60
        );
        assert_eq!(
            md5::hash_string("iwrupvqb343597"),
            0x2a8606324d7476827179caa8989f2638
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
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
            346386
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
            9958218
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
