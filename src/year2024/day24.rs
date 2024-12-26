use crate::day::{Day, Solutions};

pub struct Day24;

impl Day for Day24 {
    type Context = ();
    type Part1 = i64;
    type Part2 = String;

    fn title() -> String {
        String::from("Crossed Wires")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(2024),
            part1: Some(61495910098126),
            part2_example: Some(String::from("aaa,aoc,bbb,ccc,eee,ooo,z24,z99")),
            part2: Some(String::from("css,cwt,gdd,jmv,pqt,z05,z09,z37")),
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
