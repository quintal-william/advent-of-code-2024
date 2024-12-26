use crate::day::{Day, Solutions};

pub struct Day25;

#[derive(Debug)]
pub struct Day25Context {
    locks: Vec<[i64; 5]>,
    keys: Vec<[i64; 5]>,
}

enum PatternType {
    LOCK,
    KEY,
}

impl PatternType {
    fn from_bool(is_key: bool) -> Self {
        if is_key {
            return PatternType::KEY;
        }
        return PatternType::LOCK;
    }
}

impl Day for Day25 {
    type Context = Day25Context;
    type Part1 = i32;
    type Part2 = i32;

    fn title() -> String {
        String::from("Code Chronicle")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(3),
            part1: Some(3146),
            part2_example: Some(0),
            part2: Some(0),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        let mut ctx = Day25Context {
            locks: vec![],
            keys: vec![],
        };
        for pattern in input.split("\n\n") {
            let pattern_type = PatternType::from_bool(pattern.starts_with('.'));
            let mut heights: [i64; 5] = [-1; 5];
            for (i, line) in pattern.split('\n').enumerate() {
                for j in 0..5 {
                    let line_chars: Vec<char> = line.chars().collect();
                    match pattern_type {
                        PatternType::KEY => {
                            if heights[j] == -1 && line_chars[j] == '#' {
                                heights[j] = 5 - (i as i64 - 1);
                            }
                        }
                        PatternType::LOCK => {
                            if heights[j] == -1 && line_chars[j] == '.' {
                                heights[j] = i as i64 - 1;
                            }
                        }
                    }
                }
            }
            match pattern_type {
                PatternType::KEY => {
                    ctx.keys.push(heights);
                }
                PatternType::LOCK => {
                    ctx.locks.push(heights);
                }
            }
        }
        return ctx;
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        let mut overlapping = 0;
        for lock in &context.locks {
            for key in &context.keys {
                let mut is_overlapping = false;
                for i in 0..5 {
                    if lock[i] + key[i] > 5 {
                        is_overlapping = true;
                        break;
                    }
                }
                if !is_overlapping {
                    overlapping += 1;
                }
            }
        }
        return overlapping;
    }

    fn solve_part2(_context: &Self::Context) -> Self::Part2 {
        0
    }
}
