use std::fs;

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

fn main() {
    // Parse
    let path = "src/2024/day_02/in.txt";
    let content: Vec<Report> = fs::read_to_string(path)
        .expect(&format!("Expected input file at {path}"))
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse().expect("Expected a number"))
                .collect()
        })
        .collect();

    // Calculate
    println!(
        "Solution 1: {} (should be 379)",
        content
            .iter()
            .filter(|report| is_safe_report(&report))
            .count()
    );
    println!(
        "Solution 2: {} (should be 430)",
        content
            .iter()
            .filter(|report| is_probe_safe_report(&report))
            .count()
    );
}
