pub fn p1(s: &str) -> String {
    let mut s = s.lines().map(|line| {
        line.split_once(':')
            .unwrap()
            .1
            .trim()
            .split_ascii_whitespace()
            .map(|e| e.parse::<f32>().unwrap())
    });
    let s = s.next().unwrap().zip(s.next().unwrap());
    s.map(|(duration, record)| {
        let start =
            (((-duration + (duration.powi(2) - (4. * record)).sqrt()) / -2.) + 1.).floor() as i32;
        let end =
            (((-duration - (duration.powi(2) - (4. * record)).sqrt()) / -2.) - 1.).ceil() as i32;
        (end + 1) - start
    })
    .product::<i32>()
    .to_string()
}

pub fn p2(s: &str) -> String {
    let mut s = s.lines().map(|line| {
        let mut s = line.split_once(':').unwrap().1.to_owned();
        s.retain(|c| !c.is_ascii_whitespace());
        s.parse::<f64>().unwrap()
    });
    let (duration, record) = (s.next().unwrap(), s.next().unwrap());
    let start =
        (((-duration + (duration.powi(2) - (4. * record)).sqrt()) / -2.) + 1.).floor() as u32;
    let end = (((-duration - (duration.powi(2) - (4. * record)).sqrt()) / -2.) - 1.).ceil() as u32;
    ((end + 1) - start).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        assert_eq!(p1(input), 288.to_string());
    }
    #[test]
    pub fn test_p2() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        assert_eq!(p2(input), 71503.to_string());
    }
}
