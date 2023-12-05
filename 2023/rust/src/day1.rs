use std::{iter::Rev, ops::Range};

use crate::{Day, TestInput};

fn first_digit(line: &[u8], mut indices: impl Iterator<Item = usize>) -> Option<i32> {
    indices.find_map(|s| {
        line[s]
            .is_ascii_digit()
            .then(|| (line[s] as char).to_digit(10).unwrap() as i32)
    })
}

fn first_digit_or_word(line: &[u8], mut indices: impl Iterator<Item = usize>) -> Option<i32> {
    const WORD_MAP: [(&str, i32); 18] = [
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ];

    indices.find_map(|s| {
        WORD_MAP
            .iter()
            .find_map(|(word, value)| line[s..].starts_with(word.as_bytes()).then_some(*value))
    })
}

fn find_digits<F, B>(lines: &[String], forward: F, backward: B) -> i32
where
    F: Fn(&[u8], Range<usize>) -> Option<i32>,
    B: Fn(&[u8], Rev<Range<usize>>) -> Option<i32>,
{
    lines
        .iter()
        .map(String::as_bytes)
        .map(|l| {
            let range = 0..l.len();
            let first = forward(l, range.clone());
            let last = backward(l, range.rev());

            first.unwrap() * 10 + last.unwrap()
        })
        .sum()
}

pub struct Day1;
impl Day for Day1 {
    fn date() -> (i32, i32) {
        (2023, 1)
    }

    fn part1(lines: &[String]) -> String {
        format!("{}", find_digits(lines, first_digit, first_digit))
    }

    fn part2(lines: &[String]) -> String {
        format!(
            "{}",
            find_digits(lines, first_digit_or_word, first_digit_or_word)
        )
    }

    fn test_inputs() -> (TestInput, TestInput) {
        return (
            TestInput {
                input: "
                    1abc2
                    pqr3stu8vwx
                    a1b2c3d4e5f
                    treb7uchet
                ",
                result: "142",
            },
            TestInput {
                input: "
                    two1nine
                    eightwothree
                    abcone2threexyz
                    xtwone3four
                    4nineeightseven2
                    zoneight234
                    7pqrstsixteen
                ",
                result: "281",
            },
        );
    }
}
