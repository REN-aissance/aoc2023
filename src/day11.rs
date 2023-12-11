use std::fmt::Display;

struct Map(Vec<Vec<char>>);

impl Map {
    fn add_blank_row(&mut self, i: usize) {
        self.0.insert(i, vec!['.'; self.0[0].len()])
    }
    fn add_blank_column(&mut self, i: usize) {
        self.0.iter_mut().for_each(|vec| vec.insert(i, '.'));
    }
    fn expand(&mut self) {
        self.expand_x();
        self.expand_y();
    }
    fn expand_y(&mut self) {
        for i in (0..self.0.len()).rev() {
            if self.0[i].iter().all(|c| c == &'.') {
                self.add_blank_row(i);
            }
        }
    }
    fn expand_x(&mut self) {
        for x in (0..self.0[0].len()).rev() {
            let mut empty = true;
            for y in (0..self.0.len()).rev() {
                if self.0[y][x] == '#' {
                    empty = false;
                }
            }
            if empty {
                self.add_blank_column(x)
            }
        }
    }
    fn get_galaxies(&self) -> Vec<(usize, usize)> {
        let mut o = vec![];
        for (y, row) in self.0.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if c == &'#' {
                    o.push((x, y));
                }
            }
        }
        o
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Map(value.lines().map(|line| line.chars().collect()).collect())
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(|vec| writeln!(f, "{}", vec.iter().collect::<String>()))
            .find(|e| e.is_err())
            .unwrap_or(Ok(()))
    }
}

pub fn p1(s: &str) -> String {
    let mut map = Map::from(s);
    println!("{}", map);
    map.expand();
    println!("{}", map);

    let galaxies = map.get_galaxies();
    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i..galaxies.len() {
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            let taxicab = (g2.1.max(g1.1) - g2.1.min(g1.1)) + (g2.0.max(g1.0) - g2.0.min(g1.0));
            sum += taxicab;
        }
    }

    sum.to_string()
}

pub fn p2(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let input = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
        assert_eq!(p1(input), 374.to_string());
    }
}
