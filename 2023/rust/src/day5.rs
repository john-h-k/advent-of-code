use std::{
    collections::{BTreeMap, HashMap, HashSet},
    mem,
    ops::{Bound, Range},
};

use itertools::{chain, Itertools};

use crate::Day;

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

        // maps.reverse();
        // for map in maps.iter_mut() {
        //     *map = map
        //         .iter()
        //         .map(|(src, (src_range, dest_range))| {
        //             (dest_range.start, (dest_range.clone(), src_range.clone()))
        //         })
        //         .collect();
        // }

        // let seed_map = seeds
        //     .iter()
        //     .tuples()
        //     .map(|(&first, &len)| (first, first..(first + len)))
        //     .collect::<BTreeMap<_, _>>();
        let seed_ranges = seeds
            .iter()
            .tuples()
            .map(|(&first, &len)| first..(first + len));

        // let (&min_loc, (dest, len)) = maps[0].first_key_value().unwrap();
        // maps[0].insert(0, (0..min_loc, 0..min_loc));

        // let result = maps[0]
        //     .iter()
        //     .flat_map(|(_, (src_range, dest_range))| {
        //         src_range.clone().map(|loc| {
        //             println!("testing loc {loc}");
        //             let a = (loc, location_for_seed(loc, &maps));
        //             println!("had seed {}", a.1);
        //             a
        //         })
        //     })
        //     .filter(|(loc, seed)| {
        //         seed_map
        //             .upper_bound(Bound::Included(&seed))
        //             .value()
        //             .is_some_and(|v| v.contains(&seed))
        //     })
        //     .map(|(loc, seed)| loc)
        //     .min()
        //     .unwrap();

        let result = seed_ranges
            .flat_map(|range| {
                range.map(|seed| {
                    // println!("testing seed {seed}");
                    let a = (seed, location_for_seed(seed, &maps));
                    // println!("had loc {}", a.1);
                    a
                })
            })
            .map(|(seed, loc)| loc)
            .min()
            .unwrap();

        format!("{result}")
    }
}
