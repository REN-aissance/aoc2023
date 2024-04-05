use std::mem::replace;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn p1(input: &str) -> String {
    let mut hands = input
        .trim()
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();
            let bet = bet.parse::<u32>().unwrap();
            let hand = hand
                .chars()
                .map(|c| match c {
                    'A' => 12,
                    'K' => 11,
                    'Q' => 10,
                    'J' => 9,
                    'T' => 8,
                    c => c.to_digit(10).unwrap() as usize - 2,
                })
                .collect::<Vec<_>>();
            let counts = hand.iter().fold([0; 13], |mut acc, &n| {
                acc[n] += 1;
                acc
            });
            let counts = counts.into_iter().filter(|&n| n != 0).fold([0; 5], |mut acc, n| {
                    acc[n - 1] += 1;
                    acc
            });
            let score = match counts {
                [.., 1] => HandType::FiveOfAKind,
                [.., 1, _] => HandType::FourOfAKind,
                [_, 1, 1, ..] => HandType::FullHouse,
                [_, 0, 1, ..] => HandType::ThreeOfAKind,
                [_, 2, ..] => HandType::TwoPair,
                [_, 1, ..] => HandType::OnePair,
                _ => HandType::HighCard,
            };
            (score, hand, bet)
        })
        .collect::<Vec<_>>();
    hands.sort_unstable();
    hands
        .into_iter()
        .map(|tuple| tuple.2)
        .enumerate()
        .map(|(rank, bet)| (rank + 1) as u32 * bet)
        .sum::<u32>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let mut hands = input
        .trim()
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();
            let bet = bet.parse::<u32>().unwrap();
            let hand = hand
                .chars()
                .map(|c| match c {
                    'A' => 12,
                    'K' => 11,
                    'Q' => 10,
                    'J' => 0,
                    'T' => 9,
                    c => c.to_digit(10).unwrap() as usize - 1,
                })
                .collect::<Vec<_>>();
            let mut counts = hand.iter().fold([0; 13], |mut acc, &n| {
                acc[n] += 1;
                acc
            });
            *counts.iter_mut().max().unwrap() += replace(&mut counts[0], 0); //Jack is wild
            let counts = counts
                .into_iter()
                .filter(|&n| n != 0)
                .fold([0; 5], |mut acc, n| {
                    acc[n - 1] += 1;
                    acc
                });
            let score = match counts {
                [.., 1] => HandType::FiveOfAKind,
                [.., 1, _] => HandType::FourOfAKind,
                [_, 1, 1, ..] => HandType::FullHouse,
                [_, 0, 1, ..] => HandType::ThreeOfAKind,
                [_, 2, ..] => HandType::TwoPair,
                [_, 1, ..] => HandType::OnePair,
                _ => HandType::HighCard,
            };
            (score, hand, bet)
        })
        .collect::<Vec<_>>();
    hands.sort_unstable();
    hands
        .into_iter()
        .map(|tuple| tuple.2)
        .enumerate()
        .map(|(rank, bet)| (rank + 1) as u32 * bet)
        .sum::<u32>()
        .to_string()
}

