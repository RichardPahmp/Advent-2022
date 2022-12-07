use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("day6/input").unwrap();
    println!("Answer 1: {}", day_one(&input)); // 1175
    println!("Answer 2: {}", day_two(&input)); // 3217
}

fn find_marker(input: &str, marker_size: usize) -> usize {
    let mut set: HashSet<u8> = HashSet::with_capacity(marker_size);
    input
        .as_bytes()
        .windows(marker_size)
        .position(|window| {
            set.clear();
            set.extend(window.iter());
            set.len() == marker_size
        })
        .unwrap()
        + marker_size
}

fn day_one(input: &str) -> usize {
    find_marker(input, 4)
}

fn day_two(input: &str) -> usize {
    find_marker(input, 14)
}

#[test]
fn test_day_one() {
    assert_eq!(day_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(day_one("bvwbjplbgvbhsrlpgdmjqwftvnc"), 5);
    assert_eq!(day_one("nppdvjthqldpwncqszvftbrmjlh"), 6);
    assert_eq!(day_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(day_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

#[test]
fn test_day_two() {
    assert_eq!(day_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(day_two("bvwbjplbgvbhsrlpgdmjqwftvnc"), 23);
    assert_eq!(day_two("nppdvjthqldpwncqszvftbrmjlh"), 23);
    assert_eq!(day_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(day_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}
