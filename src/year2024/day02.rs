use crate::day::{Day, Solutions};

pub struct Day02;

type Report = Vec<i32>;

enum Trend {
    Up,
    Down,
}

fn is_safe_report(report: &Report) -> bool {
    let mut trend: Option<Trend> = None;
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];

        if diff < -3 || diff > 3 || diff == 0 {
            return false;
        }

        match trend {
            None => {
                if diff > 0 {
                    trend = Some(Trend::Up);
                } else if diff < 0 {
                    trend = Some(Trend::Down);
                }
            }
            Some(Trend::Up) => {
                if diff < 0 {
                    return false;
                }
            }
            Some(Trend::Down) => {
                if diff > 0 {
                    return false;
                }
            }
        }
    }
    return true;
}

fn is_probe_safe_report(report: &Report) -> bool {
    if is_safe_report(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if is_safe_report(&modified_report) {
            return true;
        }
    }

    return false;
}

impl Day for Day02 {
    type Context = Vec<Report>;
    type Part1 = usize;
    type Part2 = usize;

    fn title() -> String {
        String::from("Red-Nosed Reports")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(2),
            part1: Some(379),
            part2_example: Some(4),
            part2: Some(430),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        input
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|num| num.parse().expect("Expected a number"))
                    .collect()
            })
            .collect()
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        context
            .iter()
            .filter(|report| is_safe_report(&report))
            .count()
    }

    fn solve_part2(context: &Self::Context) -> Self::Part2 {
        context
            .iter()
            .filter(|report| is_probe_safe_report(&report))
            .count()
    }
}
