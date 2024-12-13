use crate::{
    cli::InputType,
    day::{DayOutput, DayValue, PartOutput},
};

pub type YearValue = u16;

pub struct FullDayOutput {
    pub part1: PartOutput,
    pub part1_example: PartOutput,
    pub part2: PartOutput,
    pub part2_example: PartOutput,
}

pub trait Year {
    fn solve_day(year: YearValue, day: DayValue, input_type: InputType) -> Option<DayOutput>;

    fn solve_all(year: YearValue) -> impl Iterator<Item = Option<FullDayOutput>> {
        (1..=25).map(move |day| {
            let example = Self::solve_day(year, day, InputType::Example);
            let puzzle = Self::solve_day(year, day, InputType::Puzzle);
            return match (example, puzzle) {
                (Some(a), Some(b)) => Some(FullDayOutput {
                    part1: a.part1,
                    part1_example: b.part1,
                    part2: a.part2,
                    part2_example: b.part2,
                }),
                _ => None,
            };
        })
    }
}
