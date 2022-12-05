use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("day5/input").unwrap();
    let (stacks, instructions) = parse_input(&input);
    println!("Answer 1: {:?}", day_one(&stacks, &instructions));
    println!("Answer 2: {:?}", day_two(&stacks, &instructions));
}

type Stack = VecDeque<char>;

#[derive(Debug)]
pub struct Instruction {
    num: u32,
    from: u32,
    to: u32,
}

fn parse_input(input: &str) -> (Vec<Stack>, Vec<Instruction>) {
    // Bless this mess.
    peg::parser! {
        grammar parser() for str {
            pub rule root(s: &mut Vec<Stack>) -> Vec<Instruction>
                = stacks(s) numberline() whitespace() instrs:instructions() {
                    instrs
                }

            rule slot() -> Option<char>
                = "[" ch:['A'..='Z'] "]" { Some(ch) }
                / "   " { None }

            rule slot_row(stacks: &mut Vec<Stack>)
                = slots:slot() ** " " {
                    if slots.len() > stacks.len() {
                        stacks.resize(slots.len(), VecDeque::new());
                    }
                    for (i, v) in slots.iter().enumerate() {
                        if let Some(c) = v {
                            let deq = &mut stacks[i];
                            deq.push_front(*c);
                        }
                    }
                }

            rule stacks(s: &mut Vec<Stack>)
                = slot_row(s) ** whitespace()

            rule instruction() -> Instruction
                = "move " n:num() " from " f:num() " to " t:num() { Instruction {
                    num: n,
                    from: f,
                    to: t,
                }}

            rule instructions() -> Vec<Instruction>
                = instrs:(instruction() ** whitespace()) { instrs }

            rule numberline()
                = (['1'..='9'] ** whitespace())

            rule num() -> u32
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule whitespace()
                = [' ' | '\r' | '\n']+
        }
    }
    let mut stacks = Vec::new();
    let instructions = parser::root(input, &mut stacks).unwrap();
    (stacks, instructions)
}

fn day_one(stacks: &[Stack], instructions: &[Instruction]) -> String {
    let mut stacks = stacks.to_owned();
    for instruction in instructions {
        for _ in 0..instruction.num {
            let val = stacks[instruction.from as usize - 1].pop_back().unwrap();
            stacks[instruction.to as usize - 1].push_back(val);
        }
    }
    stacks.iter().map(|stack| *stack.back().unwrap()).collect()
}

fn day_two(stacks: &[Stack], instructions: &[Instruction]) -> String {
    let mut stacks = stacks.to_owned();
    for instruction in instructions {
        let stack = &mut stacks[instruction.from as usize - 1];
        let split = stack.split_off(stack.len() - instruction.num as usize);
        stacks[instruction.to as usize - 1].extend(split.iter());
    }
    stacks.iter().map(|stack| *stack.back().unwrap()).collect()
}

#[test]
fn test_day_two() {
    // help me
    let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
    let (stacks, instructions) = parse_input(&input);
    assert_eq!(day_two(&stacks, &instructions), "MCD");
}
