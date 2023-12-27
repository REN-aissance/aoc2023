use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn to_offset(self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (0, 1),
        }
    }
}

pub fn p1(s: &str) -> String {
    let mut map = HashSet::new();
    let mut cur = (0, 0);

    s.lines().for_each(|line| {
        let mut iter = line.split_ascii_whitespace();
        let dir = match iter.next().unwrap() {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!(),
        };
        let count = iter.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..count {
            map.insert(cur);
            cur = (cur.0 + dir.0, cur.1 + dir.1)
        }
    });

    let x_min = map.iter().copied().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let x_max = map.iter().copied().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let y_min = map.iter().copied().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let y_max = map.iter().copied().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    //cast rays to count interior area
    let mut sum = 0;
    for y in y_min..=y_max {
        let mut prev_is_boundary = false;
        let mut inside = false;
        let mut on_edge = false;
        let mut curve_from_below = false;
        for x in x_min..=x_max {
            let is_boundary = map.contains(&(x, y));

            //encountered boundary
            if !prev_is_boundary && is_boundary {
                inside = !inside;
                curve_from_below = map.contains(&(x, y + 1));
            }
            //stepped over a boundary
            if prev_is_boundary && !is_boundary {
                if on_edge && curve_from_below == map.contains(&(x - 1, y + 1)) {
                    inside = !inside;
                }
                on_edge = false;
                curve_from_below = false;
            }
            //on a boundary
            if prev_is_boundary && is_boundary {
                on_edge = true;
            }

            if is_boundary {
                sum += 1;
            } else if inside {
                sum += 1;
                map.insert((x, y));
            }

            prev_is_boundary = is_boundary;
        }
    }

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    sum.to_string()
}

pub fn p2(s: &str) -> String {
    0.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)";
        assert_eq!(p1(input), 62.to_string());
    }

    #[test]
    fn test_p2() {
        let input = "";
        assert_eq!(p2(input), 0.to_string());
    }
}
