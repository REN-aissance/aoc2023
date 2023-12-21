use regex::Regex;

pub fn p1(s: &str) -> String {
    let mut sum = 0;
    let re = Regex::new(r"(\d+) (\w)").unwrap();
    s.lines().enumerate().for_each(|(i, line)| {
        let mut counts = [0, 0, 0];
        for split in line.split(';') {
            for cap in re.captures_iter(split) {
                let color = match &cap[2] {
                    "r" => 0,
                    "g" => 1,
                    "b" => 2,
                    _ => panic!(),
                };
                let quantity = cap[1].parse::<u32>().unwrap();
                counts[color] = quantity.max(counts[color]);
            }
        }
        if counts[0] <= 12 && counts[1] <= 13 && counts[2] <= 14 {
            sum += i + 1
        }
    });
    sum.to_string()
}

pub fn p2(s: &str) -> String {
    let mut sum = 0;
    let re = Regex::new(r"(\d+) (\w)").unwrap();
    s.lines().for_each(|line| {
        let mut counts = [0, 0, 0];
        for split in line.split(';') {
            for cap in re.captures_iter(split) {
                let color = match &cap[2] {
                    "r" => 0,
                    "g" => 1,
                    "b" => 2,
                    _ => panic!(),
                };
                let quantity = cap[1].parse::<u32>().unwrap();
                counts[color] = quantity.max(counts[color]);
            }
        }
        sum += counts.into_iter().product::<u32>();
    });
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let test = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = 8.to_string();

        assert_eq!(p1(test), expected);
    }

    #[test]
    pub fn test_p2() {
        let test = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = 2286.to_string();

        assert_eq!(p2(test), expected);
    }
}
