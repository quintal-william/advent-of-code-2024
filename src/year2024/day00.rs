use crate::solution::{Day, Solutions};

pub struct Day00;

impl Day for Day00 {
    type Context = ();
    type Part1 = i32;
    type Part2 = i32;

    fn title() -> String {
        String::from("Title")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: None,
            part1: None,
            part2_example: None,
            part2: None,
        }
    }

    fn create_context(input: &str) -> Self::Context {
        ()
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        0
    }

    fn solve_part2(context: &Self::Context) -> Self::Part2 {
        0
    }
}
