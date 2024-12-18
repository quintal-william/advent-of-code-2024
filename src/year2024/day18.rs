use crate::day::{Day, Solutions};

pub struct Day18;

impl Day for Day18 {
    type Context = ();
    type Part1 = i32;
    type Part2 = String;

    fn title() -> String {
        String::from("RAM Run")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(22),
            part1: Some(284),
            part2_example: Some(String::from("6,1")),
            part2: Some(String::from("51,50")),
        }
    }

    fn create_context(_input: &str) -> Self::Context {
        ()
    }

    fn solve_part1(_context: &Self::Context) -> Self::Part1 {
        0
    }

    fn solve_part2(_context: &Self::Context) -> Self::Part2 {
        String::from("")
    }
}
