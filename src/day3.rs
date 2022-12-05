use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}

pub fn part1() {
    let answer = fs::read_to_string("input/day3.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left: HashSet<char> = left.chars().collect();
            priority(right.chars().filter(|c| left.contains(c)).next().unwrap())
        })
        .sum::<u32>();
    println!("{:?}", answer);
}

pub fn part2() {
    let answer = fs::read_to_string("input/day3.txt")
        .unwrap()
        .lines()
        .tuples() // from itertools
        .map(|(first, second, third)| {
            let first: HashSet<char> = first.chars().collect();
            let second: HashSet<char> = second.chars().collect();
            priority(
                third
                    .chars()
                    .filter(|c| first.contains(c) && second.contains(c))
                    .next()
                    .unwrap(),
            )
        })
        .sum::<u32>();
    println!("{:?}", answer);
}
