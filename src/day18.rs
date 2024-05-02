use std::collections::HashSet;

//This approach isn't great I found. But I'm going to leave it this way to
//preserve the technique. I think It's a decent algorithm (for some other purpose)
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
        let mut curve_from_below = false; //Dummy value never read

        for x in x_min..=x_max {
            let is_boundary = map.contains(&(x, y));

            //encountered boundary
            if !prev_is_boundary && is_boundary {
                inside = !inside;
                curve_from_below = map.contains(&(x, y + 1));
            }
            //on a boundary
            else if prev_is_boundary && is_boundary {
                on_edge = true;
            }
            //stepped over a boundary
            else if prev_is_boundary && !is_boundary {
                if on_edge && curve_from_below == map.contains(&(x - 1, y + 1)) {
                    inside = !inside;
                }
                on_edge = false;
                curve_from_below = false; //Dummy value never read
            }
            //Counting and insertion for printout later
            if is_boundary {
                sum += 1;
            } else if inside {
                sum += 1;
                map.insert((x, y));
            }

            prev_is_boundary = is_boundary;
        }
    }

    //pretty map print
    // for y in y_min..=y_max {
    //     for x in x_min..=x_max {
    //         if map.contains(&(x, y)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    // println!();

    sum.to_string()
}

type Num = i64;

//Vaguely knew about Pick's Theorem. First time using it
pub fn p2(s: &str) -> String {
    let mut cur = (5, 5);
    let mut verticies = vec![5, 5];
    let mut exterior_points = 0;
    s.trim().lines().for_each(|line| {
        let mut iter = line.split_ascii_whitespace();
        iter.next(); //Skip char
        iter.next(); //Skip "count"
        let color_nums = iter
            .next()
            .unwrap()
            .chars()
            .skip(2)
            .take(6)
            .map(|c| c.to_digit(16).unwrap() as Num)
            .collect::<Vec<_>>();

        let len = color_nums[0..5]
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, d)| (16 as Num).pow(i as u32) * d + acc);

        let step = match color_nums[5] {
            0 => (len, 0),
            1 => (0, len),
            2 => (-len, 0),
            3 => (0, -len),
            _ => panic!("Invalid direction"),
        };
        let new = (step.0 + cur.0, step.1 + cur.1);
        exterior_points += len;
        verticies.push(new.0);
        verticies.push(new.1);
        cur = new
    });

    //Shoelace theorem
    let area = verticies
        .iter()
        .map_windows(|&[a, b, c, d]| (a * d) - (b * c))
        .step_by(2)
        .sum::<Num>()
        / 2;

    //Rearranged Pick's Theorem
    let interior_points = area - exterior_points / 2 + 1;

    (interior_points + exterior_points).to_string()
}

#[cfg(test)]
mod tests {
    // use std::time::Instant;

    use super::*;

    #[test]
    fn test_p1() {
        let input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)";
        // let start = Instant::now();
        assert_eq!(p1(input), 62.to_string());
        // let elapsed = start.elapsed().as_micros();
        // println!("{}us", elapsed);
    }

    #[test]
    fn test_p2() {
        let input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)";
        // let start = Instant::now();
        assert_eq!(p2(input), 952408144115_i64.to_string());
        // let elapsed = start.elapsed().as_micros();
        // println!("{}us", elapsed);
    }
}
