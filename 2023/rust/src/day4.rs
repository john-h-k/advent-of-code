use std::{collections::HashSet, str};

use crate::{Day, TestInput};

fn parse_numbers(line: &str) -> HashSet<i32> {
    line.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

pub struct Day4;
impl Day for Day4 {
    fn date() -> (i32, i32) {
        (2023, 4)
    }

    fn part1(lines: &[String]) -> String {
        let result: u32 = lines
            .iter()
            .map(|l| {
                let (winners, numbers) = l
                    .split_once(':')
                    .and_then(|(_, line)| line.split_once('|'))
                    .unwrap();

                let winners = parse_numbers(winners);
                let numbers = parse_numbers(numbers);

                let num_winners = winners.intersection(&numbers).count();

                if num_winners > 0 {
                    1 << (num_winners - 1)
                } else {
                    0
                }
            })
            .sum();

        format!("{result}")
    }

    fn part2(lines: &[String]) -> String {
        let result: usize = lines
            .iter()
            .map(|l| {
                let (winners, numbers) = l
                    .split_once(':')
                    .and_then(|(_, line)| line.split_once('|'))
                    .unwrap();

                let winners = parse_numbers(winners);
                let numbers = parse_numbers(numbers);

                winners.intersection(&numbers).count()
            })
            .enumerate()
            .fold(vec![1; lines.len()], |mut acc, (i, wins)| {
                for j in i..(i + wins) {
                    acc[j + 1] += acc[i];
                }

                acc
            })
            .into_iter()
            .sum();

        format!("{result}")
    }

    fn test_inputs() -> (TestInput, TestInput) {
        return (
            TestInput {
                input: "
                    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
                ",
                result: "13",
            },
            TestInput {
                input: "
                    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
                ",
                result: "30",
            },
        );
    }
}
