use std::fs;

fn read_calories() -> Vec<u32> {
    fs::read_to_string("input/day1.txt")
        .unwrap()
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect()
}

pub fn part1() -> u32 {
    let calories = read_calories();
    *calories.iter().max().unwrap()
}

pub fn part2() -> u32 {
    let mut calories = read_calories();
    calories.sort();
    calories.iter().rev().take(3).sum()
}
