// pub trait Solution<T> {
//     fn title(&self) -> &str;
//     fn parse(input: &str) -> T;

//     fn solve(&self) {
//         println!("Title: {}", self.title());
//         println!("Part 1: {}", self.solve_part_1());
//         println!("Part 2: {}", self.solve_part_2());
//     }
// }

use crate::cli::DayValue;
use std::fmt::Display;

pub trait Day {
    type Context: Sized;
    type Part1: Display;
    type Part2: Display;

    fn title(&self) -> &str;
    fn create_context(&self, input: &str) -> Self::Context;
    fn solve_part_1(&self, context: &Self::Context) -> Self::Part1;
    fn solve_part_2(&self, context: &Self::Context) -> Self::Part2;
}

pub trait Year {
    fn get_day(&self, day: DayValue) -> Option<impl Day>;
}
