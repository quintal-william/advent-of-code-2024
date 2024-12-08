use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    vec,
};

use crate::solution::{Day, Solutions};

pub struct Day08;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

type Frequencies = HashMap<char, Vec<Point>>;

fn for_all_pairs_of_antennas<F>(frequencies: &Frequencies, mut f: F)
where
    F: FnMut(&Point, &Point),
{
    for antennas in frequencies.values() {
        if antennas.len() < 2 {
            return;
        }

        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                f(&antennas[i], &antennas[j]);
            }
        }
    }
}

fn get_antinodes(
    start_x: i32,
    start_y: i32,
    dx: i32,
    dy: i32,
    width: usize,
    height: usize,
) -> Vec<Point> {
    let mut antinodes = vec![];
    let mut x = start_x;
    let mut y = start_y;

    while x >= 0 && x < (width as i32) && y >= 0 && y < (height as i32) {
        antinodes.push(Point { x, y });
        x += dx;
        y += dy;
    }
    return antinodes;
}

#[derive(Debug)]
pub struct Day08Context {
    frequencies: Frequencies,
    width: usize,
    height: usize,
}

impl Day for Day08 {
    type Context = Day08Context;
    type Part1 = usize;
    type Part2 = usize;

    fn title() -> String {
        String::from("Resonant Collinearity")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(14),
            part1: Some(289),
            part2_example: Some(34),
            part2: Some(1030),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut frequencies = HashMap::new();
        for (y, r) in grid.iter().enumerate() {
            for (x, c) in r.iter().enumerate() {
                if *c == '.' {
                    continue;
                }
                let mut vec = frequencies.get(c).unwrap_or(&vec![]).to_owned();
                vec.push(Point {
                    x: x as i32,
                    y: y as i32,
                });
                frequencies.insert(*c, vec);
            }
        }

        Day08Context {
            frequencies,
            width: grid[0].len(),
            height: grid.len(),
        }
    }

    fn solve_part_1(context: &Self::Context) -> Self::Part1 {
        let mut antinodes: HashSet<Point> = HashSet::new();

        for_all_pairs_of_antennas(&context.frequencies, |a, b| {
            let dx = b.x - a.x;
            let dy = b.y - a.y;

            let antinodes_a =
                get_antinodes(a.x, a.y, -1 * dx, -1 * dy, context.width, context.height);
            let antinode_a = antinodes_a.get(1);
            if antinode_a.is_some() {
                antinodes.insert(*antinode_a.unwrap());
            }

            let antinodes_b = get_antinodes(b.x, b.y, dx, dy, context.width, context.height);
            let antinode_b = antinodes_b.get(1);
            if antinode_b.is_some() {
                antinodes.insert(*antinode_b.unwrap());
            }
        });

        return antinodes.len();
    }

    fn solve_part_2(context: &Self::Context) -> Self::Part2 {
        let mut antinodes: HashSet<Point> = HashSet::new();

        for_all_pairs_of_antennas(&context.frequencies, |a, b| {
            let dx = b.x - a.x;
            let dy = b.y - a.y;

            let antinodes_a =
                get_antinodes(a.x, a.y, -1 * dx, -1 * dy, context.width, context.height); // TODO this is double work from part_1
            let antinodes_b = get_antinodes(b.x, b.y, dx, dy, context.width, context.height);

            antinodes.extend(antinodes_a);
            antinodes.extend(antinodes_b);
        });

        return antinodes.len();
    }
}
