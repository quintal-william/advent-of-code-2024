use crate::solution::{Day, Solutions};

pub struct Day12;

impl Day for Day12 {
    type Context = ();
    type Part1 = i32;
    type Part2 = i32;

    fn title() -> String {
        String::from("Garden Groups")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(1930),
            part1: Some(1424006),
            part2_example: Some(1206),
            part2: Some(858684),
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
