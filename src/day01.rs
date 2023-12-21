use onig::*;

pub fn p1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let c = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();

            format!("{}{}", c.first().unwrap(), c.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let re =
        Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|ten|1|2|3|4|5|6|7|8|9))")
            .unwrap();
    input
        .lines()
        .map(|line| {
            let c = re
                .captures_iter(line)
                .map(|cap| parse_number(cap.at(1).unwrap()))
                .collect::<Vec<_>>();

            format!("{}{}", c.first().unwrap(), c.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>()
        .to_string()
}

fn parse_number(cap: &str) -> u32 {
    let n = cap.parse::<u32>();
    match n {
        Ok(n) => n,
        Err(_) => match cap {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => panic!(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let test_input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(p1(test_input), 142.to_string());
    }

    #[test]
    pub fn test_p2() {
        let test_input =
            "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(p2(test_input), 281.to_string());
        assert_eq!(p2("eighthree"), 83.to_string());
        assert_eq!(p2("sevenine"), 79.to_string());
    }
}
