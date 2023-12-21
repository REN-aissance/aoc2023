use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

struct Map {
    map: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn cycle(&mut self) -> usize {
        self.shift(Dir::North);
        self.shift(Dir::West);
        self.shift(Dir::South);
        self.shift(Dir::East);
        self.score()
    }

    pub fn shift(&mut self, dir: Dir) {
        match dir {
            Dir::North => {
                for i in 0..self.width {
                    let mut last_empty_space = None;
                    for j in 0..self.height {
                        self.check_one(j, i, dir, &mut last_empty_space)
                    }
                }
            }
            Dir::South => {
                for i in (0..self.width).rev() {
                    let mut last_empty_space = None;
                    for j in (0..self.height).rev() {
                        self.check_one(j, i, dir, &mut last_empty_space)
                    }
                }
            }
            Dir::West => {
                for j in 0..self.height {
                    let mut last_empty_space = None;
                    for i in 0..self.width {
                        self.check_one(j, i, dir, &mut last_empty_space)
                    }
                }
            }
            Dir::East => {
                for j in (0..self.height).rev() {
                    let mut last_empty_space = None;
                    for i in (0..self.width).rev() {
                        self.check_one(j, i, dir, &mut last_empty_space)
                    }
                }
            }
        }
    }

    fn check_one(&mut self, y: usize, x: usize, dir: Dir, last_empty_space: &mut Option<usize>) {
        let cur = y * self.width + x;
        match self.map[cur] {
            '#' => *last_empty_space = None,
            '.' => {
                if last_empty_space.is_none() {
                    *last_empty_space = Some(cur)
                }
            }
            'O' => {
                if let Some(i) = *last_empty_space {
                    self.map.swap(cur, i);
                    *last_empty_space = match dir {
                        Dir::North => Some(i + self.width),
                        Dir::South => Some(i - self.width),
                        Dir::East => Some(i - 1),
                        Dir::West => Some(i + 1),
                    };
                }
            }
            _ => panic!("Invalid char in map"),
        }
    }

    pub fn print(&self) {
        self.map.iter().enumerate().for_each(|(i, c)| {
            if i % self.width == 0 {
                println!();
            }
            print!("{}", c);
        });
        println!();
    }

    pub fn score(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if *c == 'O' {
                    Some(self.height - (i / self.width))
                } else {
                    None
                }
            })
            .sum()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let width = value.split_once('\n').unwrap().0.len();
        let height = value.lines().count();
        let map: Vec<char> = value.lines().flat_map(|line| line.chars()).collect();
        Map { map, width, height }
    }
}

pub fn p1(s: &str) -> String {
    let mut map = Map::from(s);
    map.shift(Dir::North);
    map.score().to_string()
}

pub fn p2(s: &str) -> String {
    const CYCLES: usize = 1_000_000_000;
    let mut cache: HashMap<Vec<char>, (usize, usize)> = HashMap::new();
    let mut map = Map::from(s);
    for ub in 0..CYCLES {
        if let Some((lb, _)) = cache.get(&map.map) {
            let cycle_length = ub - lb;
            let mut solution_idx = CYCLES % cycle_length;
            if solution_idx < *lb {
                solution_idx += cycle_length;
            }
            return cache
                .into_values()
                .find(|(x, _)| solution_idx == *x)
                .unwrap()
                .1
                .to_string();
        } else {
            cache.insert(map.map.clone(), (ub, map.score()));
            map.cycle();
        }
    }
    panic!("No cycle found")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let input = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";
        assert_eq!(p1(input), 136.to_string())
    }
    #[test]
    fn test_p2() {
        let input = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";
        assert_eq!(p2(input), 64.to_string())
    }
}
