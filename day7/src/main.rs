use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("day7/input").unwrap();
    let parsed = parse_input(&input);
    println!("Answer 1: {}", day_one(&parsed));
    println!("Answer 2: {}", day_two(&parsed));
}

#[derive(Debug)]
pub enum FileDir<'a> {
    Dir(&'a str),
    File(&'a str, u32),
}

#[derive(Debug)]
pub enum Command<'a> {
    ChangeDir(&'a str),
    PopDir,
    Ls(Vec<FileDir<'a>>),
}

fn parse_input(input: &str) -> Vec<Command<'_>> {
    peg::parser! {
        grammar parser() for str {
            rule name() -> &'input str
                = name:$(['/' | '.'..='z']+) { name }

            rule cd() -> Command<'input>
                = "$ cd " n:name() { Command::ChangeDir(n) }

            rule popdir() -> Command<'input>
                = "$ cd .." { Command::PopDir }

            rule num() -> u32
                = n:$(['0'..='9']+) { n.parse::<u32>().unwrap() }

            rule file() -> FileDir<'input>
                = num:num() " " name:name() { FileDir::File(name, num) }

            rule dir() -> FileDir<'input>
                = "dir " name:name() { FileDir::Dir(name) }

            rule dir_contents() -> Vec<FileDir<'input>>
                = (file() / dir()) ** "\r\n"

            rule ls() -> Command<'input>
                = "$ ls\r\n" contents:dir_contents() { Command::Ls(contents) }

            rule command() -> Command<'input>
                = popdir() / cd() / ls()

            pub rule root() -> Vec<Command<'input>>
                = commands:(command() ** "\r\n") { commands }
        }
    }
    parser::root(input).unwrap()
}

fn execute_commands<'a>(commands: &[Command<'a>]) -> HashMap<Vec<&'a str>, u32> {
    let mut visited: HashMap<Vec<&'a str>, u32> = HashMap::new();
    let mut cwd = Vec::new();
    for command in commands {
        match command {
            Command::ChangeDir(dir) => {
                cwd.push(*dir);
                visited.entry(cwd.clone()).or_insert(0);
            }
            Command::PopDir => {
                cwd.pop().unwrap();
            }
            Command::Ls(entries) => {
                for entry in entries {
                    match entry {
                        FileDir::Dir(_) => {}
                        FileDir::File(_, size) => {
                            for i in (1..cwd.len() + 1).rev() {
                                *visited.get_mut(&cwd[0..i]).unwrap() += size;
                            }
                        }
                    }
                }
            }
        }
    }
    visited
}

fn day_one<'a>(commands: &[Command<'a>]) -> u32 {
    execute_commands(commands)
        .iter()
        .map(|(_, v)| v)
        .filter(|&&v| v <= 100000)
        .sum::<u32>()
}

const DISK_SIZE: u32 = 70000000;
const TARGET: u32 = 30000000;

fn day_two<'a>(commands: &[Command<'a>]) -> u32 {
    let dirs = execute_commands(commands);
    let free = DISK_SIZE - dirs.get(&vec!["/"]).unwrap();
    *dirs
        .iter()
        .map(|(_, v)| v)
        .filter(|&&v| v >= TARGET - free)
        .min()
        .unwrap()
}

#[test]
fn test_day_one() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
    let input = parse_input(&input);
    assert_eq!(day_one(&input), 95437)
}
