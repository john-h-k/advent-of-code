use crate::Day;

pub struct Day2;
impl Day for Day2 {
    fn part1(lines: &[String]) -> String {
        let score: i32 = lines
            .iter()
            .flat_map(|line| {
                let (game_name, rounds) = line.split_once(':').unwrap();
                let id: i32 = game_name.split_once(' ').unwrap().1.parse().unwrap();

                rounds
                    .split(';')
                    .flat_map(|round| round.split(','))
                    .map(|entry| entry.trim().split_once(' ').unwrap())
                    .try_fold(id, |id, (number, color)| {
                        let number = number.parse::<i32>().unwrap();
                        let lim = match color {
                            "red" => 12,
                            "green" => 13,
                            "blue" => 14,
                            _ => unreachable!("bad color"),
                        };

                        if number <= lim {
                            Ok(id)
                        } else {
                            Err(())
                        }
                    })
            })
            .sum();

        format!("{score}")
    }

    fn part2(lines: &[String]) -> String {
        let score: i32 = lines
            .iter()
            .map(|line| {
                let (_, rounds) = line.split_once(':').unwrap();

                let (r, g, b) = rounds
                    .split(';')
                    .flat_map(|round| round.split(','))
                    .map(|entry| entry.trim().split_once(' ').unwrap())
                    .map(|(number, color)| {
                        let number = number.parse::<i32>().unwrap();
                        match color {
                            "red" => (number, 0, 0),
                            "green" => (0, number, 0),
                            "blue" => (0, 0, number),
                            _ => unreachable!("bad color"),
                        }
                    })
                    .fold(
                        (0, 0, 0),
                        |(max_red, max_green, max_blue), (red, green, blue)| {
                            (max_red.max(red), max_green.max(green), max_blue.max(blue))
                        },
                    );

                r * g * b
            })
            .sum();

        format!("{score}")
    }
}
