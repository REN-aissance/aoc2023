use crate::utils::range::Range;
use ahash::HashMap;
use ranges::{GenericRange, Ranges};
use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Test {
    Lt,
    Gt,
}
impl From<&str> for Test {
    fn from(value: &str) -> Self {
        match value {
            "<" => Test::Lt,
            ">" => Test::Gt,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Label {
    X,
    M,
    A,
    S,
}
impl From<&str> for Label {
    fn from(value: &str) -> Self {
        match value {
            "x" => Label::X,
            "m" => Label::M,
            "a" => Label::A,
            "s" => Label::S,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Condition {
    label: Label,
    test: Test,
    value: usize,
    dest: String,
}
impl Condition {
    pub fn map(&self, part: &Part) -> Option<&str> {
        let lhs = part.get(self.label);
        match self.test {
            Test::Lt if lhs < self.value => Some(&self.dest),
            Test::Gt if lhs > self.value => Some(&self.dest),
            _ => None,
        }
    }
    pub fn map_range(&self, mut part_range: PartRange) -> Option<(PartRange, &str)> {
        let rhs = match self.test {
            Test::Lt if self.value > 0 => GenericRange::from(0..=(self.value - 1)),
            Test::Gt if self.value < 4000 => GenericRange::from((self.value + 1)..=4000),
            _ => return None,
        };
        let lhs = part_range.get(self.label);
        *lhs = lhs.to_owned().intersect(rhs);
        match lhs.is_empty() {
            true => None,
            false => Some((part_range, &self.dest)),
        }
    }
    pub fn diff_range(&self, mut part_range: PartRange) -> Option<PartRange> {
        let rhs = match self.test {
            Test::Lt if self.value > 0 => GenericRange::from(0..=(self.value - 1)),
            Test::Gt if self.value < 4000 => GenericRange::from((self.value + 1)..=4000),
            _ => return None,
        };
        let lhs = part_range.get(self.label);
        *lhs = lhs.to_owned().difference(rhs);
        match lhs.is_empty() {
            true => None,
            false => Some(part_range),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Workflow {
    conditions: Vec<Condition>,
    fallback: String,
}
impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let fallback_re = Regex::new(r",(\w+)}").unwrap();
        let workflow_re = Regex::new(r"([xmas])([<>])(\d+):(\w+)").unwrap();

        let fallback = fallback_re.captures_iter(value).next().unwrap()[1].to_string();
        let conditions = workflow_re
            .captures_iter(value)
            .map(|cap| Condition {
                label: cap[1].into(),
                test: cap[2].into(),
                value: cap[3].parse().unwrap(),
                dest: cap[4].into(),
            })
            .collect::<Vec<_>>();
        Workflow {
            conditions,
            fallback,
        }
    }
}
impl Workflow {
    pub fn map(&self, part: &Part) -> &str {
        self.conditions
            .iter()
            .find_map(|c| c.map(part))
            .unwrap_or(&self.fallback)
    }
    pub fn map_range(&self, part_range: PartRange) -> Vec<(PartRange, &str)> {
        let mut cur = Some(part_range);
        let mut v = self
            .conditions
            .iter()
            .filter_map(|c| match cur {
                Some(ref range) => {
                    let t = c.map_range(range.clone());
                    cur = c.diff_range(range.clone());
                    t
                }
                None => None,
            })
            .collect::<Vec<_>>();
        if let Some(part_range) = cur {
            v.push((part_range, &self.fallback));
        }
        v
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Part(Vec<usize>);
impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let part_re = Regex::new(r"\d+").unwrap();
        let values = part_re
            .find_iter(value)
            .map(|f| f.as_str().parse().unwrap())
            .collect::<Vec<_>>();
        Self(values)
    }
}
impl Part {
    pub fn score(&self) -> usize {
        self.0.iter().sum()
    }
    fn get(&self, label: Label) -> usize {
        match label {
            Label::X => self.0[0],
            Label::M => self.0[1],
            Label::A => self.0[2],
            Label::S => self.0[3],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PartRange(Vec<Ranges<usize>>);
impl Default for PartRange {
    fn default() -> Self {
        Self(vec![Ranges::from(1..=4000); 4])
    }
}
impl PartRange {
    fn score(&self) -> u64 {
        self.0.iter().map(|range| range.span() as u64).product()
    }
    fn get(&mut self, label: Label) -> &mut Ranges<usize> {
        match label {
            Label::X => &mut self.0[0],
            Label::M => &mut self.0[1],
            Label::A => &mut self.0[2],
            Label::S => &mut self.0[3],
        }
    }
}

fn prep_input(s: &str) -> (Vec<Part>, HashMap<&str, Workflow>) {
    let (workflows, parts) = s.split_once("\n\n").unwrap();
    let parts: Vec<Part> = parts.lines().map(Part::from).collect();
    let workflows = workflows
        .lines()
        .map(|line| {
            let (name, s) = line.split_once('{').unwrap();
            (name, Workflow::from(s))
        })
        .collect::<HashMap<&str, Workflow>>();
    (parts, workflows)
}

pub fn p1(s: &str) -> String {
    let (parts, workflows) = prep_input(s);

    let mut sum = 0;
    for part in parts {
        let mut cur = "in";
        while !matches!(cur, "A" | "R") {
            cur = workflows.get(cur).unwrap().map(&part);
        }
        if cur == "A" {
            sum += part.score();
        }
    }

    sum.to_string()
}

pub fn p2(s: &str) -> String {
    let (_, workflows) = prep_input(s);

    let mut sum = 0;
    let mut stack = vec![(PartRange::default(), "in")];
    while let Some((part_range, label)) = stack.pop() {
        match label {
            "A" => sum += part_range.score(),
            "R" => continue,
            _ => stack.extend(workflows.get(label).unwrap().map_range(part_range)),
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_p1() {
        assert_eq!(p1(TEST), 19114.to_string());
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(TEST), 167409079868000u64.to_string());
    }
}
