use crate::{Day, TestInput};

fn diff(nums: &[isize]) -> isize {
    if nums.iter().all(|&d| d == 0) {
        return 0;
    }

    let diffs = (1..nums.len())
        .map(|i| nums[i] - nums[i - 1])
        .collect::<Vec<_>>();

    nums.last().unwrap() + diff(&diffs)
}

pub struct Day9;
impl Day for Day9 {
    fn date() -> (i32, i32) {
        (2023, 9)
    }

    fn part1(lines: &[String]) -> String {
        let result: isize = lines
            .iter()
            .map(|line| {
                let nums = line
                    .split(' ')
                    .map(|l| l.parse::<isize>().unwrap())
                    .collect::<Vec<_>>();

                diff(&nums)
            })
            .sum();

        format!("{result}")
    }

    fn part2(lines: &[String]) -> String {
        let result: isize = lines
            .iter()
            .map(|line| {
                let nums = line
                    .split(' ')
                    .map(|l| l.parse::<isize>().unwrap())
                    .rev()
                    .collect::<Vec<_>>();

                diff(&nums)
            })
            .sum();

        format!("{result}")
    }

    fn test_inputs() -> (crate::TestInput, crate::TestInput) {
        (
            TestInput {
                input: "
                    0 3 6 9 12 15
                    1 3 6 10 15 21
                    10 13 16 21 30 45
                ",
                result: "114",
            },
            TestInput {
                input: "
                    0 3 6 9 12 15
                    1 3 6 10 15 21
                    10 13 16 21 30 45
                ",
                result: "2",
            },
        )
    }
}
