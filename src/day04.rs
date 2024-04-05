use std::{
    collections::{HashSet, VecDeque},
    u32,
};

pub fn p1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(':').unwrap();
            let (cur, winning) = line.split_once('|').unwrap();
            let cur = cur
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect::<HashSet<u8>>();
            let winning = winning
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect::<HashSet<u8>>();
            cur.intersection(&winning).count()
        })
        .filter(|&i| i != 0)
        .map(|i| 2u32.pow(i as u32 - 1))
        .sum::<u32>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let mut buf = VecDeque::from([1u32; 10]);
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(':').unwrap();
            let (cur, winning) = line.split_once('|').unwrap();
            let cur = cur
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect::<HashSet<u8>>();
            let winning = winning
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect::<HashSet<u8>>();
            cur.intersection(&winning).count()
        })
        .map(|i| {
            let mul = buf.pop_front().unwrap();
            buf.push_back(1);
            (0..i).for_each(|n| {
                buf[n] += mul;
            });
            mul
        })
        .sum::<u32>()
        .to_string()
}
