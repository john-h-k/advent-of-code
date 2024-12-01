use std::array;

use crate::{Day, TestInput};

fn hash(string: &str) -> usize {
    string
        .chars()
        .fold(0usize, |acc, c| (acc + (c as usize)) * 17 % 256)
}

pub struct Day15;
impl Day for Day15 {
    fn date() -> (i32, i32) {
        (2023, 15)
    }

    fn part1(lines: &[String]) -> String {
        let result = lines
            .iter()
            .flat_map(|l| l.split(',').map(hash))
            .sum::<usize>();

        format!("{result}")
    }

    fn part2(lines: &[String]) -> String {
        let components = lines.iter().flat_map(|l| l.split(','));

        let mut boxes: [Vec<_>; 256] = array::from_fn(|_| Vec::new());
        for component in components {
            let div = component.find('-').or_else(|| component.find('=')).unwrap();
            let (label, value) = component.split_at(div);
            let value = &value[1..];

            let box_idx = hash(label);
            let pos = boxes[box_idx]
                .iter()
                .position(|&(other_label, _)| other_label == label);

            if let Ok(value) = value.parse::<usize>() {
                if let Some(pos) = pos {
                    boxes[box_idx][pos].1 = value;
                } else {
                    boxes[box_idx].push((label, value));
                }
            } else if let Some(pos) = pos {
                boxes[box_idx].remove(pos);
            }
        }

        let result = boxes
            .into_iter()
            .enumerate()
            .flat_map(|(i, lens_box)| {
                lens_box
                    .into_iter()
                    .enumerate()
                    .map(move |(j, (_, value))| (1 + i) * (1 + j) * value)
            })
            .sum::<usize>();

        format!("{result}")
    }

    fn test_inputs() -> (TestInput, TestInput) {
        (
            TestInput {
                input: "
                rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
            ",
                result: "1320",
            },
            TestInput {
                input: "
                rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
            ",
                result: "145",
            },
        )
    }
}
