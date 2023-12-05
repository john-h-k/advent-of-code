use std::{
    collections::{BTreeMap, HashMap, HashSet},
    mem,
    ops::{Bound, Range},
};

use itertools::{chain, Itertools};

use crate::{Day, TestInput};

pub struct Day5;

type SeedMap = BTreeMap<isize, (Range<isize>, Range<isize>)>;

fn build_map(lines: &mut &[String]) -> Option<SeedMap> {
    if lines.is_empty() {
        return None;
    }

    assert!(lines[0].contains("map"));
    *lines = &lines[1..];

    let map = lines
        .iter()
        .take_while(|l| l.chars().next().is_some_and(|c| c.is_digit(10)))
        .map(|l| {
            let [dest, src, len] = l.split(' ').map(|s| s.parse().unwrap()).collect::<Vec<_>>()[..] else {
                panic!("bad input!");
            };

            (src, (src..(src + len), dest..(dest + len)))
        })
        .collect::<SeedMap>();

    *lines = &lines[map.len()..];

    while !lines.is_empty() && lines[0].is_empty() {
        *lines = &lines[1..];
    }

    Some(map)
}

fn location_for_seed(seed: isize, maps: &[SeedMap]) -> isize {
    maps.iter().fold(seed, |acc, map| {
        let upper = map.upper_bound(Bound::Included(&acc));

        if let Some((src_range, dest_range)) = upper.value() && src_range.contains(&acc) {
            let diff = acc - src_range.start;
            dest_range.start + diff
        } else {
            acc
        }
    })
}

// fn location_for_seed_with_range(input: Range<isize>, map: &SeedMap) -> Vec<(Range<isize>, isize)> {
//     let Range { start, end } = input;

//     let mut lo = map.upper_bound(Bound::Included(&start));
//     let hi = map.upper_bound(Bound::Excluded(&end));

//     let mut ranges = Vec::new();

//     while lo.key() != hi.key() {
//         let (src_range, dest_range) = lo.key_value().unwrap();
//         ranges.push();

//         lo = lo.move_next();
//     }

//     ranges
// }

impl Day for Day5 {
    fn date() -> (i32, i32) {
        (2023, 5)
    }

    fn part1(lines: &[String]) -> String {
        let (_, seeds) = lines[0].split_once(':').unwrap();
        let seeds = seeds.split(' ').filter_map(|s| s.parse::<isize>().ok());

        let mut head = 2;

        let mut maps = Vec::new();

        let mut lines = &lines[2..];

        while let Some(map) = build_map(&mut lines) {
            maps.push(map);
        }

        let result = seeds
            .map(|seed| location_for_seed(seed, &maps))
            .min()
            .unwrap();

        format!("{result}")
    }

    fn part2(lines: &[String]) -> String {
        let (_, seeds) = lines[0].split_once(':').unwrap();
        let seeds = seeds
            .split(' ')
            .filter_map(|s| s.parse::<isize>().ok())
            .collect::<Vec<_>>();

        let mut head = 2;

        let mut maps = Vec::new();

        let mut lines = &lines[2..];

        while let Some(map) = build_map(&mut lines) {
            maps.push(map);
        }

        let seed_ranges = seeds
            .iter()
            .tuples()
            .map(|(&first, &len)| first..(first + len));

        let result = seed_ranges
            .flat_map(|range| range.map(|seed| (seed, location_for_seed(seed, &maps))))
            .map(|(seed, loc)| loc)
            .min()
            .unwrap();

        format!("{result}")
    }

    fn test_inputs() -> (TestInput, TestInput) {
        return (
            TestInput {
                input: "
                    seeds: 79 14 55 13

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
                    56 93 4
                    ",
                result: "35",
            },
            TestInput {
                input: "
                    seeds: 79 14 55 13

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
                    56 93 4
                ",
                result: "46",
            },
        );
    }
}
