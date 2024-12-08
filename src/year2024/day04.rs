use crate::solution::{Day, Solutions};

pub struct Day04;

pub struct Day04Context {
    grid: Vec<Vec<char>>,
    size: usize,
}

impl Day for Day04 {
    type Context = Day04Context;
    type Part1 = i32;
    type Part2 = i32;

    fn title() -> String {
        String::from("Ceres Search")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(18),
            part1: Some(2599),
            part2_example: Some(9),
            part2: Some(1948),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        let lines: Vec<&str> = input.lines().collect();
        Day04Context {
            grid: lines.iter().map(|line| line.chars().collect()).collect(),
            size: lines.len(),
        }
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        let mut score = 0;

        for x in 0..context.size {
            for y in 0..context.size {
                let mut words: Vec<String> = vec![];

                if x < context.size - 3 {
                    words.push(String::from_iter(vec![
                        context.grid[y][x],
                        context.grid[y][x + 1],
                        context.grid[y][x + 2],
                        context.grid[y][x + 3],
                    ]));
                }
                if y < context.size - 3 && x < context.size - 3 {
                    words.push(String::from_iter(vec![
                        context.grid[y][x],
                        context.grid[y + 1][x + 1],
                        context.grid[y + 2][x + 2],
                        context.grid[y + 3][x + 3],
                    ]));
                }
                if y < context.size - 3 {
                    words.push(String::from_iter(vec![
                        context.grid[y][x],
                        context.grid[y + 1][x],
                        context.grid[y + 2][x],
                        context.grid[y + 3][x],
                    ]));
                }
                if y < context.size - 3 && x >= 3 {
                    words.push(String::from_iter(vec![
                        context.grid[y][x],
                        context.grid[y + 1][x - 1],
                        context.grid[y + 2][x - 2],
                        context.grid[y + 3][x - 3],
                    ]));
                }
                if x >= 3 {
                    words.push(String::from_iter(vec![
                        context.grid[y][x],
                        context.grid[y][x - 1],
                        context.grid[y][x - 2],
                        context.grid[y][x - 3],
                    ]));
                }
                if y >= 3 && x >= 3 {
                    words.push(String::from_iter(vec![
                        context.grid[y][x],
                        context.grid[y - 1][x - 1],
                        context.grid[y - 2][x - 2],
                        context.grid[y - 3][x - 3],
                    ]));
                }
                if y >= 3 {
                    words.push(String::from_iter(vec![
                        context.grid[y][x],
                        context.grid[y - 1][x],
                        context.grid[y - 2][x],
                        context.grid[y - 3][x],
                    ]));
                }
                if y >= 3 && x < context.size - 3 {
                    words.push(String::from_iter(vec![
                        context.grid[y][x],
                        context.grid[y - 1][x + 1],
                        context.grid[y - 2][x + 2],
                        context.grid[y - 3][x + 3],
                    ]));
                }

                score += words.iter().filter(|word| *word == "XMAS").count() as i32;
            }
        }
        return score;
    }

    fn solve_part2(context: &Self::Context) -> Self::Part2 {
        let mut score = 0;
        for x in 0..context.size {
            for y in 0..context.size {
                if x + 2 >= context.size || y + 2 >= context.size {
                    continue;
                }

                let word = format!(
                    "{}{}{}{}{}",
                    context.grid[y][x],
                    context.grid[y][x + 2],
                    context.grid[y + 1][x + 1],
                    context.grid[y + 2][x],
                    context.grid[y + 2][x + 2]
                );
                score += if word == "MSAMS" || word == "SSAMM" || word == "MMASS" || word == "SMASM"
                {
                    1
                } else {
                    0
                };
            }
        }
        return score;
    }
}
