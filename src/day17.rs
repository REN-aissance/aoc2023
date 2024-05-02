use std::collections::{BTreeSet, HashMap};

const DIRECTIONS: [Dir; 4] = [Dir::North, Dir::South, Dir::East, Dir::West];

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn offset_by(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Dir::North => Some((pos.0, pos.1.checked_sub(1)?)),
            Dir::South => Some((pos.0, pos.1.checked_add(1)?)),
            Dir::East => Some((pos.0.checked_add(1)?, pos.1)),
            Dir::West => Some((pos.0.checked_sub(1)?, pos.1)),
        }
    }
    fn opposite(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct State {
    cost: Option<u32>,
    pos: (usize, usize),
    dir: Dir,
    repeats: u32,
}

impl State {
    pub fn to_key(&self) -> Key {
        Key {
            pos: self.pos,
            dir: self.dir,
            repeats: self.repeats,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Key {
    pos: (usize, usize),
    dir: Dir,
    repeats: u32,
}

struct Map(Vec<Vec<u32>>);

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Map(value
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect())
    }
}

impl Map {
    fn get_cost(&self, pos: (usize, usize)) -> Option<u32> {
        self.0.get(pos.1)?.get(pos.0).copied()
    }

    fn modified_dijkstra(
        &self,
        start: (usize, usize),
        end: (usize, usize),
        min_steps: u32,
        max_steps: u32,
    ) -> Option<u32> {
        let mut distances: HashMap<Key, (Option<u32>, Option<Key>)> = HashMap::new();
        let mut frontier: BTreeSet<State> = BTreeSet::new();
        let mut end_key = None;
        let mut shortest_path_len = None;

        let starting_states = [
            State {
                cost: Some(0),
                pos: start,
                dir: Dir::East,
                repeats: 0,
            },
            State {
                cost: Some(0),
                pos: start,
                dir: Dir::South,
                repeats: 0,
            },
        ];

        for s in starting_states {
            frontier.insert(s.clone());
            distances.insert(s.to_key(), (s.cost, None));
        }

        while let Some(State {
            cost,
            pos,
            dir,
            repeats,
        }) = frontier.pop_first()
        {
            //end condition
            if pos == end && repeats >= min_steps {
                end_key = Some(Key { pos, dir, repeats });
                shortest_path_len = cost;
                break;
            }

            //previous was better
            let old_key = Key { pos, dir, repeats };
            if distances.contains_key(&old_key)
                && distances[&old_key]
                    .0
                    .zip(cost)
                    .map(|(old_dist, new_dist)| old_dist < new_dist)
                    .unwrap_or_default()
            {
                continue;
            }
            DIRECTIONS
                .into_iter()
                .filter(|&next_dir| next_dir != dir.opposite())
                .filter(|&next_dir| {
                    if repeats < min_steps {
                        next_dir == dir
                    } else {
                        true
                    }
                })
                .for_each(|next_dir| {
                    if let Some(new_pos) = next_dir.offset_by(pos) {
                        if let Some(new_cost) = cost.zip(self.get_cost(new_pos)).map(|(a, b)| a + b)
                        {
                            let new_repeats = if dir == next_dir { repeats + 1 } else { 1 };

                            let next_state = State {
                                pos: new_pos,
                                dir: next_dir,
                                cost: Some(new_cost),
                                repeats: new_repeats,
                            };

                            //New state is valid
                            if new_repeats <= max_steps
                                && (!distances.contains_key(&next_state.to_key())
                                    || distances[&next_state.to_key()]
                                        .0
                                        .is_some_and(|old_cost| new_cost < old_cost))
                            {
                                frontier.insert(next_state.clone());
                                distances
                                    .insert(next_state.to_key(), (Some(new_cost), Some(old_key)));
                            }
                        }
                    }
                })
        }

        //Path reconstruction
        let mut cur = end_key.unwrap();
        let mut path = vec![cur];
        while let Some(Some(prev)) = distances.get(&cur).map(|(_, val)| val) {
            path.push(*prev);
            cur = *prev;
        }

        // Path printing
        // for (y, v) in self.0.iter().enumerate() {
        //     for (x, _) in v.iter().enumerate() {
        //         let c = if let Some(key) = path.iter().find(|k| k.pos == (x, y)) {
        //             match key.dir {
        //                 Dir::North => '^',
        //                 Dir::South => 'V',
        //                 Dir::East => '>',
        //                 Dir::West => '<',
        //             }
        //         } else {
        //             '.'
        //         };
        //         print!("{}", c);
        //     }
        //     println!();
        // }
        // println!();

        shortest_path_len
    }
}

pub fn p1(s: &str) -> String {
    let map = Map::from(s);
    // map.print();
    map.modified_dijkstra((0, 0), (map.0[0].len() - 1, map.0.len() - 1), 0, 3)
        .unwrap()
        .to_string()
}

pub fn p2(s: &str) -> String {
    let map = Map::from(s);
    // map.print();
    map.modified_dijkstra((0, 0), (map.0[0].len() - 1, map.0.len() - 1), 4, 10)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533";
        assert_eq!(p1(input), 102.to_string());
    }

    #[test]
    fn test_p1_extras() {
        // Test cases that should pass
        let input_2 = "111\n111\n111";
        assert_eq!(p1(input_2), 4.to_string());

        let input_3 = "1";
        assert_eq!(p1(input_3), 0.to_string());

        let input_4 = "11";
        assert_eq!(p1(input_4), 1.to_string());

        let input_5 = "111";
        assert_eq!(p1(input_5), 2.to_string());

        let input_6 = "12\n34";
        assert_eq!(p1(input_6), 6.to_string());

        let input_7 = "13\n24\n56";
        assert_eq!(p1(input_7), 12.to_string());

        let input_9 = "11111\n11161";
        assert_eq!(p1(input_9), 7.to_string());

        let input_10 = "11111\n99991";
        assert_eq!(p1(input_10), 13.to_string());

        let input_11 = "\n11119\n99919\n99919\n11119\n19999\n11119\n99911";
        assert_eq!(p1(input_11), 16.to_string());

        let input_12 = "11\n11";
        assert_eq!(p1(input_12), 2.to_string());

        // Test case that should panic
        let input_panic = "111111";
        assert!(std::panic::catch_unwind(|| p1(input_panic)).is_err());
    }

    #[test]
    fn test_p2() {
        let input = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533";
        assert_eq!(p2(input), 94.to_string());
    }

    #[test]
    fn test_p2_2() {
        let input = "111111111111\n999999999991\n999999999991\n999999999991\n999999999991";
        assert_eq!(p2(input), 71.to_string());
    }

    #[test]
    fn test_p2_3() {
        let input = "19999\n19999\n19999\n19999\n11111";
        assert_eq!(p2(input), 8.to_string());
    }
}
