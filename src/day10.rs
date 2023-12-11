use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug, Copy, Clone, Default)]
enum Dir {
    #[default]
    Unknown,
    North,
    East,
    South,
    West,
}

impl Dir {
    fn opposite(self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
            Dir::Unknown => Dir::Unknown,
        }
    }
    fn from_cell(c: &Cell) -> Option<[Dir; 2]> {
        match c.char {
            '|' => Some([Dir::North, Dir::South]),
            '-' => Some([Dir::East, Dir::West]),
            'L' => Some([Dir::North, Dir::East]),
            'J' => Some([Dir::North, Dir::West]),
            '7' => Some([Dir::South, Dir::West]),
            'F' => Some([Dir::East, Dir::South]),
            'S' => Some([Dir::Unknown, Dir::Unknown]),
            '.' => None,
            _ => panic!("Invalid character"),
        }
    }
    fn offset(&self, pos: (usize, usize)) -> (usize, usize) {
        let o = match self {
            Dir::Unknown => (0, 0),
            Dir::North => (0, -1),
            Dir::East => (1, 0),
            Dir::South => (0, 1),
            Dir::West => (-1, 0),
        };
        ((pos.0 as i32 + o.0) as usize, (pos.1 as i32 + o.1) as usize)
    }
    fn connects(a: [Dir; 2], b: [Dir; 2]) -> bool {
        for dir1 in a {
            for dir2 in b {
                if dir1 == dir2.opposite() {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Cell {
    char: char,
    visited: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            char: '.',
            visited: false,
        }
    }
}

impl Cell {
    fn expand(self) -> Vec<Vec<Cell>> {
        let vecs = match self.char {
            '|' => vec![
                vec!['.', '|', '.'],
                vec!['.', '|', '.'],
                vec!['.', '|', '.'],
            ],
            '-' => vec![
                vec!['.', '.', '.'],
                vec!['-', '-', '-'],
                vec!['.', '.', '.'],
            ],
            'L' => vec![
                vec!['.', '|', '.'],
                vec!['.', 'L', '-'],
                vec!['.', '.', '.'],
            ],
            'J' => vec![
                vec!['.', '|', '.'],
                vec!['-', 'J', '.'],
                vec!['.', '.', '.'],
            ],
            '7' => vec![
                vec!['.', '.', '.'],
                vec!['-', '7', '.'],
                vec!['.', '|', '.'],
            ],
            'F' => vec![
                vec!['.', '.', '.'],
                vec!['.', 'F', '-'],
                vec!['.', '|', '.'],
            ],
            'S' => vec![
                vec!['.', '|', '.'],
                vec!['-', 'S', '-'],
                vec!['.', '|', '.'],
            ],
            '.' => vec![
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
            ],
            _ => panic!("Invalid char"),
        };
        let mut vecs = vecs
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| Cell {
                        char: c,
                        visited: false,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        vecs.iter_mut().flatten().for_each(|cell| {
            if matches!(cell.char, '|' | '-' | 'L' | 'J' | '7' | 'F' | 'S') {
                cell.visited = self.visited;
            }
        });
        vecs
    }
}
struct Map(Vec<Vec<Cell>>);

impl Map {
    fn get(&self, pos: (usize, usize)) -> Option<&Cell> {
        self.0.get(pos.1)?.get(pos.0)
    }
    fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut Cell> {
        self.0.get_mut(pos.1)?.get_mut(pos.0)
    }

    fn get_next_pipe(
        &self,
        pos: (usize, usize),
        current_dir: Dir,
    ) -> Option<(Dir, (usize, usize))> {
        let next_pos = current_dir.offset(pos);
        let next_dirs = Dir::from_cell(self.get(next_pos)?)?;
        let new_dir = map_dir(current_dir, next_dirs);
        Some((new_dir?, next_pos))
    }

    fn get_start(&self) -> ([Dir; 2], (usize, usize)) {
        let mut start = None;
        for (y, v) in self.0.iter().enumerate() {
            for (x, c) in v.iter().enumerate() {
                if c.char == 'S' {
                    start = Some((x, y));
                }
            }
        }
        let start = start.expect("No start found in map");
        let dirs = self.get_start_dirs(start);
        (dirs, start)
    }

    fn get_start_dirs(&self, pos: (usize, usize)) -> [Dir; 2] {
        let mut iter = [Dir::North, Dir::East, Dir::South, Dir::West]
            .into_iter()
            .filter_map(|dir| {
                let new_pos = dir.offset(pos);
                let next_dirs = Dir::from_cell(self.get(new_pos)?)?;
                if next_dirs.contains(&dir.opposite()) {
                    Some(dir)
                } else {
                    None
                }
            });
        [iter.next().unwrap(), iter.next().unwrap()]
    }

    fn fix_start(&mut self) {
        let (starting_dirs, starting_pos) = self.get_start();
        self.get_mut(starting_pos).unwrap().char = match starting_dirs {
            [Dir::North, Dir::South] => '|',
            [Dir::East, Dir::West] => '-',
            [Dir::North, Dir::East] => 'L',
            [Dir::North, Dir::West] => 'J',
            [Dir::South, Dir::West] => '7',
            [Dir::East, Dir::South] => 'F',
            _ => panic!("Invalid character"),
        };
    }

    fn get_unvisited_neighbors(
        &mut self,
        pos: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        [Dir::North, Dir::East, Dir::South, Dir::West]
            .into_iter()
            .filter_map(move |dir| {
                let new_pos = dir.offset(pos);
                let new_cell = self.get_mut(new_pos)?;
                if !new_cell.visited {
                    new_cell.visited = true;
                    if (new_pos.0 + 1) % 3 == 0 && (new_pos.1 + 1) % 3 == 0 && new_cell.char == '.'
                    {
                        new_cell.char = '@';
                    }
                    Some(new_pos)
                } else {
                    None
                }
            })
    }

    fn flood_fill(&mut self) -> u32 {
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        let mut count = 0;
        while let Some(pos) = q.pop_front() {
            if (pos.0 + 1) % 3 == 0 && (pos.1 + 1) % 3 == 0 {
                count += 1;
            }
            q.extend(self.get_unvisited_neighbors(pos));
        }
        count
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Map(value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|e| Cell {
                        char: e,
                        visited: false,
                    })
                    .collect()
            })
            .collect())
    }
}

