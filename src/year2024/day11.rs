use crate::solution::{Day, Solutions};

pub struct Day11;

impl Day for Day11 {
    type Context = ();
    type Part1 = u64;
    type Part2 = u64;

    fn title() -> String {
        String::from("Plutonian Pebbles")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(55312),
            part1: Some(191690),
            part2_example: Some(65601038650482),
            part2: Some(228651922369703),
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
