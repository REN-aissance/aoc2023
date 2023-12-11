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
    fn from_char(c: &char) -> Option<[Dir; 2]> {
        match c {
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

struct Map(Vec<Vec<char>>);

impl Map {
    fn find_start(&self) -> (usize, usize) {
        for (y, v) in self.0.iter().enumerate() {
            for (x, c) in v.iter().enumerate() {
                if c == &'S' {
                    return (x, y);
                }
            }
        }
        panic!("No start in map");
    }
    fn get(&self, pos: (usize, usize)) -> Option<&char> {
        self.0.get(pos.1)?.get(pos.0)
    }

    fn get_neighbor(&self, pos: (usize, usize), current_dir: Dir) -> Option<(Dir, (usize, usize))> {
        let next_pos = current_dir.offset(pos);
        let next_dirs = Dir::from_char(self.get(next_pos)?)?;
        let new_dir = map_dir(current_dir, next_dirs);
        Some((new_dir?, next_pos))
    }

    fn get_start_dir(&self, pos: (usize, usize)) -> Dir {
        [Dir::North, Dir::East, Dir::South, Dir::West]
            .into_iter()
            .find_map(|dir| {
                let new_pos = dir.offset(pos);
                let next_dirs = Dir::from_char(self.get(new_pos)?)?;
                if next_dirs.contains(&dir.opposite()) {
                    Some(dir)
                } else {
                    None
                }
            })
            .unwrap()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Map(value.lines().map(|line| line.chars().collect()).collect())
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
    let start = map.find_start();
    let starting_dir = map.get_start_dir(start);
    let start = starting_dir.offset(start);
    let mut cur = Some((map.get_start_dir(start), start));
    let mut count = 1;
    while let Some(next) = cur {
        cur = map.get_neighbor(next.1, next.0);
        count += 1;
    }
    (count / 2).to_string()
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
}
