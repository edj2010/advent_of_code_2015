use advent_of_code::{
    grid::{GridDimensions, GridPoint, Lattice},
    parse::{parsers, Parser},
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Action {
    On,
    Off,
    Toggle,
}

fn parse(input: &str) -> impl Iterator<Item = (Action, (GridPoint<isize>, GridPoint<isize>))> {
    parsers::tag_replace("turn on ", Action::On)
        .or(parsers::tag_replace("turn off ", Action::Off))
        .or(parsers::tag_replace("toggle ", Action::Toggle))
        .and_then(
            parsers::number()
                .skip_tag(",")
                .and_then(parsers::number())
                .map(|(a, b)| GridPoint::new(a as isize, b as isize))
                .skip_tag(" through ")
                .and_then(
                    parsers::number()
                        .skip_tag(",")
                        .and_then(parsers::number())
                        .map(|(a, b)| GridPoint::new(a as isize, b as isize)),
                ),
        )
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut lattice: Lattice<bool> = Lattice::empty();
    for (action, (a, b)) in parse(input) {
        GridDimensions::of_points_inclusive(a, b)
            .all_contained_points()
            .for_each(|p| match action {
                Action::Off => {
                    lattice.set(p, false);
                }
                Action::On => {
                    lattice.set(p, true);
                }
                Action::Toggle => *lattice.entry(p).or_default() ^= true,
            });
    }
    lattice.into_iter().filter(|(_, v)| *v).count()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let mut lattice: Lattice<u32> = Lattice::empty();
    for (action, (a, b)) in parse(input) {
        GridDimensions::of_points_inclusive(a, b)
            .all_contained_points()
            .for_each(|p| match action {
                Action::Off => {
                    let v = lattice.entry(p).or_default();
                    *v = v.saturating_sub(1);
                }
                Action::On => {
                    *(lattice.entry(p).or_default()) += 1;
                }
                Action::Toggle => {
                    *(lattice.entry(p).or_default()) += 2;
                }
            });
    }
    lattice.into_iter().map(|(_, v)| v).sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500
";
    const DAY: Day = Day::Day06;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 998996);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 1001996);
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
            400410
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
            15343601
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
