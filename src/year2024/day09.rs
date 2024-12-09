use crate::solution::{Day, Solutions};

pub struct Day09;

type Id = usize;

#[derive(Debug, Clone)]
pub enum Block {
    Empty(usize),
    Used(Id, usize),
}

fn get_checksum(memory: Vec<Block>) -> u64 {
    let mut score = 0u64;
    let mut id = 0;
    for block in memory {
        match block {
            Block::Used(value, size) => {
                for _ in 0..size {
                    score += (value * id) as u64;
                    id += 1;
                }
            }
            Block::Empty(size) => {
                id += size;
            }
        }
    }
    return score;
}

impl Day for Day09 {
    type Context = Vec<Block>;
    type Part1 = u64;
    type Part2 = u64;

    fn title() -> String {
        String::from("Disk Fragmenter")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(1928),
            part1: Some(6370402949053),
            part2_example: Some(2858),
            part2: Some(6398096697992),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        let parsed: Vec<usize> = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        let mut memory = vec![];
        for (id, block) in parsed.iter().enumerate() {
            let size = block.to_owned();
            if size > 0 {
                if id % 2 == 0 {
                    memory.push(Block::Used(id / 2, size))
                } else {
                    memory.push(Block::Empty(size));
                }
            }
        }
        return memory;
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        let mut memory = context.clone();
        let mut i = 0;
        let mut j = memory.len() - 1;
        while i < j {
            if let Block::Empty(empty_size) = memory[i] {
                let mut size_left = empty_size;
                let mut moved = vec![];
                while i < j {
                    if let Block::Used(value, size) = memory[j] {
                        if size <= size_left {
                            moved.push(Block::Used(value, size));
                            memory[j] = Block::Empty(size);
                            size_left -= size;
                        } else {
                            moved.push(Block::Used(value, size_left));
                            memory[j] = Block::Used(value, size - size_left);
                            break;
                        }
                    }
                    j -= 1;
                }

                let mut new_memory = memory[..i].to_vec();
                new_memory.extend(moved.clone());
                new_memory.extend_from_slice(&memory[i + 1..]);
                memory = new_memory;

                i += moved.len() - 1;
                j += moved.len() - 1;
            }
            i += 1;
        }
        return get_checksum(memory);
    }

    fn solve_part2(context: &Self::Context) -> Self::Part2 {
        let mut memory = context.clone();
        let mut i = memory.len() - 1;
        while i > 0 {
            if let Block::Used(value, block_size) = memory[i] {
                for j in 0..i {
                    if let Block::Empty(empty_size) = memory[j] {
                        if empty_size == block_size {
                            memory[j] = Block::Used(value, block_size);
                            memory[i] = Block::Empty(empty_size);
                            break;
                        } else if empty_size > block_size {
                            memory[i] = Block::Empty(block_size);

                            let mut new_memory = memory[..j].to_vec();
                            new_memory.extend(vec![
                                Block::Used(value, block_size),
                                Block::Empty(empty_size - block_size),
                            ]);
                            new_memory.extend_from_slice(&memory[j + 1..]);
                            memory = new_memory;

                            i += 1;
                            break;
                        }
                    }
                }
            }
            i -= 1;
        }

        return get_checksum(memory);
    }
}
