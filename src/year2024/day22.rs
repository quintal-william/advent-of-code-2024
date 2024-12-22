use crate::day::{Day, Solutions};
use std::{
    collections::{HashMap, HashSet},
    vec,
};

pub struct Day22;

#[derive(Debug)]
struct Monkey {
    prices: Vec<u8>,
    changes: Vec<i8>,
}

#[derive(Debug)]
struct SeqState {
    price: u16,
    monkey_indices: HashSet<usize>,
}

impl Day for Day22 {
    type Context = Vec<Vec<u64>>;
    type Part1 = u64;
    type Part2 = u16;

    fn title() -> String {
        String::from("Monkey Market")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(37990510),
            part1: Some(19854248602),
            part2_example: Some(23),
            part2: Some(2223),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        input
            .split_whitespace()
            .map(|line| {
                let mut secrets = vec![line.parse().unwrap()];
                for i in 0..2000 {
                    let s = secrets[i];
                    let one = ((s * 64) ^ s) % 16777216;
                    let two = ((one / 32) ^ one) % 16777216;
                    let three = ((two * 2048) ^ two) % 16777216;
                    secrets.push(three);
                }
                return secrets;
            })
            .collect()
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        context.iter().map(|s| s[s.len() - 1]).sum()
    }

    fn solve_part2(context: &Self::Context) -> Self::Part2 {
        let monkeys: Vec<Monkey> = context
            .iter()
            .map(|s| {
                let prices: Vec<u8> = s.iter().map(|n| (n % 10) as u8).collect();
                let changes: Vec<i8> = prices
                    .iter()
                    .enumerate()
                    .map(|(i, n)| (*n as i8) - if i == 0 { 0 } else { prices[i - 1] as i8 })
                    .collect();
                Monkey { prices, changes }
            })
            .collect();

        let mut seq_map = HashMap::<String, SeqState>::new();

        for (monkey_index, monkey) in monkeys.iter().enumerate() {
            for i in 4..monkey.prices.len() {
                let seq = monkey.changes[i - 3..=i]
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                let price = monkey.prices[i] as u16;

                if !seq_map.contains_key(&seq) {
                    let mut monkey_indices = HashSet::<usize>::new();
                    monkey_indices.insert(monkey_index);
                    let seq_state = SeqState {
                        price,
                        monkey_indices,
                    };
                    seq_map.insert(seq, seq_state);
                    continue;
                }

                let item = seq_map.get_mut(&seq).unwrap();
                if !item.monkey_indices.contains(&monkey_index) {
                    item.monkey_indices.insert(monkey_index);
                    item.price += price;
                }
            }
        }

        return seq_map
            .values()
            .max_by_key(|state| state.price)
            .unwrap()
            .price;
    }
}
