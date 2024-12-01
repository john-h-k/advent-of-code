use std::str;

use crate::{Day, TestInput};

fn possible_combinations(mut line: &[u8], counts: &[usize]) -> usize {
    if line.is_empty() {
        return 0;
    }

    let mut sec;
    let mut rest;

    let Some(&count) = counts.first() else {
        return 1;
    };

    loop {
        while let Some(b'.') = line.first() {
            line = &line[1..];
        }

        let Some(first) = line.get(..count) else {
            return 0;
        };

        sec = first;
        rest = &line[count..];

        if !sec.contains(&b'.') {
            break;
        }

        while let Some(b'?') = line.first() {
            line = &line[1..];
        }
    }

    let q_count = sec.iter().filter(|&&b| b == b'?').count();

    let combos = if q_count == 0 {
        1
    } else {
        (2usize).pow((q_count - 1) as u32)
    };

    dbg!(
        str::from_utf8(sec).unwrap(),
        combos,
        str::from_utf8(rest).unwrap()
    );
    println!();
    combos * possible_combinations(rest, &counts[1..])
}

pub struct Day12;
impl Day for Day12 {
    fn date() -> (i32, i32) {
        (2023, 12)
    }

    fn part1(lines: &[String]) -> String {
        let result: usize = lines
            .iter()
            .map(|line| {
                let (mut screws, counts) = line.split_once(' ').unwrap();

                while let Some(b'.') = screws.as_bytes().first() {
                    screws = &screws[1..];
                }

                let mut counts = counts
                    .split(',')
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();

                dbg!(possible_combinations(screws.as_bytes(), &counts[..]))
            })
            .sum();

        format!("{result}")
    }

    fn part2(lines: &[String]) -> String {
        todo!()
    }

    fn test_inputs() -> (TestInput, TestInput) {
        (
            TestInput {
                input: "
                     .??..??...?##. 1,1,3
                ",
                //    ???.### 1,1,3
                //     .??..??...?##. 1,1,3
                //     ?#?#?#?#?#?#?#? 1,3,1,6
                //     ????.#...#... 4,1,1
                //     ????.######..#####. 1,6,5
                //     ?###???????? 3,2,1
                // ",
                result: "21",
            },
            TestInput {
                input: "
                    ???.### 1,1,3
                    .??..??...?##. 1,1,3
                    ?#?#?#?#?#?#?#? 1,3,1,6
                    ????.#...#... 4,1,1
                    ????.######..#####. 1,6,5
                    ?###???????? 3,2,1
                ",
                result: "21",
            },
        )
    }
}
