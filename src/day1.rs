use std::fs;

fn read_calories() -> Vec<i32> {
    fs::read_to_string("input/day1.txt")
        .unwrap()
        .split("\n\n")
        .map(|lines| {
            lines
                .split_terminator('\n')
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect()
}

pub fn part1() {
    let calories = read_calories();
    let answer = calories.iter().max().unwrap();
    println!("{}", answer);
}

pub fn part2() {
    let mut calories = read_calories();
    calories.sort_by(|a, b| b.cmp(a));
    let answer = calories.iter().take(3).sum::<i32>();
    println!("{}", answer);
}
