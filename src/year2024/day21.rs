use crate::day::{Day, Solutions};

pub struct Day21;

impl Day for Day21 {
    type Context = ();
    type Part1 = i32;
    type Part2 = i64;

    fn title() -> String {
        String::from("Keypad Conundrum")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(126384),
            part1: Some(107934),
            part2_example: None,
            part2: Some(130470079151124),
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
