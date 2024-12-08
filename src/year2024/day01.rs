use itertools::Itertools;
use std::collections::HashMap;

use crate::solution::{Day, Solutions};

pub struct Day01;

pub struct Day01Context {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Day for Day01 {
    type Context = Day01Context;
    type Part1 = i32;
    type Part2 = i32;

    fn title() -> String {
        String::from("Historian Hysteria")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(11),
            part1: Some(2192892),
            part2_example: Some(31),
            part2: Some(22962826),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        let (left, right): (Vec<i32>, Vec<i32>) = input
            .lines()
            .map(|line| {
                let parts: Vec<i32> = line
                    .split_whitespace()
                    .map(|part| part.parse().expect("Expected a number"))
                    .collect();
                (parts[0], parts[1])
            })
            .unzip();
        Day01Context { left, right }
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        let mut score = 0;
        for (l, r) in context
            .left
            .iter()
            .sorted()
            .zip(context.right.iter().sorted())
        {
            score += (l - r).abs();
        }
        return score;
    }

    fn solve_part2(context: &Self::Context) -> Self::Part2 {
        let mut occurrences = HashMap::new();
        for &num in &context.right {
            *occurrences.entry(num).or_insert(0) += 1;
        }

        let mut score = 0;

        for (l, _) in context
            .left
            .iter()
            .sorted()
            .zip(context.right.iter().sorted())
        {
            score += l * (occurrences.get(&l).unwrap_or(&0))
        }

        return score;
    }
}
