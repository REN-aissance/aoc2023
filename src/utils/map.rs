#![allow(dead_code)]
use ahash::{HashMap, HashMapExt};
use ndarray::Array2;
use std::{collections::VecDeque, ops::Deref};

type Pos = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Free(char),
    Blocked,
}
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", **self)
    }
}
impl Deref for Tile {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        match self {
            Tile::Free(c) => c,
            Tile::Blocked => &'#',
        }
    }
}

#[derive(Clone, Debug)]
struct Map(Array2<Tile>);
impl Deref for Map {
    type Target = Array2<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = self.dim().0;
        let s = self
            .windows((w, 1))
            .into_iter()
            .map(|v| v.into_iter().map(|&c| *c).collect::<String>())
            .intersperse(String::from("\n"))
            .collect::<String>();
        writeln!(f, "{}", s)
    }
}
impl Map {
    pub fn new(s: &str, free_tiles: &[char]) -> Self {
        let y = s.lines().count();
        let x = s.lines().next().unwrap().chars().count();
        let map = s
            .lines()
            .flat_map(|line| {
                line.chars().map(|e| match free_tiles.contains(&e) {
                    true => Tile::Free(e),
                    false => Tile::Blocked,
                })
            })
            .collect::<Vec<_>>();
        let map = Array2::from_shape_vec((x, y), map).unwrap();
        let map = map.reversed_axes();
        Map(map)
    }
    pub fn shortest_path_bfs(&mut self, start: Pos, end: Pos) -> Vec<Pos> {
        let mut visited = HashMap::new();
        let mut q = VecDeque::from_iter(Some((start, None)));
        while let Some((cur, prev)) = q.pop_front() {
            visited.insert(cur, prev);
            q.extend(
                self.get_neighbors(cur)
                    .into_iter()
                    .filter(|p| !visited.contains_key(p))
                    .map(|e| (e, Some(cur))),
            )
        }
        let mut cur = end;
        let mut path = vec![];
        while let Some(Some(prev)) = visited.get(&cur) {
            cur = *prev;
            path.push(*prev);
        }
        path.reverse();
        path
    }
    pub fn get_neighbors(&self, p: Pos) -> Vec<Pos> {
        [(0, -1), (1, 0), (0, 1), (-1, 0)]
            .into_iter()
            .filter_map(move |p2| {
                let (w, h) = self.dim();
                let p = (p.0.checked_add_signed(p2.0)?, p.1.checked_add_signed(p2.1)?);
                if p.0 >= w || p.1 >= h {
                    return None;
                }
                match self[p] {
                    Tile::Free(_) => Some(p),
                    Tile::Blocked => None,
                }
            })
            .collect()
    }
}
