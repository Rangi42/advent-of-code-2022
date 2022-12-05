use std::fs;

fn get_assignment(assignment: &str) -> (u32, u32) {
    let (start, end) = assignment.split_once('-').unwrap();
    (start.parse::<u32>().unwrap(), end.parse::<u32>().unwrap())
}

fn count_assignments(predicate: fn((u32, u32), (u32, u32)) -> bool) -> usize {
    fs::read_to_string("input/day4.txt")
        .unwrap()
        .lines()
        .filter(|line| {
            let (first, second) = line.split_once(',').unwrap();
            predicate(get_assignment(first), get_assignment(second))
        })
        .count()
}

pub fn part1() -> usize {
    count_assignments(|first, second| {
        (first.0 <= second.0 && first.1 >= second.1) || (second.0 <= first.0 && second.1 >= first.1)
    })
}

pub fn part2() -> usize {
    count_assignments(|first, second| first.1 >= second.0 && second.1 >= first.0)
}
