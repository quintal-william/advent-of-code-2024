use crate::{
    cli::DayValue,
    solution::{Day, Year},
};

pub mod day01;
// pub mod day02;
// pub mod day03;

pub struct Year2024;

impl Year for Year2024 {
    fn get_day(&self, day: DayValue) -> Option<impl Day> {
        match day {
            1 => Some(day01::Day01),
            _ => None,
        }
    }
}
