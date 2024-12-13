use crate::{
    cli::InputType,
    day::{Day, DayOutput, DayValue},
    year::{Year, YearValue},
};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;

pub struct Year2024;

impl Year for Year2024 {
    fn solve_day(year: YearValue, day: DayValue, input_type: InputType) -> Option<DayOutput> {
        match day {
            1 => Some(day01::Day01::solve(year, day, input_type)),
            2 => Some(day02::Day02::solve(year, day, input_type)),
            3 => Some(day03::Day03::solve(year, day, input_type)),
            4 => Some(day04::Day04::solve(year, day, input_type)),
            5 => Some(day05::Day05::solve(year, day, input_type)),
            6 => Some(day06::Day06::solve(year, day, input_type)),
            7 => Some(day07::Day07::solve(year, day, input_type)),
            8 => Some(day08::Day08::solve(year, day, input_type)),
            9 => Some(day09::Day09::solve(year, day, input_type)),
            10 => Some(day10::Day10::solve(year, day, input_type)),
            11 => Some(day11::Day11::solve(year, day, input_type)),
            12 => Some(day12::Day12::solve(year, day, input_type)),
            13 => Some(day13::Day13::solve(year, day, input_type)),
            14 => Some(day14::Day14::solve(year, day, input_type)),
            _ => None,
        }
    }
}