fn map_dir(dir: Dir, dirs: [Dir; 2]) -> Option<Dir> {
    if dir.opposite() == dirs[0] {
        return Some(dirs[1]);
    }
    if dir.opposite() == dirs[1] {
        return Some(dirs[0]);
    }
    None
}

pub fn p1(s: &str) -> String {
    let map = Map::from(s);
    let start = map.get_start();
    let starting_dir = start.0[0];
    let start = start.1;
    let mut cur = Some((starting_dir, start));
    let mut count = 1;
    while let Some(next) = cur {
        cur = map.get_next_pipe(next.1, next.0);
        count += 1;
    }
    (count / 2).to_string()
}

pub fn p2(s: &str) -> String {
    let mut map = Map::from(s);
    let total = (map.0.len() * map.0[0].len()) as u32;
    let start = map.get_start();
    let starting_dir = start.0[0];
    let start = start.1;
    map.get_mut(start).unwrap().visited = true;
    let start = starting_dir.offset(start);
    let mut cur = Some((starting_dir, start));
    let mut pipe_count = 1;
    while let Some(next) = cur {
        pipe_count += 1;
        map.get_mut(next.1).unwrap().visited = true;
        cur = map.get_next_pipe(next.1, next.0);
    }
    map.fix_start();

    let map = map
        .0
        .into_iter()
        .flat_map(|vec| {
            let mut vec1 = vec![];
            let mut vec2 = vec![];
            let mut vec3 = vec![];
            for cell in vec.into_iter() {
                let vecs = cell.expand();
                vec1.extend(vecs[0].iter());
                vec2.extend(vecs[1].iter());
                vec3.extend(vecs[2].iter());
            }
            vec![vec1, vec2, vec3]
        })
        .collect();
    let map = Map(map);

    let mut new_map = vec![vec![
        Cell {
            char: '.',
            visited: false
        };
        map.0[0].len() + 2
    ]];
    new_map.extend(
        map.0
            .into_iter()
            .map(|vec| {
                let mut new_vec = vec![Cell::default()];
                new_vec.extend(vec);
                new_vec.push(Cell::default());
                new_vec
            })
            .collect::<Vec<Vec<Cell>>>(),
    );
    new_map.push(vec![
        Cell {
            char: '.',
            visited: false
        };
        new_map[0].len()
    ]);
    let mut map = Map(new_map);
    let outside_count = map.flood_fill();

    // for y in map.0.iter() {
    //     for c in y.iter() {
    //         print!("{}", c.char);
    //     }
    //     println!();
    // }
    (total - outside_count - pipe_count).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
        assert_eq!(p1(input), 4.to_string());
    }
    #[test]
    pub fn test_p1_2() {
        let input = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";
        assert_eq!(p1(input), 8.to_string());
    }
    #[test]
    pub fn test_p2() {
        let input = "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........";
        assert_eq!(p2(input), 4.to_string());
    }

    #[test]
    pub fn test_p2_2() {
        let input = "..........\n.S------7.\n.|F----7|.\n.||....||.\n.||....||.\n.|L-7F-J|.\n.|..||..|.\n.L--JL--J.\n..........";
        assert_eq!(p2(input), 4.to_string());
    }

    #[test]
    pub fn test_p2_3() {
        let input = "7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ";
        assert_eq!(p2(input), 4.to_string());
    }
}
