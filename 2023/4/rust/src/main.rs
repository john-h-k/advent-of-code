use std::{
    collections::{HashMap, HashSet},
    fs, str,
};

fn main() {
    let lines: Vec<String> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    part1(&lines);
    part2(&lines);
}

fn parse_numbers(line: &str) -> HashSet<i32> {
    line.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(lines: &[String]) {
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

    println!("{result}");
}

fn part2(lines: &[String]) {
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

    println!("{result}");
}
