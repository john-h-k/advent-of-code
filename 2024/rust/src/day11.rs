use std::collections::HashSet;

use crate::{Day, TestInput};

pub struct Day11;
impl Day for Day11 {
    fn date() -> (i32, i32) {
        (2023, 11)
    }

    fn part1(lines: &[String]) -> String {
        let lines = lines
            .iter()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<_>>();

        let mut empty_rows = vec![false; lines.len()];
        let mut empty_cols = vec![false; lines[0].len()];

        let mut galaxies = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            let indices = line
                .iter()
                .enumerate()
                .filter_map(|(i, &b)| (b == b'#').then_some(i))
                .collect::<Vec<_>>();

            if !indices.is_empty() {
                indices.iter().for_each(|&index| galaxies.push((index, i)));
            } else {
                empty_rows[i] = true;
            }
        }

        for (i, el) in empty_cols.iter_mut().enumerate() {
            if !lines.iter().map(|l| l[i]).any(|l| l == b'#') {
                *el = true;
            }
        }

        let mut total_distance = 0;

        let mut seen = HashSet::new();

        for &start in galaxies.iter() {
            for &end in galaxies.iter() {
                if start == end {
                    continue;
                }

                if seen.contains(&(start, end)) || seen.contains(&(end, start)) {
                    continue;
                }

                let mut horiz = 0;
                for x in start.0.min(end.0)..start.0.max(end.0) {
                    if empty_cols[x] {
                        horiz += 2;
                    } else {
                        horiz += 1;
                    }
                }

                let mut vert = 0;
                for y in start.1.min(end.1)..start.1.max(end.1) {
                    if empty_rows[y] {
                        vert += 2;
                    } else {
                        vert += 1;
                    }
                }

                total_distance += horiz + vert;

                seen.insert((start, end));
                seen.insert((end, start));
            }
        }

        format!("{total_distance}")
    }

    fn part2(lines: &[String]) -> String {
        let lines = lines
            .iter()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<_>>();

        let mut empty_rows = vec![false; lines.len()];
        let mut empty_cols = vec![false; lines[0].len()];

        let mut galaxies = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            let indices = line
                .iter()
                .enumerate()
                .filter_map(|(i, &b)| (b == b'#').then_some(i))
                .collect::<Vec<_>>();

            if !indices.is_empty() {
                indices.iter().for_each(|&index| galaxies.push((index, i)));
            } else {
                empty_rows[i] = true;
            }
        }

        for (i, el) in empty_cols.iter_mut().enumerate() {
            if !lines.iter().map(|l| l[i]).any(|l| l == b'#') {
                *el = true;
            }
        }

        let mut total_distance = 0usize;

        let mut seen = HashSet::new();

        for &start in galaxies.iter() {
            for &end in galaxies.iter() {
                if start == end {
                    continue;
                }

                if seen.contains(&(start, end)) || seen.contains(&(end, start)) {
                    continue;
                }

                let mut horiz = 0;
                for x in start.0.min(end.0)..start.0.max(end.0) {
                    if empty_cols[x] {
                        horiz += 1000000;
                    } else {
                        horiz += 1;
                    }
                }

                let mut vert = 0;
                for y in start.1.min(end.1)..start.1.max(end.1) {
                    if empty_rows[y] {
                        vert += 1000000;
                    } else {
                        vert += 1;
                    }
                }

                total_distance += horiz + vert;

                seen.insert((start, end));
                seen.insert((end, start));
            }
        }

        format!("{total_distance}")
    }

    fn test_inputs() -> (TestInput, TestInput) {
        (
            TestInput {
                input: "
                    ...#......
                    .......#..
                    #.........
                    ..........
                    ......#...
                    .#........
                    .........#
                    ..........
                    .......#..
                    #...#.....
                ",
                result: "374",
            },
            TestInput {
                input: "
                    ...#......
                    .......#..
                    #.........
                    ..........
                    ......#...
                    .#........
                    .........#
                    ..........
                    .......#..
                    #...#.....
                ",
                result: "82000210",
            },
        )
    }
}
