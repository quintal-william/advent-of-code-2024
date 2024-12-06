use crate::cli::{DayValue, YearValue};
use std::{fmt::Display, fs};

pub struct Solutions<Part1, Part2> {
    pub part1_example: Option<Part1>,
    pub part1: Option<Part1>,
    pub part2_example: Option<Part2>,
    pub part2: Option<Part2>,
}

// TODO naming
pub struct Solution {
    pub value: String,
    pub is_correct: Option<bool>,
}

// TODO naming
pub struct InputTypeSolutions {
    part1: Solution,
    part2: Solution,
}

// TODO naming
pub struct SolvedDay {
    pub title: String,
    pub part1_example: Solution,
    pub part1: Solution,
    pub part2_example: Solution,
    pub part2: Solution,
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
    fn solve_part_1(context: &Self::Context) -> Self::Part1;
    fn solve_part_2(context: &Self::Context) -> Self::Part2;

    fn get_input(year: YearValue, day: DayValue, input_type: &InputType) -> String {
        let ext = match input_type {
            InputType::Example => "example",
            InputType::Puzzle => "in",
        };
        let path = format!("src/year{year:0>4}/day{day:0>2}.{ext}");
        let input = fs::read_to_string(&path).expect(&format!("Expected input file at {path}"));
        return input;
    }

    fn solve_input_type(
        year: YearValue,
        day: DayValue,
        input_type: InputType,
    ) -> InputTypeSolutions {
        let ref input = Self::get_input(year, day, &input_type);
        let ref context = Self::create_context(input);
        let solutions = Self::solutions();
        // TODO is it part_1 or part1?
        let part_1 = Self::solve_part_1(context);
        let part_2 = Self::solve_part_2(context);

        InputTypeSolutions {
            part1: Solution {
                value: part_1.to_string(),
                is_correct: if input_type == InputType::Example {
                    solutions
                        .part1_example
                        .map(|expected| part_1.to_string() == expected.to_string())
                } else {
                    solutions
                        .part1
                        .map(|expected| part_1.to_string() == expected.to_string())
                },
            },
            part2: Solution {
                value: part_2.to_string(),
                is_correct: if input_type == InputType::Example {
                    solutions
                        .part2_example
                        .map(|expected| part_2.to_string() == expected.to_string())
                } else {
                    solutions
                        .part2
                        .map(|expected| part_2.to_string() == expected.to_string())
                },
            },
        }
    }

    fn solve(year: YearValue, day: DayValue) -> SolvedDay {
        let example_solutions = Self::solve_input_type(year, day, InputType::Example);
        let puzzle_solutions = Self::solve_input_type(year, day, InputType::Puzzle);

        SolvedDay {
            title: Self::title(),
            part1_example: example_solutions.part1,
            part1: puzzle_solutions.part1,
            part2_example: example_solutions.part2,
            part2: puzzle_solutions.part2,
        }
    }
}

pub trait Year {
    fn solve_day(year: YearValue, day: DayValue) -> Option<SolvedDay>;
}
