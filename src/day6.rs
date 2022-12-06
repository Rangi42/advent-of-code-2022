use std::collections::HashSet;
use std::fs;

fn find_marker(n: usize) -> usize {
    let contents = fs::read_to_string("input/day6.txt").unwrap();
    (0..contents.len() - n)
        .position(|i| HashSet::<char>::from_iter(contents[i..i + n].chars()).len() == n)
        .unwrap()
        + n
}

pub fn part1() -> usize {
    find_marker(4)
}

pub fn part2() -> usize {
    find_marker(14)
}
