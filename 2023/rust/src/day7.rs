use itertools::Itertools;

use crate::{Day, TestInput};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

fn card_value(card: u8) -> u8 {
    match card {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => 11,
        b'T' => 10,
        _ => char::to_digit(card as char, 10).unwrap() as u8,
    }
}

fn hand_type(hand: &[u8]) -> HandType {
    let groups = hand.iter().sorted().group_by(|c| *c);
    let mut group_counts = groups
        .into_iter()
        .map(|(_, group)| group.count())
        .collect_vec();

    group_counts.sort();

    match group_counts[..] {
        [5] => HandType::FiveOfKind,
        [1, 4] => HandType::FourOfKind,
        [2, 3] => HandType::FullHouse,
        [.., 3] => HandType::ThreeOfKind,
        [.., 2, 2] => HandType::TwoPair,
        [.., 2] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

type Hand = (u8, u8, u8, u8, u8);

pub struct Day7;
impl Day for Day7 {
    fn date() -> (i32, i32) {
        (2023, 7)
    }

    fn part1(lines: &[String]) -> String {
        let order = lines
            .iter()
            .map(|l| {
                let (hand, bid) = l.split_once(' ').unwrap();
                (hand, bid.parse::<usize>().unwrap())
            })
            .sorted_by_key(|(hand, _bid)| {
                let hand_ty = hand_type(hand.as_bytes());
                let values: Hand = hand.bytes().map(card_value).collect_tuple().unwrap();

                (hand_ty, values)
            });

        let result = order
            .enumerate()
            .map(|(i, (_hand, bid))| (i + 1) * bid)
            .sum::<usize>();

        format!("{result}")
    }

    fn part2(lines: &[String]) -> String {
        let order = lines
            .iter()
            .map(|l| {
                let (hand, bid) = l.split_once(' ').unwrap();
                (hand.replace('J', "1"), bid.parse::<usize>().unwrap())
            })
            .sorted_by_key(|(hand, _bid)| {
                dbg!(hand);
                let hand_ty = hand_type(hand.as_bytes());
                let joker_count = hand.bytes().filter(|c| *c == b'1').count();

                dbg!(joker_count, hand_ty);
                let hand_ty = match (joker_count, hand_ty) {
                    (_, HandType::FullHouse) if joker_count > 0 => HandType::FiveOfKind,
                    (_, HandType::FourOfKind) if joker_count > 0 => HandType::FiveOfKind,
                    (_, HandType::ThreeOfKind) if joker_count > 0 => HandType::FourOfKind,
                    (2, HandType::TwoPair) => HandType::FullHouse,
                    (1, HandType::TwoPair) => HandType::FullHouse,
                    (1, HandType::OnePair) => HandType::ThreeOfKind,
                    _ => hand_ty,
                };
                dbg!(hand_ty);

                let values: Hand = hand.bytes().map(card_value).collect_tuple().unwrap();

                (hand_ty, values)
            });

        let result = order
            .enumerate()
            .map(|(i, (_hand, bid))| (i + 1) * bid)
            .sum::<usize>();

        format!("{result}")
    }

    fn test_inputs() -> (TestInput, TestInput) {
        (
            TestInput {
                input: "
                32T3K 765
                T55J5 684
                KK677 28
                KTJJT 220
                QQQJA 483
            ",
                result: "6440",
            },
            TestInput {
                input: "
                32T3K 765
                T55J5 684
                KK677 28
                KTJJT 220
                QQQJA 483
            ",
                result: "5905",
            },
        )
    }
}
