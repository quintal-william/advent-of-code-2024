use std::time::{Duration, SystemTime, UNIX_EPOCH};

use clap::{builder, value_parser, Parser, Subcommand};
use cli_colors::Colorizer;

use crate::solution::{Solution, Year};
use crate::year2024::Year2024;

pub type YearValue = u16;
const DEFAULT_YEAR: &str = "2024";
fn year() -> builder::RangedI64ValueParser<YearValue> {
    return value_parser!(YearValue).range(2024..2025);
}

pub type DayValue = u8;
fn day() -> builder::RangedI64ValueParser<DayValue> {
    return value_parser!(DayValue).range(1..26);
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs one Advent of Code solution
    Run {
        #[arg(short, long, value_parser = year(), default_value = DEFAULT_YEAR)]
        year: YearValue,
        #[arg(short, long, value_parser = day())]
        day: DayValue,
        #[arg(short, long)]
        example: bool,
    },

    /// Runs all available Advent of Code solutions
    All {
        #[arg(short, long, value_parser = year(), default_value = DEFAULT_YEAR)]
        year: YearValue,
    },

    /// Adds a new empty Advent of Code solution
    Add {
        #[arg(short, long, value_parser = year(), default_value = DEFAULT_YEAR)]
        year: YearValue,
        #[arg(short, long, value_parser = day())]
        day: DayValue,
        #[arg(short, long)]
        title: String,
    },
}

fn report_run(
    year: YearValue,
    day: DayValue,
    part1: Solution,
    part2: Solution,
    title: String,
    duration: Duration,
) {
    let colorizer = Colorizer::new();
    let error_icon = colorizer.bold(colorizer.bright_red("x"));
    let success_icon = colorizer.bold(colorizer.bright_green("✓"));
    let unknown_icon = colorizer.bold(colorizer.white("?"));

    let part_1_icon = part1.is_correct.map_or(&unknown_icon, |is_correct| {
        if is_correct {
            &success_icon
        } else {
            &error_icon
        }
    });
    let part_2_icon = part2.is_correct.map_or(&unknown_icon, |is_correct| {
        if is_correct {
            &success_icon
        } else {
            &error_icon
        }
    });

    // Report
    println!("AoC {year}-{day}: {}", title);
    println!("{part_1_icon} Solution 1: {}", part1.value);
    println!("{part_2_icon} Solution 2: {}", part2.value);

    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    let micros = duration.subsec_micros() % 1_000;
    println!("Time: {}.{}{} seconds", secs, millis, micros);
}

fn run(year: YearValue, day: DayValue, example: bool) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let solutions = match year {
        2024 => Year2024::solve_day(year, day),
        _ => panic!("Year {year} not found."),
    }
    .expect(&format!("Day {day} not found for year {year}."));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let duration = end - start;

    if example {
        report_run(
            year,
            day,
            solutions.part1_example,
            solutions.part2_example,
            solutions.title,
            duration,
        );
    } else {
        report_run(
            year,
            day,
            solutions.part1,
            solutions.part2,
            solutions.title,
            duration,
        );
    };
}

fn all(_year: YearValue) {
    unimplemented!();
}

fn add(_year: YearValue, _day: DayValue, _title: String) {
    unimplemented!();
}

pub fn start() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { year, day, example } => {
            run(year, day, example);
        }
        Commands::All { year } => {
            all(year);
        }
        Commands::Add { year, day, title } => {
            add(year, day, title);
        }
    }
}
