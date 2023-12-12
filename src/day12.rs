use std::collections::HashMap;

pub fn p1(s: &str) -> String {
    s.trim()
        .lines()
        .map(|line| {
            let (s, nums) = line.split_once(' ').unwrap();
            let nums = nums
                .trim()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();
            valid_constructions(s.as_bytes(), &nums)
        })
        .sum::<u64>()
        .to_string()
}

pub fn p2(s: &str) -> String {
    s.trim()
        .lines()
        .map(|line| {
            let (s, nums) = line.split_once(' ').unwrap();
            let nums = nums.trim().split(',').map(|n| n.parse().unwrap());
            let nums = vec![nums; 5].into_iter().flatten().collect::<Vec<_>>();
            let s = vec![s; 5].into_iter().intersperse("?").collect::<String>();
            valid_constructions(s.as_bytes(), nums.as_slice())
        })
        .sum::<u64>()
        .to_string()
}

fn valid_constructions(s: &[u8], nums: &[usize]) -> u64 {
    let mut cache = HashMap::new();
    valid_constructions_r(s, nums, &mut cache)
}

fn valid_constructions_r<'a>(
    s: &'a [u8],
    nums: &'a [usize],
    cache: &mut HashMap<(&'a [u8], &'a [usize]), u64>,
) -> u64 {
    if let Some(n) = cache.get(&(s, nums)) {
        return *n;
    }

    if (s.is_empty() && !nums.is_empty()) || (nums.is_empty() && s.iter().any(|b| b == &b'#')) {
        cache.insert((s, nums), 0);
        return 0;
    } else if nums.is_empty() {
        cache.insert((s, nums), 1);
        return 1;
    }

    let num = *nums.first().unwrap();
    let mut count = 0;
    for i in 0..s.len() {
        let prev = i.checked_sub(1).map(|n| s[n]).unwrap_or(b'.');
        if prev != b'#' {
            let next = s.get(num + i).copied().unwrap_or(b'.');
            if s.get(i..(i + num))
                .map(|b| b.iter().all(|b| b != &b'.'))
                .unwrap_or(false)
                && next != b'#'
            {
                let s = s.get((num + i + 1)..).unwrap_or_default();
                count += valid_constructions_r(s, &nums[1..], cache);
            }
        } else {
            break;
        }
    }

    cache.insert((s, nums), count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let input = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";
        let combinations = [1, 4, 1, 1, 4, 10];
        input.lines().zip(combinations).for_each(|(line, num)| {
            assert_eq!(
                p1(line),
                num.to_string(),
                "\nInput [{}] should match {}",
                line,
                num
            )
        });
        assert_eq!(p1(input), combinations.iter().sum::<usize>().to_string());
    }

    #[test]
    fn valid_constructions_1() {
        assert_eq!(
            super::valid_constructions(b"?", &[1]),
            1,
            "Expected 1 valid construction for '?'"
        );
    }

    #[test]
    fn valid_constructions_2() {
        assert_eq!(
            super::valid_constructions(b"??", &[1]),
            2,
            "Expected 2 valid constructions for '??'"
        );
    }

    #[test]
    fn valid_constructions_3() {
        assert_eq!(
            super::valid_constructions(b"???", &[1]),
            3,
            "Expected 3 valid constructions for '???'"
        );
    }

    #[test]
    fn valid_constructions_4() {
        assert_eq!(
            super::valid_constructions(b"???", &[1, 1]),
            1,
            "Expected 1 valid construction for '???' with 1"
        );
    }

    #[test]
    fn valid_constructions_5() {
        assert_eq!(
            super::valid_constructions(b"?#?", &[1]),
            1,
            "Expected 1 valid construction for '?#?'"
        );
    }

    #[test]
    fn valid_constructions_6() {
        assert_eq!(
            super::valid_constructions(b"?#?#", &[1, 1]),
            1,
            "Expected 1 valid construction for '?#?#'"
        );
    }

    #[test]
    fn valid_constructions_7() {
        assert_eq!(
            super::valid_constructions(b"???.###", &[1, 1, 3]),
            1,
            "Expected 1 valid construction for '???.###'"
        );
    }

    #[test]
    fn valid_constructions_8() {
        assert_eq!(
            super::valid_constructions(b"?#?#?#?#?#?#?#?", &[1, 3, 1, 6]),
            1,
            "Expected 1 valid construction for '?#?#?#?#?#?#?#?'"
        );
    }

    #[test]
    fn valid_constructions_9() {
        assert_eq!(
            super::valid_constructions(b".", &[]),
            1,
            "Expected 0 valid construction for '."
        );
    }

    #[test]
    fn valid_constructions_10() {
        assert_eq!(
            super::valid_constructions(b"#?###?.???.??????", &[1, 4, 2, 2, 2]),
            6,
        );
    }

    #[test]
    fn valid_constructions_11() {
        assert_eq!(super::valid_constructions(b"#??.?", &[1, 1]), 2);
    }

    #[test]
    fn valid_constructions_12() {
        assert_eq!(super::valid_constructions(b"?.#???.#??.#", &[1, 3, 1]), 1);
    }
}
