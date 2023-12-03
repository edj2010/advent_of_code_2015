use advent_of_code::{
    grid::{GridPoint, GridPointDelta, Lattice, EAST, NORTH, SOUTH, WEST},
    parse::{parsers, Parser},
};

fn parse(input: &str) -> impl Iterator<Item = GridPointDelta<isize>> {
    parsers::tag_replace("<", WEST)
        .or(parsers::tag_replace("^", NORTH))
        .or(parsers::tag_replace(">", EAST))
        .or(parsers::tag_replace("v", SOUTH))
        .many()
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut houses: Lattice<u32> = Lattice::empty();
    let mut current_location: GridPoint<isize> = GridPoint::new(0, 0);
    houses.set(current_location, 1);
    for next_move in parse(input) {
        current_location = (current_location + next_move).unwrap();
        *(houses.entry(current_location).or_default()) += 1;
    }
    houses.into_iter().filter(|(_, count)| *count > 0).count()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut houses: Lattice<u32> = Lattice::empty();
    let mut a_location: GridPoint<isize> = GridPoint::new(0, 0);
    let mut b_location: GridPoint<isize> = GridPoint::new(0, 0);
    houses.set(a_location, 2);
    for (idx, next_move) in parse(input).enumerate() {
        let current_location = if idx % 2 == 0 {
            &mut a_location
        } else {
            &mut b_location
        };

        *current_location = (*current_location + next_move).unwrap();
        *(houses.entry(*current_location).or_default()) += 1;
    }
    houses.into_iter().filter(|(_, count)| *count > 0).count()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "";
    const DAY: Day = Day::Day03;

    #[test]
    fn part1_example() {
        assert_eq!(part1(">"), 2);
        assert_eq!(part1("^>v<"), 4);
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("^v"), 3);
        assert_eq!(part2("^>v<"), 3);
        assert_eq!(part2("^v^v^v^v^v"), 11);
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
            2081
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
            2341
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
