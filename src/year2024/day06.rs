use std::collections::HashSet;

use crate::solution::{Day, Solutions};

// pub enum TurnDir {
//     Right,
//     Left,
// }

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl Dir {
    // pub fn opposite(&self) -> Dir {
    //     match self {
    //         Dir::Up => Dir::Down,
    //         Dir::Right => Dir::Left,
    //         Dir::Down => Dir::Up,
    //         Dir::Left => Dir::Right,
    //     }
    // }
    pub fn clockwise(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
    // pub fn counter_clockwise(&self) -> Dir {
    //     match self {
    //         Dir::Up => Dir::Left,
    //         Dir::Right => Dir::Up,
    //         Dir::Down => Dir::Right,
    //         Dir::Left => Dir::Down,
    //     }
    // }
    // pub fn turn(&self, turn: TurnDir) -> Dir {
    //     match turn {
    //         TurnDir::Right => self.clockwise(),
    //         TurnDir::Left => self.counter_clockwise(),
    //     }
    // }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    point: Point,
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

        // TODO refactor this spaghetti
        match s.dir {
            Dir::Up => {
                if s.point.y == 0 {
                    break;
                }
                let next = context.grid[s.point.y - 1][s.point.x];
                if next == '#' {
                    s.dir = s.dir.clockwise()
                }
                if next == '.' || next == '^' {
                    s.point.y -= 1;
                }
            }
            Dir::Right => {
                if s.point.x == context.width - 1 {
                    break;
                }
                let next = context.grid[s.point.y][s.point.x + 1];
                if next == '#' {
                    s.dir = s.dir.clockwise()
                }
                if next == '.' || next == '^' {
                    s.point.x += 1;
                }
            }
            Dir::Down => {
                if s.point.y == context.height - 1 {
                    break;
                }
                let next = context.grid[s.point.y + 1][s.point.x];
                if next == '#' {
                    s.dir = s.dir.clockwise()
                }
                if next == '.' || next == '^' {
                    s.point.y += 1;
                }
            }
            Dir::Left => {
                if s.point.x == 0 {
                    break;
                }
                let next = context.grid[s.point.y][s.point.x - 1];
                if next == '#' {
                    s.dir = s.dir.clockwise()
                }
                if next == '.' || next == '^' {
                    s.point.x -= 1;
                }
            }
        }
    }

    return PathEval {
        is_loop: false,
        visited,
    };
}

#[derive(Debug)]
pub struct Day06Context {
    state: State,
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
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
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let guard_start_index = input.find('^').unwrap();
        let guard_start_x = guard_start_index % (grid[0].len() + 1);
        let guard_start_y = guard_start_index / (grid[0].len() + 1);

        Day06Context {
            state: State {
                dir: Dir::Up,
                point: Point {
                    x: guard_start_x,
                    y: guard_start_y,
                },
            },
            width: grid[0].len(),
            height: grid.len(),
            grid,
        }
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        let path = eval_guard_path(context);
        return HashSet::<Point>::from_iter(path.visited.iter().map(|p| p.point)).len();
    }

    fn solve_part2(_context: &Self::Context) -> Self::Part2 {
        0
        // let path = eval_guard_path(context);
        // let visited_points = HashSet::<Point>::from_iter(path.visited.iter().map(|p| p.point));
        // let mut score = 0;
        // // TODO multithread this?
        // for p in visited_points {
        //     // Lconst newContent = content.slice(0, y * (width + 1) + x) + '#' + content.slice(y * (width + 1) + x + 1);
        //     // const res = guardInLoop(newContent.split('\n'));
        //     // score += res === true ? 1 : 0;
        // }
        // return score;
    }
}
