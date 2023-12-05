use std::{
    collections::BTreeMap,
    ops::{Bound, Range},
};

use itertools::Itertools;

use crate::{Day, TestInput};

pub struct Day5;

type SeedMap = BTreeMap<isize, (Range<isize>, Range<isize>)>;
type OrderedMappings = BTreeMap<usize, Mapping>;

fn build_map(lines: &mut &[String]) -> Option<SeedMap> {
    if lines.is_empty() {
        return None;
    }

    assert!(lines[0].contains("map"));
    *lines = &lines[1..];

    let map = lines
        .iter()
        .take_while(|l| l.chars().next().is_some_and(|c| c.is_ascii_digit()))
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

fn build_map_as_mappings(lines: &mut &[String]) -> Option<OrderedMappings> {
    if lines.is_empty() {
        return None;
    }

    assert!(lines[0].contains("map"));
    *lines = &lines[1..];

    let map = lines
        .iter()
        .take_while(|l| l.chars().next().is_some_and(|c| c.is_ascii_digit()))
        .map(|l| {
            let [end, start, len] = l.split(' ').map(|s| s.parse().unwrap()).collect::<Vec<_>>()[..] else {
                panic!("bad input!");
            };

            let shift = end as isize - start as isize;

            (start, Mapping { start, len, shift })
        })
        .collect::<OrderedMappings>();

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

#[derive(Debug)]
enum RangeOverlapType {
    Subset,
    Equal,
    Superset,
    EntirelyLess,
    OverlappingLess,
    OverlappingMore,
    EntirelyMore,
}

fn overlap_type(range: &Range<usize>, of: &Range<usize>) -> RangeOverlapType {
    if range.start == of.start && range.end == of.end {
        RangeOverlapType::Equal
    } else if range.start >= of.start && range.end <= of.end {
        RangeOverlapType::Subset
    } else if range.start <= of.start && range.end >= of.end {
        RangeOverlapType::Superset
    } else if range.end < of.start {
        RangeOverlapType::EntirelyLess
    } else if range.start <= of.start {
        RangeOverlapType::OverlappingLess
    } else if range.start <= of.end {
        RangeOverlapType::OverlappingMore
    } else {
        RangeOverlapType::EntirelyMore
    }
}

#[derive(Debug, Clone, Copy)]
struct Mapping {
    start: usize,
    len: usize,
    shift: isize,
}

impl Mapping {
    fn end(&self) -> usize {
        self.start + self.len
    }

    fn shifted_start(&self) -> usize {
        self.start
            .checked_add_signed(self.shift)
            .expect("overflow!")
    }

    fn shifted_end(&self) -> usize {
        self.shifted_start() + self.len
    }

    #[allow(clippy::wrong_self_convention)]
    fn from_range(self) -> Range<usize> {
        self.start..self.end()
    }

    fn to_range(self) -> Range<usize> {
        self.shifted_start()..self.shifted_end()
    }

    fn constrained_by(self, range: &Range<usize>) -> Option<Self> {
        match overlap_type(range, &self.from_range()) {
            RangeOverlapType::Subset => Some(Self {
                start: range.start,
                len: range.len(),
                shift: self.shift,
            }),
            RangeOverlapType::OverlappingLess => Some(Self {
                start: self.start,
                len: range.end - self.start,
                shift: self.shift,
            }),
            RangeOverlapType::OverlappingMore => Some(Self {
                start: range.start,
                len: self.end() - range.start,
                shift: self.shift,
            }),
            RangeOverlapType::Equal | RangeOverlapType::Superset => Some(self),
            RangeOverlapType::EntirelyLess | RangeOverlapType::EntirelyMore => None,
        }
    }
}

fn merge_ranges_to_map(ranges: Vec<Range<usize>>, second: OrderedMappings) -> Vec<Range<usize>> {
    ranges
        .into_iter()
        .flat_map(move |range| {
            let mut lo = second.upper_bound(Bound::Included(&range.start));
            let hi = second.lower_bound(Bound::Excluded(&range.end));

            let mut subranges = Vec::new();
            while lo.key() != hi.key() {
                let Some(next_mapping) = lo.value() else {
                    subranges.push(range.clone());
                    lo.move_next();
                    continue;
                };

                dbg!(&next_mapping, &range);
                dbg!(overlap_type(&range, &next_mapping.to_range()));
                let constrained = next_mapping.constrained_by(&range);

                if let Some(constrained) = constrained {
                    subranges.push(constrained.to_range());
                } else {
                    subranges.push(range.clone());
                }

                lo.move_next();
            }
            subranges
        })
        .collect()
}

impl Day for Day5 {
    fn date() -> (i32, i32) {
        (2023, 5)
    }

    fn part1(lines: &[String]) -> String {
        let (_, seeds) = lines[0].split_once(':').unwrap();
        let seeds = seeds.split(' ').filter_map(|s| s.parse::<isize>().ok());

        let _head = 2;

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
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();

        let mut lines = &lines[2..];

        let mut seed_ranges = seeds
            .iter()
            .tuples()
            .map(|(&first, &len)| first..(first + len))
            .collect();

        while let Some(map) = build_map_as_mappings(&mut lines) {
            seed_ranges = merge_ranges_to_map(seed_ranges, map);
        }

        let result = seed_ranges
            .into_iter()
            .map(|range| range.start)
            .min()
            .unwrap();

        format!("{result}")
    }

    fn test_inputs() -> (TestInput, TestInput) {
        (
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
        )
    }
}
