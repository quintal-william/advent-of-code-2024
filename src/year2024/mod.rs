use crate::{
    cli::{DayValue, YearValue},
    solution::{Day, SolvedDay, Year},
};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day08;

pub struct Year2024;

impl Year for Year2024 {
    fn solve_day(year: YearValue, day: DayValue) -> Option<SolvedDay> {
        match day {
            1 => Some(day01::Day01::solve(year, day)),
            2 => Some(day02::Day02::solve(year, day)),
            3 => Some(day03::Day03::solve(year, day)),
            4 => Some(day04::Day04::solve(year, day)),
            5 => Some(day05::Day05::solve(year, day)),
            8 => Some(day08::Day08::solve(year, day)),
            _ => None,
        }
    }
}
