use std::collections::HashSet;

pub fn p1(s: &str) -> String {
    let mut sum = 0;
    s.lines().for_each(|line| {
        let card = line.split_once(':').unwrap().1;
        let (winning, present) = card.split_once('|').unwrap();
        let winning = winning
            .trim()
            .split_ascii_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let present = present
            .trim()
            .split_ascii_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let hits = winning.intersection(&present).count();
        if hits > 0 {
            sum += 2_u32.pow(hits as u32 - 1);
        }
    });
    sum.to_string()
}

pub fn p2(s: &str) -> String {
    let mut multipliers = vec![1; s.lines().count()];
    s.lines().enumerate().for_each(|(i, line)| {
        let card = line.split_once(':').unwrap().1;
        let (winning, present) = card.split_once('|').unwrap();
        let winning = winning
            .trim()
            .split_ascii_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let present = present
            .trim()
            .split_ascii_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let hits = winning.intersection(&present).count();
        for j in 0..hits {
            multipliers[i + j + 1] += multipliers[i];
        }
    });
    multipliers.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let test = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected = 13.to_string();
        assert_eq!(p1(test), expected);
    }

    #[test]
    pub fn test_p2() {
        let test = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected = 30.to_string();
        assert_eq!(p2(test), expected);
    }
}
