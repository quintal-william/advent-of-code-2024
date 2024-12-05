use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::{builder, value_parser, Parser, Subcommand};

use crate::solution::{Day, Year};

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

fn run(year_value: YearValue, day_value: DayValue, example: bool) {
    // Validate year and day
    let year = match year_value {
        2024 => Year2024,
        _ => panic!("Year {year_value} not found."),
    };
    let day = year
        .get_day(day_value)
        .expect(&format!("Day {day_value} not found in year {year_value}."));

    // Read input
    let ext = if example { "example" } else { "in" };
    let ref path = format!("src/year{year_value}/day{day_value:02}.{ext}");
    let ref input = fs::read_to_string(path).expect(&format!("Expected input file at {path}"));

    // Solve
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let ref context = day.create_context(input);
    let part_1 = day.solve_part_1(context);
    let part_2 = day.solve_part_2(context);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    // Report
    println!("AoC {year_value}-{day_value}: {}", day.title());
    println!("Solution 1: {}", part_1);
    println!("Solution 2: {}", part_2);

    let duration = end - start;
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    let micros = duration.subsec_micros() % 1_000;
    println!("Time: {}.{}{} seconds", secs, millis, micros);
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
