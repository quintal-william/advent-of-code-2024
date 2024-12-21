use crate::day::{Day, Solutions};

pub struct Day19;

impl Day for Day19 {
    type Context = ();
    type Part1 = i32;
    type Part2 = i64;

    fn title() -> String {
        String::from("Linen Layout")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(6),
            part1: Some(272),
            part2_example: Some(16),
            part2: Some(1041529704688380),
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
