use crate::Day;

use std::{collections::HashMap, str};

fn is_symbol(c: u8) -> bool {
    !(c.is_ascii_digit() || c == b'.')
}

fn touches_symbol(lines: &[String], i: usize, j: usize, last: usize) -> Option<(u8, usize, usize)> {
    for k in j..(j + last) {
        for dx in [-1isize, 0, 1] {
            for dy in [-1isize, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let y = i as isize + dy;
                let x = k as isize + dx;
                if y >= 0
                    && (y as usize) < lines.len()
                    && x >= 0
                    && (x as usize) < lines[0].len()
                    && is_symbol(lines[y as usize].as_bytes()[x as usize])
                {
                    return Some((
                        lines[y as usize].as_bytes()[x as usize],
                        x as usize,
                        y as usize,
                    ));
                }
            }
        }
    }

    None
}

pub struct Day3;
impl Day for Day3 {
    fn part1(lines: &[String]) -> String {
        let mut val = 0;

        let mut skip = false;

        for (i, line) in lines.iter().map(String::as_bytes).enumerate() {
            for (j, &c) in line.iter().enumerate() {
                if !skip && c.is_ascii_digit() {
                    skip = true;
                    let last = line[j..]
                        .iter()
                        .position(|c| !u8::is_ascii_digit(c))
                        .unwrap_or(line.len() - j);

                    let value: u32 = str::from_utf8(&line[j..j + last]).unwrap().parse().unwrap();

                    if touches_symbol(lines, i, j, last).is_some() {
                        val += value;
                    }
                } else if !c.is_ascii_digit() {
                    skip = false;
                }
            }
        }

        format!("{val}")
    }

    fn part2(lines: &[String]) -> String {
        let mut val = 0;

        let mut skip = false;

        let mut gears = HashMap::new();

        for (i, line) in lines.iter().map(String::as_bytes).enumerate() {
            for (j, &c) in line.iter().enumerate() {
                if !skip && c.is_ascii_digit() {
                    skip = true;
                    let last = line[j..]
                        .iter()
                        .position(|c| !u8::is_ascii_digit(c))
                        .unwrap_or(line.len() - j);

                    let value: u32 = str::from_utf8(&line[j..j + last]).unwrap().parse().unwrap();

                    if let Some((b'*', x, y)) = touches_symbol(lines, i, j, last) {
                        gears.entry((x, y)).or_insert(Vec::new()).push(value);
                    }
                } else if !c.is_ascii_digit() {
                    skip = false;
                }
            }
        }

        for gear in gears.values() {
            if gear.len() == 2 {
                val += gear[0] * gear[1];
            }
        }

        format!("{val}")
    }
}
