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

    fn solve(year: YearValue, day: DayValue, input_type: InputType) -> DayOutput {
        let ref input = Self::get_input(year, day, &input_type);
        let ref context = Self::create_context(input);
        let solutions = Self::solutions();
        let part1 = Self::solve_part1(context);
        let part2 = Self::solve_part2(context);

        DayOutput {
            title: Self::title(),
            part1: PartOutput {
                value: part1.to_string(),
                is_correct: if input_type == InputType::Example {
                    solutions
                        .part1_example
                        .map(|expected| part1.to_string() == expected.to_string())
                } else {
                    solutions
                        .part1
                        .map(|expected| part1.to_string() == expected.to_string())
                },
            },
            part2: PartOutput {
                value: part2.to_string(),
                is_correct: if input_type == InputType::Example {
                    solutions
                        .part2_example
                        .map(|expected| part2.to_string() == expected.to_string())
                } else {
                    solutions
                        .part2
                        .map(|expected| part2.to_string() == expected.to_string())
                },
            },
        }
    }
}

pub trait Year {
    fn solve_day(year: YearValue, day: DayValue, input_type: InputType) -> Option<DayOutput>;

    fn solve_all(year: YearValue) -> [Option<(DayOutput, DayOutput)>; 25] {
        let mut solutions_array: [Option<(DayOutput, DayOutput)>; 25] = Default::default();
        for day in 1..=25 {
            let example = Self::solve_day(year, day as DayValue, InputType::Example);
            let puzzle = Self::solve_day(year, day as DayValue, InputType::Puzzle);
            solutions_array[day - 1] = match (example, puzzle) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            };
        }
        return solutions_array;
    }
}
