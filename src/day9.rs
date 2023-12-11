pub fn p1(s: &str) -> String {
    s.lines()
        .map(|line| {
            let mut line = line
                .split_ascii_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let mut endings = vec![];
            while !line.iter().rev().all(|e| e == &0) {
                endings.push(*line.last().unwrap());
                line = line
                    .into_iter()
                    .map_windows(|&[a, b]| b - a)
                    .collect::<Vec<_>>();
            }
            endings.into_iter().sum::<i32>()
        })
        .sum::<i32>()
        .to_string()
}

pub fn p2(s: &str) -> String {
    s.lines()
        .map(|line| {
            let mut line = line
                .split_ascii_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let mut starts = vec![];
            while !line.iter().rev().all(|e| e == &0) {
                starts.push(*line.first().unwrap());
                line = line
                    .into_iter()
                    .map_windows(|&[a, b]| b - a)
                    .collect::<Vec<_>>();
            }
            starts.into_iter().rev().reduce(|acc, e| e - acc).unwrap()
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
        let sums = [18, 28, 68].into_iter();
        let i = input
            .lines()
            .zip(sums)
            .map(|(line, sum)| {
                assert_eq!(p1(line), sum.to_string());
                p1(line).parse::<i32>().unwrap()
            })
            .sum::<i32>();
        assert_eq!(i, 114);
    }
    #[test]
    pub fn test_p2() {
        let input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
        let sums = [-3, 0, 5].into_iter();
        let i = input
            .lines()
            .zip(sums)
            .map(|(line, sum)| {
                assert_eq!(p2(line), sum.to_string());
                p2(line).parse::<i32>().unwrap()
            })
            .sum::<i32>();
        assert_eq!(i, 2);
    }
}
