use crate::day::{Day, Solutions};

pub struct Day20;

impl Day for Day20 {
    type Context = ();
    type Part1 = i32;
    type Part2 = i32;

    fn title() -> String {
        String::from("Race Condition")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: None,
            part1: Some(1524),
            part2_example: None,
            part2: Some(1033746),
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
