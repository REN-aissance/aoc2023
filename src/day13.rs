pub fn p1(s: &str) -> String {
    s.trim_end()
        .split("\n\n")
        .map(|mirror| {
            let mut o = 0;
            let w = mirror.lines().next().unwrap().len();
            let h = mirror.lines().count();
            let mirror: Vec<_> = mirror.lines().flat_map(|line| line.chars()).collect();

            //Test vertical reflection
            let v1 = Vec::with_capacity(h);
            let v2: Vec<_> = (0..h)
                .map(|y| mirror.get((y * w)..(y * w + w)).unwrap())
                .rev()
                .collect();

            if let Some(n) = has_mirror(v2, v1) {
                o += 100 * n;
            }

            //Test horizontal reflection
            let v1 = Vec::with_capacity(w);
            let v2 = flat_vec_to_2d_transpose(w, h, mirror);

            if let Some(n) = has_mirror(v2, v1) {
                o += n;
            }
            o
        })
        .sum::<usize>()
        .to_string()
}

fn has_mirror<T>(mut v2: Vec<T>, mut v1: Vec<T>) -> Option<usize>
where
    T: Eq + PartialEq,
{
    while let Some(v) = v2.pop()
        && !v2.is_empty()
    {
        v1.push(v);
        let n = v1.len().min(v2.len());
        if v1
            .iter()
            .rev()
            .zip(v2.iter().rev().take(n))
            .all(|(ref a, ref b)| a == b)
        {
            return Some(v1.len());
        }
    }
    None
}

fn has_mirror_one_flip<T>(mut v2: Vec<T>, mut v1: Vec<T>) -> Option<usize>
where
    T: AsRef<[char]>,
{
    while let Some(v) = v2.pop()
        && !v2.is_empty()
    {
        v1.push(v);
        let n = v1.len().min(v2.len());
        if let Some(len) = has_one_match_in_many(&v1, &v2, n) {
            return Some(len);
        }
    }
    None
}

fn has_one_match_in_many<T>(v1: &[T], v2: &[T], n: usize) -> Option<usize>
where
    T: AsRef<[char]>,
{
    let len = v1.len();
    let v1 = v1.iter().rev();
    let v2 = v2.iter().rev().take(n);
    if v1
        .zip(v2)
        .try_fold(false, |acc, (a, b)| {
            if has_one_match_in_one(a, b) {
                match acc {
                    true => Err(()),
                    false => Ok(true),
                }
            } else if a
                .as_ref()
                .iter()
                .zip(b.as_ref().iter())
                .all(|(a, b)| a == b)
            {
                Ok(acc)
            } else {
                Err(())
            }
        })
        .unwrap_or_default()
    {
        Some(len)
    } else {
        None
    }
}

fn has_one_match_in_one<T>(a: T, b: T) -> bool
where
    T: AsRef<[char]>,
{
    a.as_ref()
        .iter()
        .zip(b.as_ref())
        .try_fold(false, |acc, (a, b)| {
            if a != b {
                match acc {
                    true => Err(()),
                    false => Ok(true),
                }
            } else {
                Ok(acc)
            }
        })
        .unwrap_or_default()
}

fn flat_vec_to_2d_transpose(w: usize, h: usize, mirror: Vec<char>) -> Vec<Vec<char>> {
    let mut vec = Vec::with_capacity(w);
    for x in 0..w {
        let mut col = Vec::with_capacity(h);
        for y in 0..h {
            col.push(mirror[y * w + x]);
        }
        vec.push(col);
    }
    vec.reverse();
    vec
}

pub fn p2(s: &str) -> String {
    s.trim_end()
        .split("\n\n")
        .map(|mirror| {
            let w = mirror.lines().next().unwrap().len();
            let h = mirror.lines().count();
            let mirror: Vec<_> = mirror.lines().flat_map(|line| line.chars()).collect();

            //Test vertical reflection
            let v1 = Vec::with_capacity(h);
            let v2: Vec<_> = (0..h)
                .map(|y| mirror.get((y * w)..(y * w + w)).unwrap())
                .rev()
                .collect();

            if let Some(n) = has_mirror_one_flip(v2, v1) {
                return 100 * n;
            }

            //Test horizontal reflection
            let v1 = Vec::with_capacity(w);
            let v2 = flat_vec_to_2d_transpose(w, h, mirror);

            if let Some(n) = has_mirror_one_flip(v2, v1) {
                return n;
            }
            0
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let input = "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
        assert_eq!(p1(input), 400.to_string());
    }

    #[test]
    fn test_p1_2() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
        assert_eq!(p1(input), 5.to_string());
    }

    #[test]
    fn test_p1_3() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
        assert_eq!(p1(input), 405.to_string());
    }

    #[test]
    fn test_p1_4() {
        let input = "#.#\n#.#";
        assert_eq!(p1(input), 100.to_string());
    }

    #[test]
    fn test_p1_5() {
        let input = "##\n..\n##";
        assert_eq!(p1(input), 1.to_string());
    }

    #[test]
    fn test_p2_1() {
        let input = "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
        assert_eq!(p2(input), 100.to_string());
    }

    #[test]
    fn test_p2_2() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
        assert_eq!(p2(input), 400.to_string());
    }

    #[test]
    fn test_p2_3() {
        let input = "#..\n#.#\n...";
        assert_eq!(p2(input), 100.to_string());
    }

    #[test]
    fn test_p2_4() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
        assert_eq!(p2(input), 300.to_string());
    }

    #[test]
    fn test_p2_5() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.##\n.###..##.\n#.#.##.#.";
        assert_eq!(p2(input), 0.to_string());
    }
}
