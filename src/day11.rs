use std::fmt::Display;

struct Map {
    galaxies: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Map {
    fn expand(&mut self, n: usize) {
        self.expand_x(n);
        self.expand_y(n);
    }
    fn expand_y(&mut self, n: usize) {
        for y in (0..self.height).rev() {
            if self.galaxies.iter().rev().all(|g| g.1 != y) {
                self.galaxies.iter_mut().filter(|g| g.1 > y).for_each(|g| {
                    g.1 += n - 1;
                });
                self.height += n - 1;
            }
        }
    }
    fn expand_x(&mut self, n: usize) {
        for x in (0..self.width).rev() {
            if !self.galaxies.iter().any(|g| g.0 == x) {
                self.galaxies.iter_mut().filter(|g| g.0 > x).for_each(|g| {
                    g.0 += n - 1;
                });
                self.width += n - 1;
            }
        }
    }
    fn taxicab_sum(&self) -> usize {
        let mut sum = 0;
        for i in 0..(self.galaxies.len() - 1) {
            for j in (i + 1)..self.galaxies.len() {
                let g1 = self.galaxies[i];
                let g2 = self.galaxies[j];
                sum += Self::taxicab(g1, g2);
            }
        }
        sum
    }
    fn taxicab(a: (usize, usize), b: (usize, usize)) -> usize {
        (a.1.max(b.1) - a.1.min(b.1)) + (a.0.max(b.0) - a.0.min(b.0))
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let height = value.lines().count();
        let width = value.lines().next().unwrap().chars().count();
        let galaxies = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
            })
            .collect();
        Map {
            galaxies,
            height,
            width,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.galaxies.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

pub fn p1(s: &str) -> String {
    let mut map = Map::from(s);
    map.expand(2);
    map.taxicab_sum().to_string()
}

pub fn p2(s: &str) -> String {
    let mut map = Map::from(s);
    map.expand(1_000_000);
    map.taxicab_sum().to_string()
}

pub fn test(s: &str, n: usize) -> String {
    let mut map = Map::from(s);
    map.expand(n);
    map.taxicab_sum().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let input = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
        assert_eq!(p1(input), 374.to_string());
    }

    #[test]
    fn test_p2() {
        let input = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
        assert_eq!(test(input, 10), 1030.to_string());
    }

    #[test]
    fn test_p2_2() {
        let input = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
        assert_eq!(test(input, 100), 8410.to_string());
    }
}
