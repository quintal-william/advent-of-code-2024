use itertools::Itertools;
use std::collections::HashMap;

use crate::solution::Day;

pub struct Day01Context {
    left: Vec<i32>,
    right: Vec<i32>,
}

pub struct Day01;

impl Day for Day01 {
    type Context = Day01Context;
    type Part1 = i32;
    type Part2 = i32;

    fn title(&self) -> &str {
        "Historian Hysteria"
    }

    fn create_context(&self, input: &str) -> Self::Context {
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
        return Day01Context { left, right };
    }

    fn solve_part_1(&self, context: &Self::Context) -> Self::Part1 {
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

    fn solve_part_2(&self, context: &Self::Context) -> Self::Part2 {
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
