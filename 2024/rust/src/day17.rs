use std::{
    array,
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use itertools::Itertools;

use crate::{Day, TestInput};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Day17;
impl Day for Day17 {
    fn date() -> (i32, i32) {
        (2023, 17)
    }

    fn part1(lines: &[String]) -> String {
        let height = lines.len();
        let width = lines[0].len();

        let lines = lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect_vec()
            })
            .collect_vec();

        let mut distances = Vec::new();
        for _ in 0..(height * width) {
            distances.push([usize::MAX; 4]);
        }

        let mut q = BinaryHeap::new();

        q.push(Reverse((0usize, (0isize, 0isize), 0usize, 0usize, 0usize)));

        let mut result = usize::MAX;
        while let Some(Reverse((dist, dir, count, x, y))) = q.pop() {
            println!("visiting {x}, {y}");
            if y + 1 == height && x + 1 == width {
                result = dist;
                break;
            }

            for (i, (dx, dy)) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().enumerate() {
                if (-dx, -dy) == dir {
                    continue;
                }

                if count == 4 && (dx, dy) == dir {
                    continue;
                }

                let new_y = y.checked_add_signed(dy);
                let new_x = x.checked_add_signed(dx);

                let (new_x, new_y) = match (new_x, new_y) {
                    (Some(new_x), Some(new_y)) if new_y < height && new_x < width => (new_x, new_y),
                    _ => {
                        continue;
                    }
                };

                let count = if (dx, dy) == dir { count + 1 } else { 1 };

                let new_dist = dist.saturating_add(lines[new_y][new_x]);

                if new_dist < distances[new_y * width + new_x][i] {
                    q.push(Reverse((new_dist, (dx, dy), count, new_x, new_y)));
                    distances[new_y * width + new_x][i] = new_dist;
                }
            }
        }

        result += lines[height - 1][width - 1];

        format!("{result}")
    }

    fn part2(lines: &[String]) -> String {
        todo!()
    }

    fn test_inputs() -> (TestInput, TestInput) {
        (
            TestInput {
                input: "
                2413432311323
                3215453535623
                3255245654254
                3446585845452
                4546657867536
                1438598798454
                4457876987766
                3637877979653
                4654967986887
                4564679986453
                1224686865563
                2546548887735
                4322674655533
            ",
                result: "102",
            },
            TestInput {
                input: "
                2413432311323
                3215453535623
                3255245654254
                3446585845452
                4546657867536
                1438598798454
                4457876987766
                3637877979653
                4654967986887
                4564679986453
                1224686865563
                2546548887735
                4322674655533
            ",
                result: "102",
            },
        )
    }
}
