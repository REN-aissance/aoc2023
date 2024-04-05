#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(usize)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cell {
    char: char,
    visited: bool,
    visited_from: [bool; 4],
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        let visited = false;
        let visited_from = [false; 4];
        Cell {
            char: value,
            visited,
            visited_from,
        }
    }
}

#[derive(Clone)]
struct Map {
    map: Vec<Cell>,
    height: usize,
    width: usize,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let height = value.lines().count();
        let width = value.lines().next().unwrap().len();
        let map = value
            .lines()
            .flat_map(|line| line.chars().map(Cell::from))
            .collect::<Vec<_>>();
        Map { map, height, width }
    }
}

impl Map {
    pub fn score(&self) -> usize {
        self.map.iter().filter(|c| c.visited).count()
    }

    pub fn send_ray(&mut self, mut pos: usize, mut dir: Dir) {
        while let Some(cur) = self.map.get_mut(pos) {
            if cur.visited_from[dir as usize] {
                break;
            }
            cur.visited = true;
            cur.visited_from[dir as usize] = true;
            match (cur.char, dir) {
                ('\\', Dir::North) => {
                    dir = Dir::West;
                }
                ('\\', Dir::East) => {
                    dir = Dir::South;
                }
                ('\\', Dir::South) => {
                    dir = Dir::East;
                }
                ('\\', Dir::West) => {
                    dir = Dir::North;
                }
                ('/', Dir::North) => {
                    dir = Dir::East;
                }
                ('/', Dir::South) => {
                    dir = Dir::West;
                }
                ('/', Dir::East) => {
                    dir = Dir::North;
                }
                ('/', Dir::West) => {
                    dir = Dir::South;
                }
                ('-', Dir::North | Dir::South) => {
                    if let Some(pos) = self.offset_by(pos, Dir::East) {
                        self.send_ray(pos, Dir::East);
                    }
                    if let Some(pos) = self.offset_by(pos, Dir::West) {
                        self.send_ray(pos, Dir::West);
                    }
                    break;
                }
                ('|', Dir::East | Dir::West) => {
                    if let Some(pos) = self.offset_by(pos, Dir::North) {
                        self.send_ray(pos, Dir::North);
                    }
                    if let Some(pos) = self.offset_by(pos, Dir::South) {
                        self.send_ray(pos, Dir::South);
                    }
                    break;
                }
                ('|' | '-' | '.', _) => (),
                _ => panic!("Something went wrong"),
            }
            if let Some(next) = self.offset_by(pos, dir) {
                pos = next;
            } else {
                break;
            }
        }
    }

    pub fn offset_by(&self, pos: usize, dir: Dir) -> Option<usize> {
        match dir {
            Dir::North => pos.checked_sub(self.width),
            Dir::East => {
                if pos % self.width != self.width - 1 {
                    pos.checked_add(1)
                } else {
                    None
                }
            }
            Dir::South => pos.checked_add(self.width),
            Dir::West => {
                if pos % self.width != 0 {
                    pos.checked_sub(1)
                } else {
                    None
                }
            }
        }
    }

    pub fn print(&self) {
        self.map.iter().enumerate().for_each(|(i, c)| {
            if i % self.width == 0 {
                println!();
            }
            let c = match c.visited {
                true => '#',
                false => c.char,
            };
            print!("{}", c);
        });
        println!();
    }
}

pub fn p1(s: &str) -> String {
    let mut map = Map::from(s);
    // map.print();
    map.send_ray(0, Dir::East);
    // map.print();
    map.score().to_string()
}

pub fn p2(s: &str) -> String {
    let map = Map::from(s);
    (0..map.width)
        //Top edge
        .map(|i| {
            let mut map = map.clone();
            map.send_ray(i, Dir::South);
            map.score()
        })
        //Left edge
        .chain((0..map.map.len()).step_by(map.width).map(|i| {
            let mut map = map.clone();
            map.send_ray(i, Dir::East);
            map.score()
        }))
        //Right edge
        .chain(
            ((map.width - 1)..map.map.len())
                .step_by(map.width)
                .map(|i| {
                    let mut map = map.clone();
                    map.send_ray(i, Dir::West);
                    map.score()
                }),
        )
        //Bottom Edge
        .chain(((map.map.len() - map.width)..map.map.len()).map(|i| {
            let mut map = map.clone();
            map.send_ray(i, Dir::North);
            map.score()
        }))
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";
        assert_eq!(p1(input), 46.to_string());
    }
    #[test]
    fn test_p1_2() {
        let input = "..\\\n...\n...";
        assert_eq!(p1(input), 5.to_string());
    }
    #[test]
    fn test_p1_3() {
        let input = ".\\.\n.\\.\n...";
        assert_eq!(p1(input), 4.to_string());
    }
    #[test]
    fn test_p1_4() {
        let input = ".\\.\n./.\n...";
        assert_eq!(p1(input), 4.to_string());
    }

    #[test]
    fn test_p2() {
        let input = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";
        assert_eq!(p2(input), 51.to_string());
    }
}
