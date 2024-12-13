use crate::day::{Day, Solutions};

pub struct Day07;

pub struct Equation {
    target: u64,
    nums: Vec<u64>,
}

#[derive(Clone, Copy)]
enum Operator {
    Sum,
    Multiply,
    Concat,
}

fn equation_is_possible(eq: &Equation, operators: &Vec<Operator>, node: Vec<Operator>) -> bool {
    if node.len() + 1 == eq.nums.len() {
        let mut outcome = 0;
        for i in 0..eq.nums.len() {
            if i == 0 {
                outcome = eq.nums[i]
            } else {
                match node[i - 1] {
                    Operator::Sum => outcome += eq.nums[i],
                    Operator::Multiply => outcome *= eq.nums[i],
                    Operator::Concat => {
                        outcome = format!("{}{}", outcome, eq.nums[i]).parse().unwrap()
                    }
                }
            }
        }
        return outcome == eq.target;
    }

    for operator in operators {
        let mut next_node = node.clone();
        next_node.push(*operator);
        if equation_is_possible(eq, operators, next_node) {
            return true;
        }
    }
    return false;
}

fn evaluate(equations: &Vec<Equation>, operators: Vec<Operator>) -> u64 {
    equations
        .iter()
        .map(|eq| {
            if equation_is_possible(eq, &operators, vec![]) {
                eq.target
            } else {
                0
            }
        })
        .sum()
}

impl Day for Day07 {
    type Context = Vec<Equation>;
    type Part1 = u64;
    type Part2 = u64;

    fn title() -> String {
        String::from("Bridge Repair")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(3749),
            part1: Some(465126289353),
            part2_example: Some(11387),
            part2: Some(70597497486371),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        input
            .lines()
            .map(|line| {
                let split_line: Vec<&str> = line.split(": ").collect();
                let target = split_line[0].parse().unwrap();
                let nums: Vec<u64> = split_line[1]
                    .split(' ')
                    .map(|n| n.parse().unwrap())
                    .collect();
                Equation { target, nums }
            })
            .collect()
    }

    fn solve_part1(context: &Self::Context) -> Self::Part1 {
        let operators = vec![Operator::Sum, Operator::Multiply];
        evaluate(context, operators)
    }

    fn solve_part2(context: &Self::Context) -> Self::Part2 {
        let operators = vec![Operator::Sum, Operator::Multiply, Operator::Concat];
        evaluate(context, operators)
    }
}
