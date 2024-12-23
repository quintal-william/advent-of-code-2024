use std::collections::HashMap;

use itertools::Itertools;

use crate::day::{Day, Solutions};

pub struct Day19;

type NumConfigs = i64;
type NumConfigsMap<'a> = HashMap<&'a str, NumConfigs>;

fn get_num_configs<'a>(design: &'a str, towels: &Vec<&'a str>, num_configs_map: &mut NumConfigsMap<'a>) -> NumConfigs {
    if let Some(num_configs) = num_configs_map.get(design) {
        return *num_configs;
    }

    let mut num_configs = 0;
    for towel in towels {
        if towel == &design {
            num_configs += 1;
        }
        if design.starts_with(towel) {
            num_configs += get_num_configs(&design[towel.len()..], towels, num_configs_map)
        }
    }

    num_configs_map.insert(design, num_configs);
    return num_configs;
}

impl Day for Day19 {
    type Context = Vec<NumConfigs>;
    type Part1 = usize;
    type Part2 = i64;

    fn title() -> String {
        String::from("Linen Layout")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(6),
            part1: Some(272),
            part2_example: Some(16),
            part2: Some(1041529704688380),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        let content: Vec<&str> = input.split("\n\n").collect();
        let towels: Vec<&str> = content[0].split(", ").sorted_by_key(|s| -(s.len() as isize)).collect();
        let ref mut num_configs_map: NumConfigsMap = HashMap::new();
        content[1].split('\n').map(|design| get_num_configs(design, &towels, num_configs_map)).collect()
    }

    fn solve_part1(num_configs: &Self::Context) -> Self::Part1 {
        num_configs.iter().filter(|v| **v > 0).collect::<Vec<&NumConfigs>>().len()
    }

    fn solve_part2(num_configs: &Self::Context) -> Self::Part2 {
        num_configs.iter().map(|&x| x as i64).sum()
    }
}
