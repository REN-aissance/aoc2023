use regex::Regex;

pub fn p1(input: &str) -> String {
    let re = Regex::new(r"^\D*(\d)?.*(\d)\D*$").unwrap();
    input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|caps| {
            let d2 = &caps[2];
            let d1 = caps.get(1).map(|e| e.as_str()).unwrap_or(d2);
            format!("{}{}", d1, d2).parse::<u32>().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let input = input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    p1(&input)
}