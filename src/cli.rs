use std::time::{Duration, SystemTime, UNIX_EPOCH};

use clap::{builder, value_parser, Parser, Subcommand};
use cli_colors::Colorizer;

use crate::solution::{InputType, Year};
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

fn report_duration(duration: Duration) {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    let micros = duration.subsec_micros() % 1_000;
    println!("Execution time: {}.{}{} seconds", secs, millis, micros);
}

fn get_icon(is_correct: Option<bool>) -> String {
    let colorizer = Colorizer::new();
    let error_icon = colorizer.bold(colorizer.bright_red("x"));
    let success_icon = colorizer.bold(colorizer.bright_green("✓"));
    let unknown_icon = colorizer.bold(colorizer.white("?"));

    is_correct.map_or(unknown_icon, |is_correct| {
        if is_correct {
            success_icon
        } else {
            error_icon
        }
    })
}

fn run(year: YearValue, day: DayValue, example: bool) {
    let input_type = if example {
        InputType::Example
    } else {
        InputType::Puzzle
    };

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let day_output = match year {
        2024 => Year2024::solve_day(year, day, input_type),
        _ => panic!("Year {year} not found."),
    }
    .expect(&format!("Day {day} not found for year {year}."));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let duration = end - start;

    println!("AoC {year}-{day}: {}", day_output.title);
    println!(
        "{} Solution 1: {}",
        get_icon(day_output.part1.is_correct),
        day_output.part1.value
    );
    println!(
        "{} Solution 2: {}",
        get_icon(day_output.part2.is_correct),
        day_output.part2.value
    );
    report_duration(duration);
}

fn all(year: YearValue) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let solutions_list = match year {
        2024 => Year2024::solve_all(year),
        _ => panic!("Year {year} not found."),
    };
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let duration = end - start;

    for y in 0..5 {
        if y == 0 {
            println!("+-=-AoC {year}: All solutions-=-+");
            println!("+-----------------------------+");
        }

        for x in 0..5 {
            if x == 0 {
                print!("|");
            }
            let s = &solutions_list[y * 5 + x];
            print!("{}", format!("{:0>2} ", y * 5 + x + 1));
            if s.is_none() {
                let colorizer = Colorizer::new();
                let missing_icon = colorizer.bold(colorizer.white("-"));
                print!("{missing_icon}{missing_icon}|")
            } else {
                print!("{}", get_icon(s.as_ref().unwrap().0.part1.is_correct));
                print!("{}", get_icon(s.as_ref().unwrap().0.part2.is_correct));
                print!("|");
            }
        }
        println!();
        for x in 0..5 {
            if x == 0 {
                print!("|");
            }
            let s = &solutions_list[y * 5 + x];
            print!("   ");
            if s.is_none() {
                let colorizer = Colorizer::new();
                let missing_icon = colorizer.bold(colorizer.white("-"));
                print!("{missing_icon}{missing_icon}|")
            } else {
                print!("{}", get_icon(s.as_ref().unwrap().1.part1.is_correct));
                print!("{}", get_icon(s.as_ref().unwrap().1.part2.is_correct));
                print!("|");
            }
        }
        println!();
        println!("+-----------------------------+");
    }

    report_duration(duration);
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
