use crate::day::{Day, Solutions};

pub struct Day17;

impl Day for Day17 {
    type Context = ();
    type Part1 = String;
    type Part2 = i64;

    fn title() -> String {
        String::from("Chronospatial Computer")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: None,
            part1: Some(String::from("1,3,5,1,7,2,5,1,6")),
            part2_example: Some(117440),
            part2: Some(236555997372013),
        }
    }

    fn create_context(_input: &str) -> Self::Context {
        ()
    }

    fn solve_part1(_context: &Self::Context) -> Self::Part1 {
        String::from("")
    }

    fn solve_part2(_context: &Self::Context) -> Self::Part2 {
        0
    }
}
