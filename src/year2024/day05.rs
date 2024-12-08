use crate::solution::{Day, Solutions};

pub struct Day05;

impl Day for Day05 {
    type Context = ();
    type Part1 = i32;
    type Part2 = i32;

    fn title() -> String {
        String::from("Print Queue")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(143),
            part1: Some(4185),
            part2_example: Some(123),
            part2: Some(4480),
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
