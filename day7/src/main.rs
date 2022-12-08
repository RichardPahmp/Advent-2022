use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("day7/input").unwrap();
    let parsed = parse_input(&input);
    println!("Answer 1: {}", day_one(&parsed));
    // println!("Answer 2: {}", day_two(&parsed));
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
                = name:$(['.'..='z']+) { name }

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
                = cd() / popdir() / ls()

            pub rule root() -> Vec<Command<'input>>
                = commands:(command() ** "\r\n") { commands }
        }
    }
    parser::root(input).unwrap()
}

fn day_one<'a>(commands: &[Command<'a>]) -> u32 {
    let mut visited: HashMap<&'a str, u32> = HashMap::new();
    let mut stack = Vec::new();
    for command in commands {
        match command {
            Command::ChangeDir(dir) => {
                stack.push(dir);
                let entry = visited.entry(dir).or_insert(0);
                println!("{entry}");
            }
            Command::PopDir => {
                stack.pop();
            }
            Command::Ls(entries) => {
                for entry in entries {
                    match entry {
                        FileDir::Dir(_) => {}
                        FileDir::File(name, size) => {
                            for dir in &stack {
                                *visited.get_mut(*dir).unwrap() += size;
                            }
                        }
                    }
                }
            }
        }
    }
    todo!()
}

#[test]
fn test_day_one() {
    input = r#"$ cd /
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
7214296 k"#;
    let input = parse_input(&input);
    assert_eq!(day_one(&input), 95437)
}
