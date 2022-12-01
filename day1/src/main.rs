use std::fs;

fn main() {
    let elves = parse_input();
    println!("Answer 1: {}", part_one(&elves));
    println!("Answer 2: {}", part_two(&elves));
}

struct Elf(Vec<i32>);

impl Elf {
    pub fn calories(&self) -> i32 {
        self.0.iter().sum()
    }
}

fn parse_input() -> Vec<Elf> {
    let input = fs::read_to_string("day1/input").unwrap();
    input.split("\r\n\r\n").map(|chunk| {
        let food: Vec<i32> = chunk.trim().lines().map(|num| num.parse::<i32>().unwrap()).collect();
        Elf(food)
    }).collect()
}

fn part_one(elves: &[Elf]) -> i32 {
    elves.iter().map(Elf::calories).max().unwrap()
}

fn part_two(elves: &[Elf]) -> i32 {
    let mut food: Vec<i32> = elves.iter().map(Elf::calories).collect();
    food.sort();
    food.iter().rev().take(3).sum()
}