use crate::cli::{DayValue, YearValue};
use std::{fmt::Display, fs};

pub struct Solutions<Part1, Part2> {
    pub part1_example: Option<Part1>,
    pub part1: Option<Part1>,
    pub part2_example: Option<Part2>,
    pub part2: Option<Part2>,
}

pub struct PartOutput {
    pub value: String,
    pub is_correct: Option<bool>,
}

pub struct DayOutput {
    pub title: String,
    pub part1: PartOutput,
    pub part2: PartOutput,
}

pub struct FullDayOutput {
    pub part1: PartOutput,
    pub part1_example: PartOutput,
    pub part2: PartOutput,
    pub part2_example: PartOutput,
}

#[derive(PartialEq)]
pub enum InputType {
    Example,
    Puzzle,
}

pub trait Day {
    type Context;
    type Part1: Display;
    type Part2: Display;

    fn title() -> String;
    fn solutions() -> Solutions<Self::Part1, Self::Part2>;
    fn create_context(input: &str) -> Self::Context;
    fn solve_part1(context: &Self::Context) -> Self::Part1;
    fn solve_part2(context: &Self::Context) -> Self::Part2;

    fn get_input(year: YearValue, day: DayValue, input_type: &InputType) -> String {
        let ext = match input_type {
            InputType::Example => "example",
            InputType::Puzzle => "in",
        };
        let path = format!("src/year{year:0>4}/day{day:0>2}.{ext}");
        let input = fs::read_to_string(&path).expect(&format!("Expected input file at {path}"));
        return input;
    }

    fn create_part_output<T: Display>(value: &T, expected: Option<&T>) -> PartOutput {
        PartOutput {
            value: value.to_string(),
            is_correct: expected.map(|expected| value.to_string() == expected.to_string()),
        }
    }

    fn solve(year: YearValue, day: DayValue, input_type: InputType) -> DayOutput {
        let ref input = Self::get_input(year, day, &input_type);
        let ref context = Self::create_context(input);
        let solutions = Self::solutions();

        let part1 = Self::solve_part1(context);
        let part1_output = if input_type == InputType::Example {
            Self::create_part_output(&part1, solutions.part1_example.as_ref())
        } else {
            Self::create_part_output(&part1, solutions.part1.as_ref())
        };

        let part2 = Self::solve_part2(context);
        let part2_output = if input_type == InputType::Example {
            Self::create_part_output(&part2, solutions.part2_example.as_ref())
        } else {
            Self::create_part_output(&part2, solutions.part2.as_ref())
        };

        DayOutput {
            title: Self::title(),
            part1: part1_output,
            part2: part2_output,
        }
    }
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
