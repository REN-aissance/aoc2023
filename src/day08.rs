use std::collections::HashMap;

pub fn p1(s: &str) -> String {
    let mut s = s.trim_end().lines();
    let instructions = s.next().unwrap().chars().map(|c| match c {
        'L' => 0,
        'R' => 1,
        _ => panic!("Invalid instructions"),
    });

    //discard blank line
    s.next();

    let map = s
        .clone()
        .enumerate()
        .map(|(i, line)| (line.split_once('=').unwrap().0.trim(), i))
        .collect::<HashMap<&str, usize>>();

    let graph = s
        .map(|line| {
            line.split_once(" = ")
                .unwrap()
                .1
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .map(|e| *map.get(e.trim_start()).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let mut count = 0;
    let mut cur = *map.get("AAA").unwrap();
    while cur != *map.get("ZZZ").unwrap() {
        instructions.clone().for_each(|i| {
            // print!("{} -> ", cur);
            cur = graph[cur][i];
            // println!("{}", cur);
            count += 1;
        });
    }
    count.to_string()
}

pub fn p2(s: &str) -> String {
    let mut s = s.trim_end().lines();
    let instructions = s
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid instructions"),
        })
        .cycle();

    //discard blank line
    s.next();

    let map = s
        .clone()
        .enumerate()
        .map(|(i, line)| (line.split_once('=').unwrap().0.trim(), i))
        .collect::<HashMap<&str, usize>>();

    let graph = s
        .map(|line| {
            line.split_once(" = ")
                .unwrap()
                .1
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .map(|e| *map.get(e.trim_start()).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let starts = map
        .keys()
        .filter(|c| c.ends_with('A'))
        .map(|s| *map.get(s).unwrap())
        .collect::<Vec<_>>();
    let ends = map
        .keys()
        .filter(|c| c.ends_with('Z'))
        .map(|s| *map.get(s).unwrap())
        .collect::<Vec<_>>();

    let mut counts = vec![];
    for mut cur in starts {
        let mut count = 0;
        let mut i = instructions.clone();
        while !ends.contains(&cur) {
            cur = graph[cur][i.next().unwrap()];
            count += 1;
        }
        counts.push(count);
    }
    counts
        .into_iter()
        .reduce(|acc, x| lcm(x, acc))
        .unwrap()
        .to_string()
}

fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1_1() {
        let input = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";
        assert_eq!(p1(input), 2.to_string());
    }
    #[test]
    pub fn test_p2() {
        let input = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        assert_eq!(p2(input), 6.to_string());
    }
}
