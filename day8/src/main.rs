use itertools::Itertools;
use std::{collections::HashSet, fs, slice::Iter};

use grid::Grid;

fn main() {
    let input = fs::read_to_string("day8/input").unwrap();
    let input = parse_input(&input);
    println!("Answer 1: {}", day_one(&input)); // 1845
    println!("Answer 2: {}", day_two(&input)); // 230112
}

fn parse_input(input: &str) -> Grid<u8> {
    let width = input.lines().next().unwrap().len();
    let vec: Vec<u8> = input
        .lines()
        .flat_map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as u8))
        .collect();
    Grid::from_vec(vec, width)
}

struct TreeLine<'a, I>
where
    I: Iterator<Item = &'a u8>,
{
    tallest: u8,
    idx: usize,
    iter: I,
}

impl<'a, I> TreeLine<'a, I>
where
    I: Iterator<Item = &'a u8>,
{
    fn new(iter: I) -> Self {
        Self {
            tallest: 0,
            idx: 0,
            iter,
        }
    }
}

impl<'a, I> Iterator for TreeLine<'a, I>
where
    I: Iterator<Item = &'a u8>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for &tree in self.iter.by_ref() {
            if tree > self.tallest {
                let i = self.idx;
                self.idx += 1;
                self.tallest = tree;
                return Some(i);
            } else {
                self.idx += 1;
            }
        }
        None
    }
}

fn scenic_score(col: usize, row: usize, grid: &Grid<u8>) -> usize {
    let mut total = 1;
    let height = grid[row][col];
    let mut counter = 0;
    for i in col + 1..grid.cols() {
        counter += 1;
        if grid[row][i] >= height {
            break;
        }
    }

    total *= counter;
    counter = 0;

    for i in (0..col).rev() {
        counter += 1;
        if grid[row][i] >= height {
            break;
        }
    }

    total *= counter;
    counter = 0;

    for i in row + 1..grid.rows() {
        counter += 1;
        if grid[i][col] >= height {
            break;
        }
    }

    total *= counter;
    counter = 0;

    for i in (0..row).rev() {
        counter += 1;
        if grid[i][col] >= height {
            break;
        }
    }

    total *= counter;
    total as usize
}

fn day_one(grid: &Grid<u8>) -> usize {
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    visible.extend((0..grid.cols()).map(|n| (n, 0)));
    visible.extend((0..grid.cols()).map(|n| (n, grid.rows() - 1)));
    visible.extend((0..grid.rows()).map(|n| (0, n)));
    visible.extend((0..grid.rows()).map(|n| (grid.cols() - 1, n)));

    for row in 1..grid.rows() - 1 {
        visible.extend(TreeLine::new(grid.iter_row(row)).map(|col| (col, row)));
        visible.extend(
            TreeLine::new(grid.iter_row(row).rev()).map(|col| (grid.cols() - col - 1, row)),
        );
    }
    for col in 1..grid.cols() - 1 {
        visible.extend(TreeLine::new(grid.iter_col(col)).map(|row| (col, row)));
        visible.extend(
            TreeLine::new(grid.iter_col(col).rev()).map(|row| (col, grid.rows() - row - 1)),
        );
    }
    visible.len()
}

fn day_two(grid: &Grid<u8>) -> usize {
    (0..grid.cols())
        .cartesian_product(0..grid.rows())
        .map(|(col, row)| scenic_score(col, row, grid))
        .max()
        .unwrap()
}

#[test]
fn test_day_one() {
    let input = r#"30373
25512
65332
33549
35390"#;
    let input = parse_input(&input);
    assert_eq!(day_one(&input), 21);
}

#[test]
fn test_day_two() {
    let input = r#"30373
25512
65332
33549
35390"#;
    let input = parse_input(&input);
    assert_eq!(day_two(&input), 8);
}

#[test]
fn test_visible_iter() {
    let nums = [1, 2, 3, 2, 1, 4];
    let mut iter = TreeLine::new(nums.iter());
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);
    let mut iter = TreeLine::new(nums.iter().rev());
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);
}
