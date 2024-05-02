pub fn _p1(_s: &str) -> String {
    0.to_string()
}

pub fn _p2(_s: &str) -> String {
    0.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "";
        assert_eq!(_p1(input), 0.to_string());
    }

    #[test]
    fn test_p2() {
        let input = "";
        assert_eq!(_p2(input), 0.to_string());
    }
}
