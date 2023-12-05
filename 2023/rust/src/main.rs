#![feature(btree_cursors)]
#![feature(let_chains)]

use std::{env, error::Error};

use chrono::{self, Datelike, NaiveDate};

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

trait Day {
    fn part1(lines: &[String]) -> String;
    fn part2(lines: &[String]) -> String;
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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

    let input = if false {
        get_test_input()
    } else {
        get_day_input(year, day)
    };

    let input = input.lines().map(|l| l.to_string()).collect::<Vec<_>>();

    match day {
        1 => run_day::<Day1>(&input),
        2 => run_day::<Day2>(&input),
        3 => run_day::<Day3>(&input),
        4 => run_day::<Day4>(&input),
        5 => run_day::<Day5>(&input),
        day => todo!("day {day} not implemented"),
    }
}

fn get_test_input() -> String {
    "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
        .into()
}

fn get_day_input(year: i32, day: i32) -> String {
    let session = env::var("AOC_SESSION")
        .expect("must set `AOC_SESSION` env var to your advent of code session cookie");

    ureq::get(&format!("https://adventofcode.com/{year}/day/{day}/input"))
        .set("Cookie", &format!("session={session}"))
        .call()
        .expect("downloading input failed!")
        .into_string()
        .expect("parsing downloaded input failed!")
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
