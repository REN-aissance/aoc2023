use core::mem::variant_count;

const HAND_SIZE: usize = 5;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
enum Card {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::_10,
            '9' => Card::_9,
            '8' => Card::_8,
            '7' => Card::_7,
            '6' => Card::_6,
            '5' => Card::_5,
            '4' => Card::_4,
            '3' => Card::_3,
            '2' => Card::_2,
            _ => panic!("Invalid input"),
        }
    }
}
impl Card {
    pub fn idx1(&self) -> usize {
        *self as usize
    }
    pub fn idx2(&self) -> usize {
        let i = self.idx1();
        match i {
            9 => 0,
            0..=8 => i + 1,
            _ => i,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
struct Hand([Card; HAND_SIZE]);
impl Hand {
    fn score_p1(&self) -> Score {
        let mut counts = [0; variant_count::<Card>()];
        self.0.iter().for_each(|c| counts[c.idx1()] += 1);
        let mut count_counts = [0; HAND_SIZE];
        counts
            .into_iter()
            .filter(|&i| i != 0)
            .for_each(|i| count_counts[i - 1] += 1); //fix indexing
        Hand::score_from_counts(count_counts)
    }

    fn score_p2(&self) -> Score {
        let mut counts = [0; variant_count::<Card>()];
        let mut jacks = 0;
        self.0.iter().for_each(|c| counts[c.idx2()] += 1);
        //Don't extract jacks if there are 5, thus ignoring the jack rule
        if counts[0] != 5 {
            std::mem::swap(&mut counts[0], &mut jacks);
        }

        let mut count_counts = [0; HAND_SIZE];
        counts
            .into_iter()
            .filter(|&i| i != 0)
            .for_each(|i| count_counts[i - 1] += 1); //fix indexing

        //jack logic causes highest frequency card to be incremented, count map logic looks like this
        if jacks > 0 {
            for i in (0..count_counts.len()).rev() {
                if count_counts[i] != 0 {
                    count_counts[i + jacks] = 1;
                    count_counts[i] -= 1;
                    break;
                }
            }
        }
        Hand::score_from_counts(count_counts)
    }

    fn score_from_counts(counts: [usize; 5]) -> Score {
        if counts[4] == 1 {
            Score::FiveOfAKind
        } else if counts[3] == 1 {
            Score::FourOfAKind
        } else if counts[2] == 1 && counts[1] == 1 {
            Score::FullHouse
        } else if counts[2] == 1 {
            Score::ThreeOfAKind
        } else if counts[1] == 2 {
            Score::TwoPair
        } else if counts[1] == 1 {
            Score::OnePair
        } else {
            Score::HighCard
        }
    }
}

impl FromIterator<char> for Hand {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Hand([
            iter.next().unwrap().into(),
            iter.next().unwrap().into(),
            iter.next().unwrap().into(),
            iter.next().unwrap().into(),
            iter.next().unwrap().into(),
        ])
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
enum Score {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn p1(s: &str) -> String {
    let mut hands = s
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();
            let hand = hand.chars().collect::<Hand>();
            let score = hand.score_p1();
            let bet = bet.parse::<u32>().unwrap();
            (score, hand, bet)
        })
        .collect::<Vec<_>>();
    hands.sort();
    hands
        .into_iter()
        .map(|e| e.2) //Hands, scores no longer necessary
        .enumerate()
        .fold(0, |acc, (i, bet)| acc + ((i as u32 + 1) * bet))
        .to_string()
}

pub fn p2(s: &str) -> String {
    use std::cmp::Ordering;

    let mut hands = s
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();
            let hand = hand.chars().collect::<Hand>();
            let score = hand.score_p2();
            let bet = bet.parse::<u32>().unwrap();
            (score, hand, bet)
        })
        .collect::<Vec<_>>();

    //Annoying custom sort for p2
    hands.sort_by(|a, b| {
        //Sort by score unless equal
        let ord = a.0.cmp(&b.0);
        if !ord.is_eq() {
            return ord;
        }

        //Otherwise sort by cards
        for (a, b) in a.1 .0.iter().zip(b.1 .0.iter()) {
            let ord = a.idx2().cmp(&b.idx2());
            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    });
    hands
        .into_iter()
        .map(|e| e.2) //Hands no longer necessary
        .enumerate()
        .fold(0, |acc, (i, bet)| acc + ((i as u32 + 1) * bet))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        assert_eq!(p1(input), 6440.to_string());
    }
    #[test]
    pub fn test_p2() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        assert_eq!(p2(input), 5905.to_string());
    }
}
