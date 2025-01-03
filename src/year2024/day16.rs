use crate::day::{Day, Solutions};

pub struct Day16;

impl Day for Day16 {
    type Context = ();
    type Part1 = i32;
    type Part2 = i32;

    fn title() -> String {
        String::from("Reindeer Maze")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(7036),
            part1: Some(94444),
            part2_example: Some(45),
            part2: Some(502),
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
