use std::time::{SystemTime, UNIX_EPOCH};

use clap::{builder, value_parser, Parser, Subcommand};

type Year = u16;
const DEFAULT_YEAR: &str = "2024";
fn year() -> builder::RangedI64ValueParser<Year> {
    return value_parser!(Year).range(2024..2025);
}

type Day = u8;
fn day() -> builder::RangedI64ValueParser<Day> {
    return value_parser!(Day).range(1..26);
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
        year: Year,
        #[arg(short, long, value_parser = day())]
        day: Day,
        #[arg(short, long)]
        example: bool,
    },

    /// Runs all available Advent of Code solutions
    All {
        #[arg(short, long, value_parser = year(), default_value = DEFAULT_YEAR)]
        year: Year,
    },

    /// Adds a new empty Advent of Code solution
    Add {
        #[arg(short, long, value_parser = year(), default_value = DEFAULT_YEAR)]
        year: Year,
        #[arg(short, long, value_parser = day())]
        day: Day,
    },
}

fn run(_year: Year, _day: Day, _example: bool) {
    unimplemented!();
    // let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    //do my stuff
    // let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
}

fn all(_year: Year) {
    unimplemented!();
}

fn add(_year: Year, _day: Day) {
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
        Commands::Add { year, day } => {
            add(year, day);
        }
    }
}
