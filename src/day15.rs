use regex::Regex;

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
struct Lens<'a> {
    name: &'a str,
    value: u8,
}

impl Lens<'_> {
    pub fn new(name: &str, value: u8) -> Lens<'_> {
        Lens { name, value }
    }
}

impl std::fmt::Display for Lens<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.name, self.value)
    }
}

struct HashMap<'a>(Vec<Vec<Lens<'a>>>);

impl<'a> HashMap<'a> {
    pub fn new() -> Self {
        HashMap(vec![Vec::default(); 256])
    }

    pub fn insert(&mut self, key: &'a str, value: Lens<'a>) {
        if let Some(lens) = self.0[Self::hash(key) as usize]
            .iter_mut()
            .find(|lens| lens.name == key)
        {
            lens.value = value.value;
        } else {
            self.0[Self::hash(key) as usize].push(value);
        }
    }

    pub fn remove(&mut self, key: &'a str) {
        self.0[Self::hash(key) as usize].retain(|e| e.name != key);
    }

    pub fn hash(s: &str) -> u8 {
        s.chars().map(|c| c as u8).fold(0_u8, |mut acc, n| {
            acc = acc.wrapping_add(n);
            acc = acc.wrapping_mul(17);
            acc
        })
    }

    pub fn print(&self) {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, list)| !list.is_empty())
            .for_each(|(i, list)| {
                print!("{}: ", i);
                list.iter()
                    .map(|e| e.to_string())
                    .intersperse(", ".to_string())
                    .for_each(|e| print!("{}", e));
                println!();
            })
    }
}

pub fn p1(s: &str) -> String {
    s.trim()
        .split(',')
        .map(HashMap::hash)
        .map(|n| n as u32)
        .sum::<u32>()
        .to_string()
}

pub fn p2(s: &str) -> String {
    let re = Regex::new(r"(\w+)([-=])(\d)?").unwrap();
    let mut hashmap = HashMap::new();
    re.captures_iter(s).for_each(|cap| {
        let (name, op, value) = (
            cap.get(1).unwrap().as_str(),
            cap.get(2).unwrap().as_str().chars().next().unwrap(),
            cap.get(3).map(|e| e.as_str().parse::<u8>().unwrap()),
        );
        match op {
            '=' => hashmap.insert(name, Lens::new(name, value.unwrap())),
            '-' => hashmap.remove(name),
            _ => panic!("Invalid operator"),
        }
    });
    hashmap
        .0
        .into_iter()
        .enumerate()
        .map(|(i, list)| {
            list.into_iter().enumerate().fold(0, |acc, (j, ele)| {
                acc + ((i + 1) * (j + 1) * ele.value as usize)
            })
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let expected = 1320;
        assert_eq!(p1(input), expected.to_string());
    }

    #[test]
    fn test_p2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let expected: usize = 145;
        assert_eq!(p2(input), expected.to_string());
    }
}
