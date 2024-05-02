//Insanely high difficulty level compared to previous years. This is day 5????
//What was I doing with all these generics???
use num::CheckedSub;

#[derive(Debug)]
pub struct Map<T> {
    dest: T,
    source: T,
    len: T,
}

impl<A, B> FromIterator<A> for Map<B>
where
    A: Into<B>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Map {
            dest: iter.next().unwrap().into(),
            source: iter.next().unwrap().into(),
            len: iter.next().unwrap().into(),
        }
    }
}

impl<T> Map<T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + PartialOrd
        + Ord
        + std::ops::AddAssign
        + CheckedSub
        + Copy,
{
    #[cfg(test)]
    fn new(dest: T, source: T, len: T) -> Self {
        Map { dest, source, len }
    }
    fn map_single(&self, n: T) -> Option<T> {
        if self.source <= n && n <= self.source + self.len {
            match self.dest.cmp(&self.source) {
                std::cmp::Ordering::Less => Some(n.checked_sub(&(self.source - self.dest))?),
                _ => Some(n + (self.dest - self.source)),
            }
        } else {
            None
        }
    }
    fn map_range(&self, range: (T, T)) -> Option<(T, T)> {
        let mut lb = range.0.max(self.source);
        let mut ub = (range.0 + range.1).min(self.source + self.len);
        if lb >= ub {
            return None;
        }

        if self.source > self.dest {
            lb = lb.checked_sub(&(self.source - self.dest))?;
            ub = ub.checked_sub(&(self.source - self.dest))?;
        } else {
            lb += self.dest - self.source;
            ub += self.dest - self.source;
        }
        Some((lb, ub - lb))
    }
    fn remainder(&self, seed_range: (T, T)) -> Vec<(T, T)> {
        let mut out = vec![];
        if seed_range.0 < self.source {
            out.push((seed_range.0, self.source - seed_range.0));
        }
        if seed_range.0 + seed_range.1 > self.source + self.len {
            out.push((
                self.source + self.len,
                (seed_range.0 + seed_range.1) - (self.source + self.len),
            ));
        }

        out
    }
}

pub fn p1(s: &str) -> String {
    let mut s = s.split("\n\n");
    let mut seeds: Vec<u64> = parse_seeds(&mut s);
    let maps = parse_maps(s);

    for seed in seeds.iter_mut() {
        for category in &maps {
            for map in category {
                if let Some(new_seed) = map.map_single(*seed) {
                    // print!("{} ", seed);
                    *seed = new_seed;
                    break;
                }
            }
        }
        // println!();
    }

    seeds.into_iter().min().unwrap().to_string()
}

fn parse_maps<T: std::str::FromStr>(s: std::str::Split<'_, &str>) -> Vec<Vec<Map<T>>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let maps = s
        .map(|category| {
            let mut category = category.lines();
            category.next(); //skip label
            category
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|e| e.parse::<T>().unwrap())
                        .collect::<Map<T>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    maps
}

fn parse_seeds<T: std::str::FromStr>(s: &mut std::str::Split<'_, &str>) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let seeds = s
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|e| e.parse().unwrap())
        .collect::<Vec<_>>();
    seeds
}

pub fn p2(s: &str) -> String {
    let mut s = s.split("\n\n");
    let mut seed_ranges = vec![];
    let mut seed_iter = s
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace();
    let maps = parse_maps(s);

    while let Some(seed_p1) = seed_iter.next() {
        if let Some(seed_p2) = seed_iter.next() {
            seed_ranges.push((seed_p1.parse().unwrap(), seed_p2.parse().unwrap()));
        }
    }

    let mut stack = seed_ranges;
    for category in maps.iter() {
        let mut mapped_seeds = vec![];
        while let Some(seed_range) = stack.pop() {
            //Try each map on seed_range. Map in place if never mapped
            let mut mapped = false;
            for map in category {
                if let Some(mapped_seed_range) = map.map_range(seed_range) {
                    mapped_seeds.push(mapped_seed_range); //save mapped range for next category
                    stack.extend(map.remainder(seed_range)); //return unmapped portion to stack
                    mapped = true;
                }
            }
            if !mapped {
                mapped_seeds.push(seed_range);
            }
        }
        stack = mapped_seeds;
    }

    stack
        .into_iter()
        .fold(u64::MAX, |acc, e| acc.min(e.0))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";
        assert_eq!(p1(input), 35.to_string());
    }

    #[test]
    pub fn test_p2() {
        let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";
        assert_eq!(p2(input), 46.to_string());
    }

    #[test]
    pub fn t1() {
        let map = Map::new(52, 50, 48);
        assert_eq!(map.map_range((79, 14)), Some((81, 14)))
    }

    #[test]
    pub fn t2() {
        let map = Map::new(0, 50, 5);
        assert_eq!(map.map_range((50, 10)), Some((0, 5)))
    }

    #[test]
    pub fn t3() {
        let map = Map::new(0, 50, 5);
        assert_eq!(map.map_range((45, 6)), Some((0, 1)))
    }

    #[test]
    pub fn t4() {
        let map = Map::new(45, 77, 23);
        assert_eq!(map.map_range((74, 14)), Some((45, 11)))
    }

    #[test]
    pub fn t5() {
        let map = Map::new(45, 77, 23);
        assert_eq!(map.remainder((74, 14)), vec![(74, 3)]);
    }
}
