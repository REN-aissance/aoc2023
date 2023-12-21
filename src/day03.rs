const SYMBOLS: [char; 10] = ['@', '#', '$', '%', '&', '*', '/', '+', '=', '-'];
pub fn p1(s: &str) -> String {
    let mut sum = 0;
    let map = s
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for j in 0..map.len() {
        for i in 0..map[0].len() {
            let c = map[j][i];
            if SYMBOLS.contains(&c) {
                sum += nums_around(&map, i, j)
                    .into_iter()
                    .map(|e| e.2)
                    .sum::<u32>();
            }
        }
    }
    sum.to_string()
}

fn nums_around(map: &[Vec<char>], x: usize, y: usize) -> Vec<(usize, usize, u32)> {
    let mut out = Vec::with_capacity(8);
    for j in (y.saturating_sub(1))..=(y.saturating_add(1)) {
        for i in (x.saturating_sub(1))..=(x.saturating_add(1)) {
            if !(i == x && j == y) {
                if let Some(num) = build_num(map, i, j) {
                    out.push(num);
                }
            }
        }
    }
    out.dedup();
    out
}

fn build_num(map: &[Vec<char>], x: usize, y: usize) -> Option<(usize, usize, u32)> {
    let mut i = x;
    if !map.get(y)?.get(x)?.is_ascii_digit() {
        return None;
    }
    while let Some(c) = map[y].get(i) {
        if c.is_ascii_digit() {
            match i.checked_sub(1) {
                Some(n) => i = n,
                None => break,
            }
        } else {
            i += 1;
            break;
        }
    }
    let mut num = vec![];
    while let Some(c) = map[y].get(i)
        && c.is_ascii_digit()
    {
        num.push(c);
        match i.checked_add(1) {
            Some(n) => i = n,
            None => break,
        }
    }
    let num = num.into_iter().collect::<String>().parse::<u32>().ok()?;
    Some((i, y, num))
}

pub fn p2(s: &str) -> String {
    let mut sum = 0;
    let map = s
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for j in 0..map.len() {
        for i in 0..map[0].len() {
            if map[j][i] == '*' {
                let gears = nums_around(&map, i, j);
                if gears.len() == 2 {
                    sum += gears[0].2 * gears[1].2;
                }
            }
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let test = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let expected = 4361.to_string();
        assert_eq!(p1(test), expected);
        let test = "..50+50..";
        let expected = 100.to_string();
        assert_eq!(p1(test), expected);
        let test = "*50";
        let expected = 50.to_string();
        assert_eq!(p1(test), expected);
        let test = "12*\n...\n12*";
        let expected = 24.to_string();
        assert_eq!(p1(test), expected);
    }

    #[test]
    pub fn test_p2() {
        let test = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let expected = 467835.to_string();
        assert_eq!(p2(test), expected);
    }
}
