use std::collections::{HashMap, VecDeque};

use crate::{Day, TestInput};

fn get_start_and_real_s_char(lines: &[String]) -> (u8, (isize, isize)) {
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            if c == b'S' {
                let start = (x as isize, y as isize);

                let get_char = |dx: isize, dy: isize| {
                    y.checked_add_signed(dy).and_then(|y| {
                        x.checked_add_signed(dx)
                            .and_then(|x| lines.get(y).and_then(|l| l.as_bytes().get(x)))
                    })
                };

                let conn_top = matches!(get_char(0, -1), Some(b'|' | b'F' | b'7'));
                let conn_bottom = matches!(get_char(0, 1), Some(b'|' | b'L' | b'J'));
                let conn_left = matches!(get_char(-1, 0), Some(b'-' | b'F' | b'L'));
                let conn_right = matches!(get_char(1, 0), Some(b'-' | b'J' | b'7'));

                let char = if conn_top && conn_bottom {
                    b'|'
                } else if conn_top && conn_left {
                    b'J'
                } else if conn_top && conn_right {
                    b'L'
                } else if conn_bottom && conn_left {
                    b'7'
                } else if conn_bottom && conn_right {
                    b'F'
                } else {
                    unreachable!()
                };

                return (char, start);
            }
        }
    }

    unreachable!()
}

pub struct Day10;
impl Day for Day10 {
    fn date() -> (i32, i32) {
        (2023, 10)
    }

    fn part1(lines: &[String]) -> String {
        let (real_s_char, start) = get_start_and_real_s_char(lines);

        let mut queue = VecDeque::new();
        queue.push_back((0isize, start));

        let mut visited = HashMap::new();

        let mut max_distance = 0;

        while let Some((distance, (x, y))) = queue.pop_front() {
            let Some(&pipe) = lines.get(y as usize).and_then(|l| l.as_bytes().get(x as usize)) else {
                continue;
            };

            if pipe == b'.' {
                continue;
            }

            let entry = visited.entry((x, y)).or_insert(false);
            if *entry {
                continue;
            }

            *entry = true;

            max_distance = max_distance.max(distance);

            let c = if pipe == b'S' { real_s_char } else { pipe };

            let visit = match c {
                b'|' => vec![(x, y - 1), (x, y + 1)],
                b'-' => vec![(x - 1, y), (x + 1, y)],
                b'7' => vec![(x - 1, y), (x, y + 1)],
                b'J' => vec![(x - 1, y), (x, y - 1)],
                b'F' => vec![(x + 1, y), (x, y + 1)],
                b'L' => vec![(x + 1, y), (x, y - 1)],
                b'S' => unreachable!(),
                _ => vec![],
            };

            queue.extend(visit.iter().map(|&v| (distance + 1, v)));
        }

        format!("{max_distance}")
    }

    fn part2(lines: &[String]) -> String {
        let (real_s_char, start) = get_start_and_real_s_char(lines);

        let mut queue = VecDeque::new();
        queue.push_back((0isize, start));

        let mut visited = HashMap::new();

        while let Some((distance, (x, y))) = queue.pop_front() {
            let Some(&pipe) = lines.get(y as usize).and_then(|l| l.as_bytes().get(x as usize)) else {
                continue;
            };

            if pipe == b'.' {
                continue;
            }

            let entry = visited.entry((x, y)).or_insert(false);
            if *entry {
                continue;
            }

            *entry = true;

            let c = if pipe == b'S' { real_s_char } else { pipe };

            let visit = match c {
                b'|' => vec![(x, y - 1), (x, y + 1)],
                b'-' => vec![(x - 1, y), (x + 1, y)],
                b'7' => vec![(x - 1, y), (x, y + 1)],
                b'J' => vec![(x - 1, y), (x, y - 1)],
                b'F' => vec![(x + 1, y), (x, y + 1)],
                b'L' => vec![(x + 1, y), (x, y - 1)],
                b'S' => unreachable!(),
                _ => vec![],
            };

            queue.extend(visit.iter().map(|&v| (distance + 1, v)));
        }

        let lines = lines
            .iter()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<_>>();

        let mut parities = vec![vec![(false, 0, 0); lines[0].len()]; lines.len()];

        for y in 0..lines.len() {
            for x in 0..lines[0].len() {
                let (_, mut prev_left, _) = x
                    .checked_sub(1)
                    .map(|x| parities[y][x])
                    .unwrap_or((false, 0, 0));

                let (_, _, mut prev_above) = y
                    .checked_sub(1)
                    .map(|y| parities[y][x])
                    .unwrap_or((false, 0, 0));

                let mut is_loop = false;
                if visited.contains_key(&(x as isize, y as isize)) {
                    is_loop = true;
                    if matches!(lines[y][x], b'-' | b'J' | b'L') {
                        prev_above += 1;
                    } else if matches!(lines[y][x], b'|' | b'7' | b'J') {
                        prev_left += 1;
                    }
                }

                parities[y][x] = (is_loop, prev_left, prev_above);
            }
        }

        dbg!(&parities);

        let mut result = 0;
        for y in 0..lines.len() {
            let max_left = parities[y].last().unwrap().1;
            for x in 0..lines[0].len() {
                let max_above = parities.last().unwrap()[x].2;
                let (is_loop, prev_left, prev_above) = parities[y][x];

                if !is_loop
                    && ((prev_left % 2 == 1 && prev_left < max_left)
                        && (prev_above % 2 == 1 && prev_above < max_above))
                {
                    println!("space at {x}, {y}");
                    result += 1;
                }
            }
        }

        format!("{result}")
    }

    fn test_inputs() -> (TestInput, TestInput) {
        (
            TestInput {
                input: "
                    ..F7.
                    .FJ|.
                    SJ.L7
                    |F--J
                    LJ...
                ",
                result: "8",
            },
            TestInput {
                // input: "
                //     ...........
                //     .S-------7.
                //     .|F-----7|.
                //     .||.....||.
                //     .||.....||.
                //     .|L-7.F-J|.
                //     .|..|.|..|.
                //     .L--J.L--J.
                //     ...........
                // ",
                // result: "4",
                // input: "
                //     .F----7F7F7F7F-7....
                //     .|F--7||||||||FJ....
                //     .||.FJ||||||||L7....
                //     FJL7L7LJLJ||LJ.L-7..
                //     L--J.L7...LJS7F-7L7.
                //     ....F-J..F7FJ|L7L7L7
                //     ....L7.F7||L7|.L7L7|
                //     .....|FJLJ|FJ|F7|.LJ
                //     ....FJL-7.||.||||...
                //     ....L---J.LJ.LJLJ...
                // ",
                // result: "8",
                input: "
                    FF7FSF7F7F7F7F7F---7
                    L|LJ||||||||||||F--J
                    FL-7LJLJ||||||LJL-77
                    F--JF--7||LJLJ7F7FJ-
                    L---JF-JLJ.||-FJLJJ7
                    |F|F-JF---7F7-L7L|7|
                    |FFJF7L7F-JF7|JL---7
                    7-L-JL7||F7|L7F-7F7|
                    L.L7LFJ|||||FJL7||LJ
                    L7JLJL-JLJLJL--JLJ.L
                ",
                result: "10",
            },
        )
    }
}
