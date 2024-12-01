use crate::{Day, TestInput};

enum Object {
    RoundRock,
    RoughRock,
    Tile,
}

type RockMap = Vec<Vec<Object>>;

fn run_cycle(rock_map: &mut RockMap) {}

pub struct Day14;
impl Day for Day14 {
    fn date() -> (i32, i32) {
        (2023, 14)
    }

    fn part1(lines: &[String]) -> String {
        let mut result = 0;
        for x in 0..lines[0].len() {
            let mut total = 0;
            let mut value = lines.len();

            for (y, line) in lines.iter().enumerate() {
                match line.as_bytes()[x] {
                    b'O' => {
                        total += value;
                        value -= 1;
                    }
                    b'#' => {
                        value = lines.len() - y - 1;
                    }
                    _ => {}
                }
            }

            result += total;
        }

        format!("{result}")
    }

    fn part2(lines: &[String]) -> String {
        todo!()
    }

    fn test_inputs() -> (TestInput, TestInput) {
        (
            TestInput {
                input: "
                    O....#....
                    O.OO#....#
                    .....##...
                    OO.#O....O
                    .O.....O#.
                    O.#..O.#.#
                    ..O..#O..O
                    .......O..
                    #....###..
                    #OO..#....
                ",
                result: "136",
            },
            TestInput {
                input: "
                    ???.### 1,1,3
                    .??..??...?##. 1,1,3
                    ?#?#?#?#?#?#?#? 1,3,1,6
                    ????.#...#... 4,1,1
                    ????.######..#####. 1,6,5
                    ?###???????? 3,2,1
                ",
                result: "21",
            },
        )
    }
}
