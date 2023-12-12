use crate::{Day, TestInput};

fn possible_combinations(mut line: &[u8], mut counts: &mut [usize]) -> Option<usize> {
    if let Some(0) = counts.first() {
        let Some(b'.') = line.first() else {
            return None;
        };

        while let Some(b'.') = line.first() {
            line = &line[1..];
        }
    }

    if !counts.is_empty() && counts[0] == 0 {
        counts = &mut counts[1..];
    }

    if counts.is_empty() {
        return Some(1);
    }

    if line.is_empty() {
        return None;
    }

    match line[0] {
        b'.' => None,
        b'?' => {
            println!(
                "combos for {}",
                String::from_utf8(line[1..].to_vec()).unwrap()
            );
            let fixed = possible_combinations(&line[1..], counts);

            counts[0] -= 1;
            let broken = possible_combinations(&line[1..], counts);
            counts[0] += 1;
            println!("{} -- {}", fixed.unwrap_or(0), broken.unwrap_or(0));

            Some(fixed.unwrap_or(0) + broken.unwrap_or(0))
        }
        b'#' => {
            counts[0] -= 1;
            let result = possible_combinations(&line[1..], counts);
            counts[0] += 1;
            result
        }
        _ => unreachable!(),
    }
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

                dbg!(possible_combinations(screws.as_bytes(), &mut counts[..]).unwrap())
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
                    ???.### 1,1,3
                    .??..??...?##. 1,1,3
                    ?#?#?#?#?#?#?#? 1,3,1,6
                    ????.#...#... 4,1,1
                    ????.######..#####. 1,6,5
                    ?###???????? 3,2,1
                ",
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
