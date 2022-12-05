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
    count_assignments(|(first_start, first_end), (second_start, second_end)| {
        (first_start <= second_start && first_end >= second_end)
            || (second_start <= first_start && second_end >= first_end)
    })
}

pub fn part2() -> usize {
    count_assignments(|(first_start, first_end), (second_start, second_end)| {
        first_end >= second_start && second_end >= first_start
    })
}
