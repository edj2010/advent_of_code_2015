use advent_of_code::parse::{parsers, Parser};
use std::collections::HashMap;

fn parse(input: &str) -> impl Iterator<Item = String> {
    parsers::many_chars(|c| c != '\n')
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

fn nice(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    (chars
        .iter()
        .filter(|&&c| "aeiou".find(|d| d == c).is_some())
        .count()
        >= 3)
        && (0..(s.len() - 1)).any(|idx| chars[idx] == chars[idx + 1])
        && ["ab", "cd", "pq", "xy"]
            .iter()
            .all(|substring| !s.contains(substring))
}

fn nice2(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut seen_doubles: HashMap<(char, char), Vec<usize>> = HashMap::new();
    (0..(s.len() - 1))
        .map(|idx| (idx, (chars[idx], chars[idx + 1])))
        .for_each(|(idx, substring)| seen_doubles.entry(substring).or_default().push(idx));
    (0..(s.len() - 2)).any(|idx| chars[idx] == chars[idx + 2])
        && seen_doubles
            .values()
            .any(|v| (v.len() == 2 && (v[1] - v[0]) > 1) || v.len() > 2)
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    parse(input).filter(|s| nice(s)).count()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    parse(input).filter(|s| nice2(s)).count()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const DAY: Day = Day::Day05;

    #[test]
    fn part1_example() {
        assert!(nice("ugknbfddgicrmopn"));
        assert!(nice("aaa"));
        assert!(!nice("jchzalrnumimnmhp"));
        assert!(!nice("haegwjzuvuyypxyu"));
        assert!(!nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part2_example() {
        assert!(nice2("qjhvhtzxzqqjkmpb"));
        assert!(nice2("xxyxx"));
        assert!(!nice2("aaaba"));
        assert!(nice2("aaaaba"));
        assert!(nice2("aaabaa"));
        assert!(!nice2("uurcxstgmygtbstg"));
        assert!(!nice2("ieodomkazucvgmuy"));
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
            255
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
            55
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
