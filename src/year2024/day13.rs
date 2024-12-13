use regex::Regex;

use crate::{
    point::Point,
    solution::{Day, Solutions},
};

pub struct Day13;

pub struct Puzzle {
    a: Point<i64>,
    b: Point<i64>,
    prize: Point<i64>,
}

fn solve(p: &Puzzle) -> Option<(i64, i64)> {
    let b_nominator = p.prize.y * p.a.x - p.prize.x * p.a.y;
    let b_denominator = p.b.y * p.a.x - p.b.x * p.a.y;
    if b_nominator % b_denominator != 0 {
        return None;
    }
    let b = b_nominator / b_denominator;

    let a_nominator = p.prize.x - p.b.x * b;
    let a_denominator = p.a.x;
    if a_nominator % a_denominator != 0 {
        return None;
    };
    let a = a_nominator / a_denominator;

    return Some((a, b));
}

impl Day for Day13 {
    type Context = Vec<Puzzle>;
    type Part1 = i64;
    type Part2 = i64;

    fn title() -> String {
        String::from("Claw Contraption")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(480),
            part1: Some(26599),
            part2_example: Some(875318608908),
            part2: Some(106228669504887),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        let re = Regex::new(r"X(?:\+|=)(\d+),\sY(?:\+|=)(\d+)").unwrap();
        return input
            .split("\n\n")
            .map(|line| {
                let caps: Vec<(&str, [&str; 2])> =
                    re.captures_iter(line).map(|c| c.extract()).collect();
                let (_, [a_x, a_y]) = caps[0];
                let (_, [b_x, b_y]) = caps[1];
                let (_, [prize_x, prize_y]) = caps[2];
                return Puzzle {
                    a: Point::new(a_x.parse().unwrap(), a_y.parse().unwrap()),
                    b: Point::new(b_x.parse().unwrap(), b_y.parse().unwrap()),
                    prize: Point::new(prize_x.parse().unwrap(), prize_y.parse().unwrap()),
                };
            })
            .collect();
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        let mut coins = 0;
        for p in context {
            if let Some((a, b)) = solve(p) {
                coins += a * 3 + b;
            }
        }
        return coins;
    }

    fn solve_part2(context: &Self::Context) -> Self::Part2 {
        let mut coins = 0;
        let additional_steps = 10_000_000_000_000;
        let additional_point = Point::new(additional_steps, additional_steps);
        for p in context {
            let ref new_puzzle = Puzzle {
                a: p.a,
                b: p.b,
                prize: p.prize + additional_point,
            };
            if let Some((a, b)) = solve(new_puzzle) {
                coins += a * 3 + b;
            }
        }
        return coins;
    }
}
