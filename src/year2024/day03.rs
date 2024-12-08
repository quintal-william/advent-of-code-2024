use regex::Regex;
use std::vec;

use crate::solution::{Day, Solutions};

pub struct Day03;

pub enum Command {
    Do,
    Dont,
    Mul(i32, i32),
}

fn command_from_caps(caps: regex::Captures) -> Command {
    if caps.get(1).is_some() {
        Command::Do
    } else if caps.get(2).is_some() {
        Command::Dont
    } else {
        let a = caps.get(4).unwrap().as_str().parse().unwrap();
        let b = caps.get(5).unwrap().as_str().parse().unwrap();
        Command::Mul(a, b)
    }
}

impl Day for Day03 {
    type Context = Vec<Command>;
    type Part1 = i32;
    type Part2 = i32;

    fn title() -> String {
        String::from("Mull It Over")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(161),
            part1: Some(170778545),
            part2_example: Some(48),
            part2: Some(82868252),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        let re = Regex::new(r"(do)\(\)|(don't)\(\)|(mul)\((\d{1,3}),(\d{1,3})\)").unwrap();
        let mut commands = vec![];
        for caps in re.captures_iter(input) {
            commands.push(command_from_caps(caps));
        }
        return commands;
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        let mut part1 = 0;
        for command in context {
            if let Command::Mul(a, b) = command {
                part1 += a * b;
            }
        }
        return part1;
    }

    fn solve_part2(context: &Self::Context) -> Self::Part2 {
        let mut part2 = 0;
        let mut is_enabled = true;
        for command in context {
            match command {
                Command::Do => is_enabled = true,
                Command::Dont => is_enabled = false,
                Command::Mul(a, b) => {
                    if is_enabled {
                        part2 += a * b;
                    }
                }
            }
        }
        return part2;
    }
}
