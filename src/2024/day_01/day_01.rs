use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    // Parse
    let (left, right): (Vec<i32>, Vec<i32>) = fs::read_to_string("2024/day-01/in.txt")
        .expect("Expected file at 2024/day-01/in.txt")
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line
                .split_whitespace()
                .map(|part| part.parse().expect("Expected a number"))
                .collect();
            (parts[0], parts[1])
        })
        .unzip();

    // Calculate
    let mut occurrences = HashMap::new();
    for &num in &right {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for (l, r) in left.iter().sorted().zip(right.iter().sorted()) {
        part1 += (l - r).abs();
        part2 += l * (occurrences.get(&l).unwrap_or(&0))
    }

    println!("Solution 1: {}", part1);
    println!("Solution 2: {}", part2);
}
