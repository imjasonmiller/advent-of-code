use clap::{crate_authors, Parser};

use std::{
    fs,
    io::{self, Read},
    num::ParseIntError,
    path::PathBuf,
    str::FromStr,
};

use anyhow::Context;
use thiserror::Error;

mod day1;
// mod day10;
// mod day11;
// mod day18;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day9;
// mod day8;

macro_rules! solution_days {
    (
        $($Day:ident)*
    ) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum SolutionDay {
            $($Day,)*
        }

        impl FromStr for SolutionDay {
            type Err = SolutionDayError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let value = s.parse::<u8>()?;

                let candidate = 1;

                $(
                    #[allow(unused_variables)]
                    let candidate = {
                        if value == candidate {
                            return Ok(SolutionDay::$Day)
                        } else {
                            candidate + 1
                        }
                    };
                )*

                Err(SolutionDayError::BadDay(value))
            }
        }
    };
}

#[derive(Debug, Clone, Error)]
pub enum SolutionDayError {
    #[error("Failed to parse day: {0}")]
    Parse(#[from] ParseIntError),

    #[error("{0} is not an Advent Puzzle Day")]
    BadDay(u8),
}

#[rustfmt::skip]
solution_days! {
    day1 day2 day3 day4 day5
    day6 day7 day8 day9 day10
    day11 day12 day13 day14 day15
    day16 day17 day18 day19 day20
    day21 day22 day23 day24 day25 
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolutionPart {
    part1,
    part2,
}

#[derive(Debug, Clone, Error)]
pub enum SolutionPartError {
    #[error("Failed to parse day: {0}")]
    Parse(#[from] ParseIntError),

    #[error("{0} is not an Advent Puzzle  Part; must be 1 or 2")]
    BadPart(u8),
}
impl FromStr for SolutionPart {
    type Err = SolutionPartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<u8>()?;

        match value {
            1 => Ok(SolutionPart::part1),
            2 => Ok(SolutionPart::part2),
            n => Err(SolutionPartError::BadPart(n)),
        }
    }
}

macro_rules! match_puzzles {
    ($day:expr, $part:expr, $input:expr; $(
        $Day:ident, $Part:ident;
    )*) => {{
        #[allow(unreachable_patterns)]
        match ($day, $part) {
            $(
                (SolutionDay::$Day, SolutionPart::$Part) => println!("{}", $Day::$Part($input)?),
            )*
            (day, part) => anyhow::bail!("No solution for {:?}, {:?}", day, part),
        }
    }};
}

#[derive(Parser, Debug)]
#[clap(author = crate_authors!())]
struct Opts {
    #[clap(short, long, about = "Day of puzzle")]
    day: SolutionDay,
    #[clap(short, long, about = "Part of daily puzzle")]
    part: SolutionPart,
    #[clap(short, long, about = "Input for puzzle")]
    input: Option<PathBuf>,
}


fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let mut input = String::new();

    match opts.input {
        Some(path) => {
            let mut file = fs::File::open(&path)
                .with_context(|| format!("Failed to open input file '{}'", path.display()))?;

            file.read_to_string(&mut input)
                .with_context(|| format!("Failed to read from input file '{}'", path.display()))?;
        }
        None => {
            io::stdin()
                .read_to_string(&mut input)
                .context("Failed to read from stdin")?;
        }
    }

    match_puzzles! {
        opts.day, opts.part, &input;

        day1, part1;
        day1, part2;
        // day2, part1;
        // day2, part2;
        // day3, part1;
        // day3, part2;
        // day4, part1;
        // day4, part2;
        // day5, part1;
        // day5, part2;
        // day6, part1;
        // day6, part2;
        // day7, part1;
        // day7, part2;
        // day8, part1;
        // day8, part2;
        // day9, part1;
        // day9, part2;
        // day10, part1;
        // day10, part2;
        // day11, part1;
        // day11, part2;
        // day18, part1;
        // day18, part2;
    }

    Ok(())
}
