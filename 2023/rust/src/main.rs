#![feature(btree_cursors)]
#![feature(let_chains)]

use std::{env, time::Instant};

use chrono::{self, Datelike, NaiveDate};
use clap::Parser;
use day6::Day6;
use day7::Day7;

use crate::{day1::Day1, day2::Day2, day3::Day3, day4::Day4, day5::Day5};

fn year_and_day() -> (i32, i32) {
    let now = chrono::offset::Local::now().date_naive();

    let day = now.num_days_from_ce()
        - NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
            .expect("invalid date!")
            .num_days_from_ce();

    // zero-indexed -> one-indexed
    let day = day + 1;

    let (_, year) = now.year_ce();

    (year as i32, day)
}

pub struct TestInput {
    input: &'static str,
    result: &'static str,
}

trait Day {
    fn date() -> (i32, i32);

    fn part1(lines: &[String]) -> String;
    fn part2(lines: &[String]) -> String;

    fn test_inputs() -> (TestInput, TestInput);
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn day_str(day: i32) -> String {
    let suffix = match day % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };

    format!("{day}{suffix}")
}

#[derive(Parser)]
struct Args {
    #[arg(value_parser = clap::value_parser!(i32).range(1..=25))]
    day: Option<i32>,

    #[arg(short, long)]
    test: bool,

    #[arg(short, long)]
    all: bool,
}

fn main() {
    let args = Args::parse();

    if args.all && args.day.is_some() {
        eprintln!("Can't specify `-a/--all` and also provide a day!");
        return;
    }

    let (year, day) = year_and_day();
    println!("Today is the {} of December {year}\n", day_str(day));

    let days = if args.all {
        1..26
    } else {
        let day = args.day.unwrap_or(day);
        day..(day + 1)
    };

    for day in days {
        let Ok(()) = run_day_from_date(&args, day) else {
            if !args.all {
                eprintln!("Day {day} not implemented!");
            }

            break;
        };
    }
}

fn run_day_from_date(args: &Args, day: i32) -> Result<(), ()> {
    let run_day = match day {
        1 => run_day::<Day1>,
        2 => run_day::<Day2>,
        3 => run_day::<Day3>,
        4 => run_day::<Day4>,
        5 => run_day::<Day5>,
        6 => run_day::<Day6>,
        7 => run_day::<Day7>,
        _day => Err(())?,
    };

    run_day(args);
    Ok(())
}

fn get_day_input<Day: crate::Day>() -> String {
    let session = env::var("AOC_SESSION")
        .expect("must set `AOC_SESSION` env var to your advent of code session cookie");

    let (year, day) = Day::date();
    ureq::get(&format!("https://adventofcode.com/{year}/day/{day}/input"))
        .set("Cookie", &format!("session={session}"))
        .call()
        .expect("downloading input failed!")
        .into_string()
        .expect("parsing downloaded input failed!")
}

#[derive(Clone)]
struct Input {
    lines: Vec<String>,
    result: Option<String>,
}

impl From<TestInput> for Input {
    fn from(TestInput { input, result }: TestInput) -> Self {
        let mut lines = input.lines().collect::<Vec<_>>();

        if lines.first().is_some_and(|l| l.trim().is_empty()) {
            lines.remove(0);
        }

        if lines.last().is_some_and(|l| l.trim().is_empty()) {
            lines.pop();
        }

        let min_indent = lines
            .iter()
            .filter_map(|l| l.find(|c: char| !c.is_whitespace()))
            .min()
            .unwrap_or(0);

        let lines = lines
            .into_iter()
            .map(|l| {
                if min_indent < l.len() {
                    l.split_at(min_indent).1
                } else {
                    l
                }
            })
            .map(|l| l.to_string())
            .collect();

        let result = Some(result.to_string());

        Self { lines, result }
    }
}

fn test_inputs<Day: crate::Day>() -> (Input, Input) {
    let (p1, p2) = Day::test_inputs();
    (p1.into(), p2.into())
}

fn real_inputs<Day: crate::Day>() -> (Input, Input) {
    let lines = get_day_input::<Day>()
        .lines()
        .map(|l| l.to_string())
        .collect();

    let input = Input {
        lines,
        result: None,
    };

    (input.clone(), input)
}

fn run_part(Input { lines, result }: Input, f: impl Fn(&[String]) -> String) {
    let start = Instant::now();
    let part_result = f(&lines[..]);
    let elapsed = Instant::now() - start;

    println!("Took {elapsed:?}");
    println!("Result: {part_result}");

    let Some(result) = result else {
        return
    };

    if result != part_result {
        println!("ERROR! Expected result '{result}' but got '{part_result}'")
    } else {
        println!("Test passed!")
    }
}

fn run_day<Day: crate::Day>(args: &Args) {
    println!("\n-------------------------------\n");
    println!("Running day {}...\n", Day::date().1);

    let (input1, input2) = if args.test {
        println!("Using TEST input");
        test_inputs::<Day>()
    } else {
        real_inputs::<Day>()
    };

    println!("Running part1...");
    run_part(input1, Day::part1);

    println!("Running part2...");
    run_part(input2, Day::part2);
}
