use std::collections::HashSet;
use std::fs;

enum Dir {
    U,
    D,
    L,
    R,
}

fn simulate(knots: usize) -> usize {
    let mut rope = vec![(0, 0); knots];
    let mut seen = HashSet::<(i32, i32)>::new();
    seen.insert(*rope.last().unwrap());
    for (dir, num) in fs::read_to_string("input/day9.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (dir, num) = line.split_once(' ').unwrap();
            (
                match dir {
                    "U" => Dir::U,
                    "D" => Dir::D,
                    "L" => Dir::L,
                    "R" => Dir::R,
                    _ => unreachable!(),
                },
                num.parse::<u32>().unwrap(),
            )
        })
    {
        for _ in 0..num {
            match dir {
                Dir::L => rope[0].0 -= 1,
                Dir::R => rope[0].0 += 1,
                Dir::U => rope[0].1 -= 1,
                Dir::D => rope[0].1 += 1,
            }
            for i in 1..knots {
                let (head, tail) = (rope[i - 1], &mut rope[i]);
                match (head.0 - tail.0, head.1 - tail.1) {
                    (-1 | 0 | 1, -1 | 0 | 1) => (),
                    (-2, 0) => tail.0 -= 1,
                    (2, 0) => tail.0 += 1,
                    (0, -2) => tail.1 -= 1,
                    (0, 2) => tail.1 += 1,
                    (-1 | -2, -1 | -2) => {
                        tail.0 -= 1;
                        tail.1 -= 1;
                    }
                    (-1 | -2, 1 | 2) => {
                        tail.0 -= 1;
                        tail.1 += 1;
                    }
                    (1 | 2, -1 | -2) => {
                        tail.0 += 1;
                        tail.1 -= 1;
                    }
                    (1 | 2, 1 | 2) => {
                        tail.0 += 1;
                        tail.1 += 1;
                    }
                    _ => unreachable!(),
                }
            }
            seen.insert(*rope.last().unwrap());
        }
    }
    seen.len()
}

pub fn part1() -> usize {
    simulate(2)
}

pub fn part2() -> usize {
    simulate(10)
}
