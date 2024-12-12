use std::collections::HashSet;

use crate::{
    dir::Dir,
    grid::Grid,
    point::Point,
    solution::{Day, Solutions},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    point: Point<usize>,
    dir: Dir,
}

#[derive(Debug)]
struct PathEval {
    is_loop: bool,
    visited: HashSet<State>,
}

fn eval_guard_path(context: &Day06Context) -> PathEval {
    let mut s = context.state.clone();
    let mut visited: HashSet<State> = HashSet::new();

    loop {
        if visited.contains(&s) {
            return PathEval {
                is_loop: true,
                visited,
            };
        }

        visited.insert(s.clone());

        match s.dir {
            Dir::Up => {
                if s.point.y == 0 {
                    break;
                }
            }
            Dir::Right => {
                if s.point.x == context.grid.width() - 1 {
                    break;
                }
            }
            Dir::Down => {
                if s.point.y == context.grid.height() - 1 {
                    break;
                }
            }
            Dir::Left => {
                if s.point.x == 0 {
                    break;
                }
            }
        }

        let next_point = s.point.dir(&s.dir);
        let next = *context.grid.get(&next_point);
        if next == '#' {
            s.dir = s.dir.clockwise()
        }
        if next == '.' || next == '^' {
            s.point = next_point;
        }
    }

    return PathEval {
        is_loop: false,
        visited,
    };
}

#[derive(Debug, Clone)]
pub struct Day06Context {
    state: State,
    grid: Grid<char>,
}

pub struct Day06;

impl Day for Day06 {
    type Context = Day06Context;
    type Part1 = usize;
    type Part2 = usize;

    fn title() -> String {
        String::from("Guard Gallivant")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(41),
            part1: Some(4826),
            part2_example: Some(6),
            part2: Some(1721),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        let grid = Grid::from_input(input, |e| e);
        let guard_start_index = input.find('^').unwrap();

        Day06Context {
            state: State {
                dir: Dir::Up,
                point: Point {
                    x: guard_start_index % (grid.width() + 1),
                    y: guard_start_index / (grid.width() + 1),
                },
            },
            grid,
        }
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        return HashSet::<Point<usize>>::from_iter(
            eval_guard_path(context).visited.iter().map(|p| p.point),
        )
        .len();
    }

    fn solve_part2(context: &Self::Context) -> Self::Part2 {
        let visited_points = HashSet::<Point<usize>>::from_iter(
            eval_guard_path(context).visited.iter().map(|p| p.point),
        );

        let mut obstacles = 0;
        for p in visited_points {
            let mut new_context = context.clone();
            new_context.grid.set(&p, '#');
            let path = eval_guard_path(&new_context);
            if path.is_loop {
                obstacles += 1;
            }
        }
        return obstacles;
    }
}
