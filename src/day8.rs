use itertools::Itertools; // for find_position
use std::fs;

fn get_trees() -> (Vec<Vec<u8>>, usize, usize) {
    let trees = fs::read_to_string("input/day8.txt")
        .unwrap()
        .lines()
        .map(|line| line.bytes().map(|tree| tree - b'0').collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let width = trees[0].len();
    let height = trees.len();
    (trees, width, height)
}

pub fn part1() -> usize {
    let (trees, width, height) = get_trees();
    (0..height)
        .map(|y| {
            (0..width)
                .filter(|&x| {
                    let tree = trees[y][x];
                    (0..y).all(|y2| trees[y2][x] < tree)
                        || (y + 1..height).all(|y2| trees[y2][x] < tree)
                        || (0..x).all(|x2| trees[y][x2] < tree)
                        || (x + 1..width).all(|x2| trees[y][x2] < tree)
                })
                .count()
        })
        .sum()
}

pub fn part2() -> usize {
    let (trees, width, height) = get_trees();
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    let tree = trees[y][x];
                    let above = (0..y)
                        .rev()
                        .find_position(|y2| trees[*y2][x] >= tree)
                        .map(|(i, _)| i + 1)
                        .unwrap_or(y as usize);
                    let below = (y + 1..height)
                        .find_position(|y2| trees[*y2][x] >= tree)
                        .map(|(i, _)| i + 1)
                        .unwrap_or((height - y - 1) as usize);
                    let left = (0..x)
                        .rev()
                        .find_position(|x2| trees[y][*x2] >= tree)
                        .map(|(i, _)| i + 1)
                        .unwrap_or(x as usize);
                    let right = (x + 1..width)
                        .find_position(|x2| trees[y][*x2] >= tree)
                        .map(|(i, _)| i + 1)
                        .unwrap_or((width - x - 1) as usize);
                    above * below * left * right
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap() as usize
}
