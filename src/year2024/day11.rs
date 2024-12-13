use std::collections::HashMap;

use crate::solution::{Day, Solutions};

pub struct Day11;

type Stones = HashMap<String, u64>;

fn add_stones(stones: &mut Stones, name: String, count: u64) {
    let curr_count = stones.get(&name).unwrap_or(&0);
    stones.insert(name, curr_count + count);
}

fn blink(stones: Stones) -> Stones {
    let mut new_stones: Stones = HashMap::new();
    for (stone, count) in stones {
        if stone == "0" {
            add_stones(&mut new_stones, "1".to_string(), count);
        } else if stone.len() % 2 == 0 {
            let key_1 = stone[..stone.len() / 2].parse::<u64>().unwrap().to_string();
            add_stones(&mut new_stones, key_1, count);

            let key_2 = stone[stone.len() / 2..].parse::<u64>().unwrap().to_string();
            add_stones(&mut new_stones, key_2, count);
        } else {
            let key = (stone.parse::<u64>().unwrap() * 2024).to_string();
            add_stones(&mut new_stones, key, count);
        }
    }
    return new_stones;
}

fn blink_n_times(start_stones: Stones, n: u8) -> u64 {
    let mut stones = start_stones;
    for _ in 0..n {
        stones = blink(stones);
    }
    return stones.values().sum();
}

impl Day for Day11 {
    type Context = Stones;
    type Part1 = u64;
    type Part2 = u64;

    fn title() -> String {
        String::from("Plutonian Pebbles")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(55312),
            part1: Some(191690),
            part2_example: Some(65601038650482),
            part2: Some(228651922369703),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        let mut stones: Stones = HashMap::new();
        for stone in input.split(' ') {
            stones.insert(stone.parse().unwrap(), 1);
        }
        return stones;
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        blink_n_times(context.clone(), 25)
    }

    fn solve_part2(context: &Self::Context) -> Self::Part2 {
        blink_n_times(context.clone(), 75)
    }
}
