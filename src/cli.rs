use crate::point::Point;
use crate::solution::{InputType, Year};
use crate::year2024::Year2024;
use clap::{builder, value_parser, Parser, Subcommand};
use console::{style, StyledObject, Term};
use std::io::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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

    Bench {
        #[arg(short, long, value_parser = year(), default_value = DEFAULT_YEAR)]
        year: YearValue,
        #[arg(short, long, value_parser = day())]
        day: DayValue,
        #[arg(short, long)]
        example: bool,
    },

    /// Adds a new empty Advent of Code solution
    Add {
        #[arg(short, long, value_parser = year(), default_value = DEFAULT_YEAR)]
        year: YearValue,
        #[arg(short, long, value_parser = day())]
        day: DayValue,
    },
}

fn report_duration(duration: Duration) {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    let micros = duration.subsec_micros() % 1_000;
    println!("Execution time: {}.{}{} seconds", secs, millis, micros);
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
    let term = &mut Term::stdout();
    let solutions = match year {
        2024 => Year2024::solve_all(year),
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
    report_duration(end - start);
}

fn bench(_year: YearValue, _day: DayValue, _example: bool) {
    unimplemented!();
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
        Commands::Bench { year, day, example } => {
            bench(year, day, example);
        }
        Commands::Add { year, day } => {
            add(year, day);
        }
    }
}
