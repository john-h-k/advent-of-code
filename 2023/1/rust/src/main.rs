use std::fs;

const WORD_MAP: [(&str, u32); 18] = [
    ("one", 1),
    ("1", 1),
    ("two", 2),
    ("2", 2),
    ("three", 3),
    ("3", 3),
    ("four", 4),
    ("4", 4),
    ("five", 5),
    ("5", 5),
    ("six", 6),
    ("6", 6),
    ("seven", 7),
    ("7", 7),
    ("eight", 8),
    ("8", 8),
    ("nine", 9),
    ("9", 9),
];

fn first_digit_or_word(line: &[u8], mut indices: impl Iterator<Item = usize>) -> Option<u32> {
    indices.find_map(|s| {
        WORD_MAP
            .iter()
            .find_map(|(word, value)| line[s..].starts_with(word.as_bytes()).then_some(*value))
    })
}

fn find_value(line: &[u8]) -> u32 {
    let range = 0..line.len();
    let first = first_digit_or_word(line, range.clone());
    let last = first_digit_or_word(line, range.rev());

    first.unwrap() * 10 + last.unwrap()
}

fn main() {
    let result: u32 = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|l| find_value(l.as_bytes()))
        .sum();

    println!("{result}");
}
