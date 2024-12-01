use std::collections::HashMap;

use crate::{Day, TestInput};

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / (gcd(a, b))
}

fn iter_lcm(iter: impl Iterator<Item = usize>) -> usize {
    iter.reduce(lcm).unwrap()
}

fn steps_until_z<'a>(
    nodes: impl Iterator<Item = &'a str> + 'a,
    paths: HashMap<&'a str, [&'a str; 2]>,
    dirs: Vec<usize>,
) -> impl Iterator<Item = usize> + 'a {
    nodes.map(move |node| {
        let mut dirs = dirs.iter().cycle();

        let mut result = 0;

        let mut node = node;
        while !node.ends_with('Z') {
            let dir = *dirs.next().unwrap();
            node = paths[node][dir];

            result += 1;
        }

        result
    })
}

pub struct Day8;
impl Day for Day8 {
    fn date() -> (i32, i32) {
        (2023, 8)
    }

    fn part1(lines: &[String]) -> String {
        let dirs = lines[0]
            .trim()
            .chars()
            .map(|c| match c {
                'L' => 0,
                'R' => 1,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        let paths = lines[2..]
            .iter()
            .map(|l| {
                let (from, to) = l.split_once('=').unwrap();
                let to = to.trim();
                let (left, right) = to[1..(to.len() - 1)].split_once(',').unwrap();

                (from.trim(), [left.trim(), right.trim()])
            })
            .collect::<HashMap<_, _>>();

        let mut steps = 0usize;
        let mut pos = "AAA";

        while pos != "ZZZ" {
            pos = paths[pos][dirs[steps % dirs.len()]];
            steps += 1;
        }

        format!("{steps}")
    }

    fn part2(lines: &[String]) -> String {
        let dirs = lines[0]
            .trim()
            .chars()
            .map(|c| match c {
                'L' => 0,
                'R' => 1,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        let paths = lines[2..]
            .iter()
            .map(|l| {
                let (from, to) = l.split_once('=').unwrap();
                let to = to.trim();
                let (left, right) = to[1..(to.len() - 1)].split_once(',').unwrap();

                (from.trim(), [left.trim(), right.trim()])
            })
            .collect::<HashMap<_, _>>();

        let nodes = paths
            .keys()
            .filter(|node| node.ends_with('A'))
            .cloned()
            .collect::<Vec<_>>();

        let steps = steps_until_z(nodes.into_iter(), paths, dirs);
        let result = iter_lcm(steps);

        format!("{result}")
    }

    fn test_inputs() -> (TestInput, TestInput) {
        (
            TestInput {
                input: "
                    RL

                    AAA = (BBB, CCC)
                    BBB = (DDD, EEE)
                    CCC = (ZZZ, GGG)
                    DDD = (DDD, DDD)
                    EEE = (EEE, EEE)
                    GGG = (GGG, GGG)
                    ZZZ = (ZZZ, ZZZ)
                ",
                result: "2",
            },
            TestInput {
                input: "
                    LR

                    11A = (11B, XXX)
                    11B = (XXX, 11Z)
                    11Z = (11B, XXX)
                    22A = (22B, XXX)
                    22B = (22C, 22C)
                    22C = (22Z, 22Z)
                    22Z = (22B, 22B)
                    XXX = (XXX, XXX)
                ",
                result: "6",
            },
        )
    }
}
