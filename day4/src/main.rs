use std::{fs, ops::RangeInclusive};

fn main() {
    let input = fs::read_to_string("day4/input").unwrap();
    let parsed = parse_input(input);
    println!("Answer 1: {}", day_one(&parsed));
    println!("Answer 2: {}", day_two(&parsed));
}

pub struct Section(RangeInclusive<u32>);

impl Section {
    fn contains_section(&self, other: &Section) -> bool {
        self.0.start() <= other.0.start() && self.0.end() >= other.0.end()
    }

    fn overlaps(&self, other: &Section) -> bool {
        self.0.contains(other.0.start())
            || self.0.contains(other.0.end())
            || other.contains_section(self)
    }
}

type Input = (Section, Section);

fn parse_input(input: String) -> Vec<Input> {
    peg::parser! {
        grammar parser() for str {
            rule num() -> u32
                = n:$(['0'..='9']+) { n.parse().expect("Invalid number!") }

            rule section() -> Section
                = start:num() "-" end:num() { Section(start..=end)}

            pub rule line() -> (Section, Section)
                = left:section() "," right:section() { (left, right) }
        }
    }

    input
        .lines()
        .map(parser::line)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn day_one(input: &[Input]) -> u32 {
    let mut num = 0;
    for (left, right) in input {
        if left.contains_section(right) || right.contains_section(left) {
            num += 1;
        }
    }
    num
}

fn day_two(input: &[Input]) -> u32 {
    let mut num = 0;
    for (left, right) in input {
        if left.overlaps(right) {
            num += 1;
        }
    }
    num
}

#[test]
fn test_part_two() {
    let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
    let parsed = parse_input(String::from(input));
    assert_eq!(day_two(&parsed), 4)
}
