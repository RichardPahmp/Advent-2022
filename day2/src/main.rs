use std::fs;

fn main() {
    println!("Answer 1: {}", part_one(&parse_input_one()));
    println!("Answer 2: {}", part_two(&parse_input_two()));
}

#[derive(Debug, Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn point_value(self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn from_char(ch: char) -> Self {
        match ch {
            'A' | 'X' => Choice::Rock,
            'B' | 'Y' => Choice::Paper,
            'C' | 'Z' => Choice::Scissors,
            _ => panic!("Invalid character!"),
        }
    }

    fn fight(self, other: Choice) -> MatchResult {
        use Choice::*;
        use MatchResult::*;
        match (self, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Win,
            (Paper, Paper) | (Rock, Rock) | (Scissors, Scissors) => Draw,
            _ => Loss,
        }
    }

    fn beats(self) -> Choice {
        use Choice::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn loses_to(self) -> Choice {
        use Choice::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl MatchResult {
    fn point_value(self) -> i32 {
        match self {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Loss => 0,
        }
    }

    fn from_char(ch: char) -> Self {
        use MatchResult::*;
        match ch {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("Invalid character!"),
        }
    }
}

fn parse_input_one() -> Vec<(Choice, Choice)> {
    let input = fs::read_to_string("day2/input").unwrap();

    peg::parser! {
        grammar parser() for str {
            rule abc() -> Choice
                = ch:['A' | 'B' | 'C'] { Choice::from_char(ch)}

            rule xyz() -> Choice
                = ch:['X' | 'Y' | 'Z'] { Choice::from_char(ch)}

            pub rule line() -> (Choice, Choice)
                = first:abc() " " second: xyz() { (first, second)}
        }
    }

    let result: Result<_, _> = input.lines().map(parser::line).collect();
    result.unwrap()
}

fn parse_input_two() -> Vec<(Choice, MatchResult)> {
    let input = fs::read_to_string("day2/input").unwrap();

    peg::parser! {
        grammar parser() for str {
            rule abc() -> Choice
                = ch:['A' | 'B' | 'C'] { Choice::from_char(ch)}

            rule xyz() -> MatchResult
                = ch:['X' | 'Y' | 'Z'] { MatchResult::from_char(ch)}

            pub rule line() -> (Choice, MatchResult)
                = first:abc() " " second: xyz() { (first, second)}
        }
    }

    input
        .lines()
        .map(parser::line)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn part_one(input: &[(Choice, Choice)]) -> i32 {
    let mut total = 0;
    for (their_choice, my_choice) in input {
        let match_result = my_choice.fight(*their_choice);
        total += my_choice.point_value() + match_result.point_value();
    }
    total
}

fn part_two(input: &[(Choice, MatchResult)]) -> i32 {
    input
        .iter()
        .map(|(their_choice, match_result)| {
            match_result.point_value()
                + match match_result {
                    MatchResult::Win => their_choice.loses_to().point_value(),
                    MatchResult::Loss => their_choice.beats().point_value(),
                    MatchResult::Draw => their_choice.point_value(),
                }
        })
        .sum()
}
