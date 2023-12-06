use std::ops;

use crate::{Day, TestInput};

fn quad_solve(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discrim = (b * b) - (4.0 * a * c);

    if discrim < 0.0 {
        return None;
    }

    let discrim = discrim.sqrt();

    let plus = (-b + discrim) / (2.0 * a);
    let minus = (-b - discrim) / (2.0 * a);

    Some((plus, minus))
}

fn better_possible_win_count(time: usize, distance: usize) -> usize {
    match quad_solve(-1.0, time as f64, -(distance as f64)) {
        None => 0,
        Some((l, r)) => {
            let (l, r) = (l.min(r), l.max(r));

            let count = (r.floor() - l.ceil()) as usize;

            if l.fract() == 0.0 && r.fract() == 0.0 {
                count - 1
            } else {
                count + 1
            }
        }
    }
}

fn possible_win_count(time: usize, distance: usize) -> usize {
    let possibilities = (1..(time / 2 + 1))
        .filter(|t| t * (time - t) > distance)
        .count()
        * 2;

    if time % 2 == 0 {
        possibilities - 1
    } else {
        possibilities
    }
}

pub struct Day6;
impl Day for Day6 {
    fn date() -> (i32, i32) {
        (2023, 6)
    }

    fn part1(lines: &[String]) -> String {
        let times = lines[0].split_once(':').unwrap().1.split_whitespace();
        let distances = lines[1].split_once(':').unwrap().1.split_whitespace();

        let result = times
            .zip(distances)
            .filter_map(|(t, d)| Some(possible_win_count(t.parse().ok()?, d.parse().ok()?)))
            .fold(1, ops::Mul::mul);

        format!("{result}")
    }

    fn part2(lines: &[String]) -> String {
        let time = lines[0]
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .collect::<String>()
            .parse()
            .unwrap();

        let distance = lines[1]
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .collect::<String>()
            .parse()
            .unwrap();

        let result = better_possible_win_count(time, distance);

        format!("{result}")
    }

    fn test_inputs() -> (TestInput, TestInput) {
        (
            TestInput {
                input: "
                Time:      7  15   30
                Distance:  9  40  200
            ",
                result: "288",
            },
            TestInput {
                input: "
                Time:      7  15   30
                Distance:  9  40  200
            ",
                result: "71503",
            },
        )
    }
}
