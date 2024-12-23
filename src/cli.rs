use crate::year::Year;
use crate::year2024;
use crate::{day::DayValue, point::Point, year::YearValue};
use clap::{builder, value_parser, Parser, Subcommand};
use console::{style, StyledObject, Term};
use indicatif::ProgressIterator;
use std::io::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub enum InputType {
    Example,
    Puzzle,
}

const DEFAULT_YEAR: &str = "2024";
fn validate_year() -> builder::RangedI64ValueParser<YearValue> {
    return value_parser!(YearValue).range(2024..2025);
}

fn validate_day() -> builder::RangedI64ValueParser<DayValue> {
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
        #[arg(short, long, value_parser = validate_year(), default_value = DEFAULT_YEAR)]
        year: YearValue,
        #[arg(short, long, value_parser = validate_day())]
        day: DayValue,
        #[arg(short, long)]
        example: bool,
    },

    /// Runs all available Advent of Code solutions
    All {
        #[arg(short, long, value_parser = validate_year(), default_value = DEFAULT_YEAR)]
        year: YearValue,
    },

    Bench {
        #[arg(short, long, value_parser = validate_year(), default_value = DEFAULT_YEAR)]
        year: YearValue,
        #[arg(short, long, value_parser = validate_day())]
        day: DayValue,
        #[arg(short, long)]
        example: bool,
        #[arg(short, long, value_parser = value_parser!(u32).range(1..))]
        iterations: u32
    },

    /// Adds a new empty Advent of Code solution
    Add {
        #[arg(short, long, value_parser = validate_year(), default_value = DEFAULT_YEAR)]
        year: YearValue,
        #[arg(short, long, value_parser = validate_day())]
        day: DayValue,
    },
}

fn get_icon(is_correct: Option<bool>) -> StyledObject<String> {
    let error_icon = style(String::from("x")).bold().red();
    let success_icon = style(String::from("✓")).bold().green();
    let unknown_icon = style(String::from("?")).bold().dim();

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
    // TODO do this in an iterator and draw to console incrementally
    let day_output = match year {
        2024 => year2024::Year2024::solve_day(year, day, input_type),
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
    println!("Execution time: {:?}", duration);
}

fn all(year: YearValue) {
    let term = &mut Term::stdout();
    let solutions = match year {
        2024 => year2024::Year2024::solve_all(year),
        _ => panic!("Year {year} not found."),
    };

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    const CELL_HEIGHT: isize = 3;
    const CELL_WIDTH: isize = 6;
    const CALENDAR_CELLS_SIZE: isize = 5;
    const CALENDAR_PIXEL_HEIGHT: isize = CALENDAR_CELLS_SIZE * CELL_HEIGHT + 1;
    const CALENDAR_PIXEL_WIDTH: isize = CALENDAR_CELLS_SIZE * CELL_WIDTH + 1;

    fn write(term: &mut Term, s: &str) {
        term.write(s.as_bytes()).unwrap();
    }

    fn move_cursor(term: &mut Term, p: Point<isize>) {
        if p.x > 0 {
            term.move_cursor_right(p.x as usize).unwrap();
        } else if p.x < 0 {
            term.move_cursor_left(p.x.abs() as usize).unwrap();
        }
        if p.y > 0 {
            term.move_cursor_down(p.y as usize).unwrap();
        } else if p.y < 0 {
            term.move_cursor_up(p.y.abs() as usize).unwrap();
        }
    }

    for (day, solution) in solutions.enumerate() {
        if day == 0 {
            write(term, &"\n".repeat(CALENDAR_PIXEL_HEIGHT as usize + 1));
            move_cursor(term, Point::new(0, -1 * CALENDAR_PIXEL_HEIGHT - 1));
            write(term, &format!("+-=-AoC {year}: All solutions-=-+\n"));
        }

        write(term, "+-----+");
        move_cursor(term, Point::new(-1 * (CELL_WIDTH + 1), 1));

        let example_icons = solution.as_ref().map_or_else(
            || {
                let missing_icon = style("-").bold().dim();
                return format!("{missing_icon}{missing_icon}");
            },
            |output| {
                let part1_icon = get_icon(output.part1_example.is_correct);
                let part2_icon = get_icon(output.part2_example.is_correct);
                return format!("{part1_icon}{part2_icon}");
            },
        );

        write(term, &format!("|{:0>2} {example_icons}|", day + 1));
        move_cursor(term, Point::new(-1 * (CELL_WIDTH + 1), 1));

        let puzzle_icons = solution.as_ref().map_or_else(
            || {
                let missing_icon = style("-").bold().dim();
                return format!("{missing_icon}{missing_icon}");
            },
            |output| {
                let part1_icon = get_icon(output.part1.is_correct);
                let part2_icon = get_icon(output.part2.is_correct);
                return format!("{part1_icon}{part2_icon}");
            },
        );

        write(term, &format!("|   {puzzle_icons}|"));
        move_cursor(term, Point::new(-1 * (CELL_WIDTH + 1), 1));
        write(term, "+-----+");

        if day % 5 == 4 {
            move_cursor(term, Point::new(-1 * CALENDAR_PIXEL_WIDTH, 0));
        } else {
            move_cursor(term, Point::new(-1, -1 * CELL_HEIGHT));
        }

        if day == 24 {
            move_cursor(term, Point::new(0, 1));
        }
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Total execution time: {:?}", end - start);
}

fn bench(year: YearValue, day: DayValue, example: bool, iterations: u32) {
    run(year, day, example);
    
    let input_type = if example {
        InputType::Example
    } else {
        InputType::Puzzle
    };

    let mut moving_mean = 0.0;
    for i in (0..iterations).progress() {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        match year {
            2024 => year2024::Year2024::solve_day(year, day, input_type.clone()),
            _ => panic!("Year {year} not found."),
        }
        .expect(&format!("Day {day} not found for year {year}."));
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        moving_mean += ((end - start).as_nanos() as f64 - moving_mean) / ((i as f64) + 1.0);
    }
    println!("Mean execution time: {:?}", Duration::from_nanos(moving_mean as u64));
}

fn add(_year: YearValue, _day: DayValue) {
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
        Commands::Bench { year, day, example, iterations } => {
            bench(year, day, example, iterations);
        }
        Commands::Add { year, day } => {
            add(year, day);
        }
    }
}
