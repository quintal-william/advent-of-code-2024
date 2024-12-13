use std::{collections::HashSet, vec};

use crate::{
    day::{Day, Solutions},
    dir::Dir,
    grid::Grid,
    point::Point,
};

pub struct Day10;

fn get_neighbors(p: Point<usize>, grid: &Grid<u8>) -> Vec<Point<usize>> {
    let mut points = vec![];
    if p.y > 0 {
        points.push(p.dir(&Dir::Up));
    }
    if p.x < grid.width() - 1 {
        points.push(p.dir(&Dir::Right));
    }
    if p.y < grid.height() - 1 {
        points.push(p.dir(&Dir::Down));
    }
    if p.x > 0 {
        points.push(p.dir(&Dir::Left));
    }
    return points;
}

fn get_trail_score(
    p: Point<usize>,
    grid: &Grid<u8>,
    level: u8,
    visited: &mut Option<HashSet<Point<usize>>>,
) -> i32 {
    if let Some(set) = visited {
        if set.contains(&p) {
            return 0;
        }
        set.insert(p);
    }

    if level == 9 {
        return 1;
    }

    let mut score = 0;
    for next in get_neighbors(p, grid) {
        if *grid.get(&next) == level + 1 {
            score += get_trail_score(next, grid, level + 1, visited);
        }
    }
    return score;
}

pub struct Day10Context {
    grid: Grid<u8>,
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
        let content: Vec<u8> = input
            .replace("\n", "")
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let width = input.find('\n').unwrap();
        let grid = Grid::from_vec(content, width);

        let mut trail_heads = vec![];
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let p = Point { x, y };
                if *grid.get(&p) == 0 {
                    trail_heads.push(p);
                }
            }
        }

        Day10Context { grid, trail_heads }
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        let mut score = 0;
        for trail_head in &context.trail_heads {
            score += get_trail_score(*trail_head, &context.grid, 0, &mut Some(HashSet::new()));
        }
        return score;
    }

    fn solve_part2(context: &Self::Context) -> Self::Part2 {
        let mut score = 0;
        for trail_head in &context.trail_heads {
            score += get_trail_score(*trail_head, &context.grid, 0, &mut None);
        }
        return score;
    }
}
