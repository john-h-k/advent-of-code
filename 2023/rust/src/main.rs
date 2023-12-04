use std::{env, error::Error};

use chrono::{self, Datelike, NaiveDate};

use crate::{day1::Day1, day2::Day2, day3::Day3, day4::Day4};

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

trait Day {
    fn part1(lines: &[String]) -> String;
    fn part2(lines: &[String]) -> String;
}

mod day1;
mod day2;
mod day3;
mod day4;

fn day_str(day: i32) -> String {
    let suffix = match day % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };

    format!("{day}{suffix}")
}

fn main() {
    let args = env::args();

    let (year, mut day) = year_and_day();
    println!("Today is the {} of December {year}\n", day_str(day));

    if args.len() > 1 {
        let day_arg = args.last().unwrap();
        day = day_arg
            .parse()
            .unwrap_or_else(|_| panic!("invalid day '{}'", day_arg));

        println!("Running day {day}...\n");
    };

    assert!(matches!(day, 0..=25), "invalid date");

    let input = get_day_input(year, day)
        .expect("downloading input failed!")
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<_>>();

    match day {
        1 => run_day::<Day1>(&input),
        2 => run_day::<Day2>(&input),
        3 => run_day::<Day3>(&input),
        4 => run_day::<Day4>(&input),
        day => todo!("day {day} not implemented"),
    }
}

fn get_day_input(year: i32, day: i32) -> Result<String, Box<dyn Error>> {
    let session = env::var("AOC_SESSION")
        .expect("must set `AOC_SESSION` env var to your advent of code session cookie");

    let input = ureq::get(&format!("https://adventofcode.com/{year}/day/{day}/input"))
        .set("Cookie", &format!("session={session}"))
        .call()?
        .into_string()?;

    Ok(input)
}

fn run_day<TDay: Day>(lines: &[String]) {
    println!("Running part1...");
    let part1_result = TDay::part1(lines);
    println!("Part 1 result: \n{part1_result}");

    println!("\n-------------------------------");

    println!("Running part2...");
    let part2_result = TDay::part2(lines);
    println!("Part 2 result: \n{part2_result}")
}
