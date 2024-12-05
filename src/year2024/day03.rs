use regex::Regex;
use std::fs;

pub struct Day03;

impl Day for Day03 {
    fn title(&self) -> &str {
        "Mull It Over"
    }
}

enum Command {
    Do,
    Dont,
    Mul(i32, i32),
}

fn command_from_caps(caps: regex::Captures) -> Command {
    if caps.get(1).is_some() {
        Command::Do
    } else if caps.get(2).is_some() {
        Command::Dont
    } else {
        let a = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
        let b = caps.get(5).unwrap().as_str().parse::<i32>().unwrap();
        Command::Mul(a, b)
    }
}

pub fn main() {
    let path = "src/year2024/day03/in.txt";
    let content = fs::read_to_string(path).expect(&format!("Expected input file at {path}"));

    let mut part1 = 0;
    let mut part2 = 0;
    let mut is_enabled = true;

    let re = Regex::new(r"(do)\(\)|(don't)\(\)|(mul)\((\d{1,3}),(\d{1,3})\)").unwrap();
    for caps in re.captures_iter(&content) {
        match command_from_caps(caps) {
            Command::Do => is_enabled = true,
            Command::Dont => is_enabled = false,
            Command::Mul(a, b) => {
                part1 += a * b;
                if is_enabled {
                    part2 += a * b;
                }
            }
        }
    }

    // 170778545
    println!("Solution 1: {}", part1);

    // 82868252
    println!("Solution 2: {}", part2);
}
