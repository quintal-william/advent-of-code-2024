use crate::solution::{Day, Solutions};

pub struct Day07;

impl Day for Day07 {
    type Context = ();
    type Part1 = i64;
    type Part2 = i64;

    fn title() -> String {
        String::from("Bridge Repair")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(3749),
            part1: Some(465126289353),
            part2_example: Some(11387),
            part2: Some(70597497486371),
        }
    }

    fn create_context(_input: &str) -> Self::Context {
        ()
    }

    fn solve_part1(_context: &Self::Context) -> Self::Part1 {
        0
    }

    fn solve_part2(_context: &Self::Context) -> Self::Part2 {
        0
    }
}
