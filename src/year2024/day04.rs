use crate::solution::{Day, Solutions};

pub struct Day04;

pub struct Day04Context {
    grid: Vec<Vec<char>>,
    size: i32,
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
            size: lines.len() as i32,
        }
    }

    fn solve_part_1(context: &Self::Context) -> Self::Part1 {
        let mut score = 0;

        for x in 0..context.size {
            for y in 0..context.size {
                let mut words: Vec<String> = vec![];

                if x + 3 < context.size {
                    words.push(format!(
                        "{}{}{}{}",
                        context.grid[y as usize][x as usize],
                        context.grid[y as usize][(x + 1) as usize],
                        context.grid[y as usize][(x + 2) as usize],
                        context.grid[y as usize][(x + 3) as usize]
                    ));
                }
                if y + 3 < context.size && x + 3 < context.size {
                    words.push(format!(
                        "{}{}{}{}",
                        context.grid[y as usize][x as usize],
                        context.grid[(y + 1) as usize][(x + 1) as usize],
                        context.grid[(y + 2) as usize][(x + 2) as usize],
                        context.grid[(y + 3) as usize][(x + 3) as usize]
                    ));
                }
                if y + 3 < context.size {
                    words.push(format!(
                        "{}{}{}{}",
                        context.grid[y as usize][x as usize],
                        context.grid[(y + 1) as usize][x as usize],
                        context.grid[(y + 2) as usize][x as usize],
                        context.grid[(y + 3) as usize][x as usize]
                    ));
                }
                if y + 3 < context.size && x - 3 >= 0 {
                    words.push(format!(
                        "{}{}{}{}",
                        context.grid[y as usize][x as usize],
                        context.grid[(y + 1) as usize][(x - 1) as usize],
                        context.grid[(y + 2) as usize][(x - 2) as usize],
                        context.grid[(y + 3) as usize][(x - 3) as usize]
                    ));
                }
                if x - 3 >= 0 {
                    words.push(format!(
                        "{}{}{}{}",
                        context.grid[y as usize][x as usize],
                        context.grid[y as usize][(x - 1) as usize],
                        context.grid[y as usize][(x - 2) as usize],
                        context.grid[y as usize][(x - 3) as usize]
                    ));
                }
                if y - 3 >= 0 && x - 3 >= 0 {
                    words.push(format!(
                        "{}{}{}{}",
                        context.grid[y as usize][x as usize],
                        context.grid[(y - 1) as usize][(x - 1) as usize],
                        context.grid[(y - 2) as usize][(x - 2) as usize],
                        context.grid[(y - 3) as usize][(x - 3) as usize]
                    ));
                }
                if y - 3 >= 0 {
                    words.push(format!(
                        "{}{}{}{}",
                        context.grid[y as usize][x as usize],
                        context.grid[(y - 1) as usize][x as usize],
                        context.grid[(y - 2) as usize][x as usize],
                        context.grid[(y - 3) as usize][x as usize]
                    ));
                }
                if y - 3 >= 0 && x + 3 < context.size {
                    words.push(format!(
                        "{}{}{}{}",
                        context.grid[y as usize][x as usize],
                        context.grid[(y - 1) as usize][(x + 1) as usize],
                        context.grid[(y - 2) as usize][(x + 2) as usize],
                        context.grid[(y - 3) as usize][(x + 3) as usize]
                    ));
                }

                score += words.iter().filter(|word| *word == "XMAS").count() as i32;
            }
        }
        return score;
    }

    fn solve_part_2(context: &Self::Context) -> Self::Part2 {
        let mut score = 0;
        for x in 0..context.size {
            for y in 0..context.size {
                if x + 2 >= context.size || y + 2 >= context.size {
                    continue;
                }

                let word = format!(
                    "{}{}{}{}{}",
                    context.grid[y as usize][x as usize],
                    context.grid[y as usize][(x + 2) as usize],
                    context.grid[(y + 1) as usize][(x + 1) as usize],
                    context.grid[(y + 2) as usize][x as usize],
                    context.grid[(y + 2) as usize][(x + 2) as usize]
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
