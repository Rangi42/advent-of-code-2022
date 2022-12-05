use regex::Regex;
use std::fs;

fn handle_crates(mover: fn(&mut Vec<Vec<char>>, usize, usize, usize) -> ()) -> String {
    let contents = fs::read_to_string("input/day5.txt").unwrap();
    let (drawing, moves) = contents.split_once("\n\n").unwrap();

    let drawing = drawing.lines().rev().skip(1).collect::<Vec<&str>>();
    let mut stacks =
        vec![Vec::<char>::new(); (drawing.iter().map(|line| line.len()).max().unwrap() + 1) / 4];
    for (index, stack) in stacks.iter_mut().enumerate() {
        for row in &drawing {
            match row.chars().nth(index * 4 + 1) {
                Some(c) if c != ' ' => stack.push(c),
                _ => (),
            }
        }
    }

    let move_regex = Regex::new(r"(?m)^move (\d+) from (\d+) to (\d+)$").unwrap();
    for cap in move_regex.captures_iter(moves) {
        let num = cap[1].parse::<usize>().unwrap();
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;
        mover(&mut stacks, num, from, to);
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .collect::<String>()
}

pub fn part1() -> String {
    handle_crates(
        |stacks: &mut Vec<Vec<char>>, num: usize, from: usize, to: usize| {
            for _ in 0..num {
                let item = stacks[from].pop().unwrap();
                stacks[to].push(item);
            }
        },
    )
}

pub fn part2() -> String {
    handle_crates(
        |stacks: &mut Vec<Vec<char>>, num: usize, from: usize, to: usize| {
            let len = stacks[from].len();
            let mut items = stacks[from].split_off(len - num);
            stacks[to].append(&mut items);
        },
    )
}
