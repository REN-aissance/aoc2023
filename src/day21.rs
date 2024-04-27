use std::collections::VecDeque;

const STEPS: usize = 64;
type Pos = (usize, usize);

pub struct Tile {
    c: char,
    visited: Option<usize>,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        let visited = match c {
            'S' => Some(0),
            _ => None,
        };
        Tile { c, visited }
    }
}

pub fn get_neighbors(cur: &Pos) -> impl Iterator<Item = Pos> + Clone + '_ {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(x, y)| Some((cur.0.checked_add_signed(x)?, cur.1.checked_add_signed(y)?)))
}

pub fn p1(input: &str) -> String {
    let mut map = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let cur = map
        .iter()
        .enumerate()
        .find_map(|(y, v)| {
            v.iter().enumerate().find_map(|(x, tile)| match tile.c {
                'S' => Some((x, y)),
                _ => None,
            })
        })
        .unwrap();

    let mut q = VecDeque::from([cur]);
    while let Some(pos) = q.pop_front() {
        let visited = map[pos.1][pos.0].visited.unwrap();
        if visited == STEPS {
            continue;
        }
        let neighbors = get_neighbors(&pos)
            .filter(|pos| {
                map.get(pos.1)
                    .and_then(|v| v.get(pos.0))
                    .is_some_and(|t| t.c != '#' && t.visited.is_none())
            })
            .collect::<Vec<_>>();
        for n in neighbors.clone() {
            map.get_mut(n.1)
                .and_then(|v| v.get_mut(n.0))
                .unwrap()
                .visited = Some(visited + 1);
            if visited % 2 == 0 {
                map.get_mut(n.1).and_then(|v| v.get_mut(n.0)).unwrap().c = 'o';
            }
        }
        q.extend(neighbors);
    }

    map.iter()
        .map(|v| v.iter().map(|t| t.c).collect::<String>())
        .for_each(|s| println!("{s}"));

    map.iter()
        .flatten()
        .filter(|tile| tile.visited.is_some_and(|b| b % 2 == 0))
        .count()
        .to_string()
}

pub fn p2(_input: &str) -> String {
    unimplemented!()
}
