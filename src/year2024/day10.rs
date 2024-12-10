use std::{collections::HashSet, vec};

use crate::{
    dir::Dir,
    point::Point,
    solution::{Day, Solutions},
};

pub struct Day10;

fn get_neighbors(p: Point<usize>, width: usize, height: usize) -> Vec<Point<usize>> {
    let mut points = vec![];
    if p.y > 0 {
        points.push(p.dir(&Dir::Up));
    }
    if p.x < width - 1 {
        points.push(p.dir(&Dir::Right));
    }
    if p.y < height - 1 {
        points.push(p.dir(&Dir::Down));
    }
    if p.x > 0 {
        points.push(p.dir(&Dir::Left));
    }
    return points;
}

fn get_trail_score_2(p: Point<usize>, context: &Day10Context, level: u8) -> i32 {
    if level == 9 {
        return 1;
    }

    let mut score = 0;
    for next in get_neighbors(p, context.width, context.height) {
        if context.grid[next.y][next.x] == level + 1 {
            score += get_trail_score_2(next, context, level + 1);
        }
    }
    return score;
}

fn get_trail_score_1(
    p: Point<usize>,
    context: &Day10Context,
    visited: &mut HashSet<Point<usize>>,
    level: u8,
) -> i32 {
    if visited.contains(&p) {
        return 0;
    }
    visited.insert(p);

    if level == 9 {
        return 1;
    }

    let mut score = 0;
    for next in get_neighbors(p, context.width, context.height) {
        if context.grid[next.y][next.x] == level + 1 {
            score += get_trail_score_1(next, context, visited, level + 1);
        }
    }
    return score;
}

pub struct Day10Context {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    trail_heads: Vec<Point<usize>>,
}

impl Day for Day10 {
    type Context = Day10Context;
    type Part1 = i32;
    type Part2 = i32;

    fn title() -> String {
        String::from("Hoof It")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(36),
            part1: Some(719),
            part2_example: Some(81),
            part2: Some(1530),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        let grid: Vec<Vec<u8>> = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();
        let width = grid[0].len();
        let height = grid.len();

        let mut trail_heads = vec![];
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == 0 {
                    trail_heads.push(Point::new(x, y));
                }
            }
        }

        Day10Context {
            grid,
            width,
            height,
            trail_heads,
        }
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        let mut score = 0;
        for trail_head in &context.trail_heads {
            let mut visited = HashSet::new();
            score += get_trail_score_1(*trail_head, context, &mut visited, 0);
        }
        return score;
    }

    fn solve_part2(context: &Self::Context) -> Self::Part2 {
        let mut score = 0;
        for trail_head in &context.trail_heads {
            score += get_trail_score_2(*trail_head, context, 0);
        }
        return score;
    }
}
