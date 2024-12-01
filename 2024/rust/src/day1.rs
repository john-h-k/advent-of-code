use std::collections::HashMap;

use crate::{Day, TestInput};

pub struct Day1;

impl Day for Day1 {
    fn date() -> (i32, i32) {
        (2024, 1)
    }

    fn part1(lines: &[String]) -> String {
        // parse the left and right lists
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in lines {
            let mut parts = line.split_whitespace();
            while let (Some(left_num), Some(right_num)) = (parts.next(), parts.next()) {
                left.push(left_num.parse::<i32>().unwrap());
                right.push(right_num.parse::<i32>().unwrap());
            }
        }

        // sort both lists
        left.sort();
        right.sort();

        // compute the total distance
        let total_distance: i32 = left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| (l - r).abs())
            .sum();

        total_distance.to_string()
    }

    fn part2(lines: &[String]) -> String {
        // parse the left and right lists
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in lines {
            let mut parts = line.split_whitespace();
            while let (Some(left_num), Some(right_num)) = (parts.next(), parts.next()) {
                left.push(left_num.parse::<i32>().unwrap());
                right.push(right_num.parse::<i32>().unwrap());
            }
        }

        // create a frequency map for the right list
        let mut right_freq = HashMap::new();
        for num in right {
            *right_freq.entry(num).or_insert(0) += 1;
        }

        // calculate the similarity score
        let similarity_score: i32 = left
            .iter()
            .map(|&num| num * right_freq.get(&num).unwrap_or(&0))
            .sum();

        similarity_score.to_string()
    }

    fn test_inputs() -> (TestInput, TestInput) {
        (
            TestInput {
                input: "
                    3 4
                    4 3
                    2 5
                    1 3
                    3 9
                    3 3
                ",
                result: "11",
            },
            TestInput {
                input: "
                    3 4
                    4 3
                    2 5
                    1 3
                    3 9
                    3 3
                ",
                result: "31",
            },
        )
    }
}
