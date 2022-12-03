use std::{collections::HashSet, fs};

fn main() {
    let input = parse_input();
    println!("Answer 1: {}", day_one(&input));
    println!("Answer 2: {}", day_two(&input));
}

fn parse_input() -> Vec<String> {
    let input = fs::read_to_string("day3/input").unwrap();
    input.lines().map(String::from).collect()
}

fn score(ch: &char) -> u32 {
    match ch {
        'a'..='z' => u32::from(*ch) - 96,
        'A'..='Z' => u32::from(*ch) - (64 - 26),
        _ => panic!("Invalid letter!"),
    }
}

fn day_one(input: &[String]) -> u32 {
    let mut sum = 0;
    for line in input {
        let (first, second) = line.split_at(line.len() / 2);
        let first_set: HashSet<char> = first.chars().collect();
        let second_set: HashSet<char> = second.chars().collect();
        let intersection: &char = first_set.intersection(&second_set).last().unwrap();
        sum += score(intersection);
    }
    sum
}

fn day_two(input: &[String]) -> u32 {
    let mut sum = 0;
    for group in input.chunks_exact(3) {
        let sets: Vec<HashSet<char>> = group.iter().map(|line| line.chars().collect()).collect();
        let intersection = sets[0]
            .intersection(&sets[1])
            .filter(|ch| sets[2].contains(ch))
            .last()
            .unwrap();
        sum += score(intersection);
    }
    sum
}
