use crate::solution::{Day, Solutions};

pub struct Day09;

impl Day for Day09 {
    type Context = Vec<u8>;
    type Part1 = u64;
    type Part2 = u64;

    fn title() -> String {
        String::from("Disk Fragmenter")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(1928),
            part1: None,
            part2_example: None,
            part2: None,
        }
    }

    fn create_context(input: &str) -> Self::Context {
        input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        let mut memory = vec![];
        for (id, block) in context.iter().enumerate() {
            for _ in 0..*block {
                if id % 2 == 1 {
                    memory.push(None);
                } else {
                    memory.push(Some(id / 2))
                }
            }
        }

        let mut i = 0;
        let mut j = memory.len();
        while i != j && i < memory.len() {
            if memory[i].is_none() {
                while i != j {
                    j -= 1;
                    if let Some(Some(entry)) = memory.pop() {
                        memory[i] = Some(entry);
                        break;
                    }
                }
            }
            i += 1;
        }

        let mut score = 0u64;
        for (id, entry) in memory.iter().enumerate() {
            if let Some(e) = *entry {
                score += (id * e) as u64;
            }
        }

        return score;
    }

    fn solve_part2(_context: &Self::Context) -> Self::Part2 {
        0
    }
}
