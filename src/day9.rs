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
            let dir = match dir {
                "U" => Dir::U,
                "D" => Dir::D,
                "L" => Dir::L,
                "R" => Dir::R,
                _ => unreachable!(),
            };
            let num = num.parse::<u32>().unwrap();
            (dir, num)
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
                let delta = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                let tail = &mut rope[i];
                match delta {
                    (-2, 0) => {
                        tail.0 -= 1;
                    }
                    (2, 0) => {
                        tail.0 += 1;
                    }
                    (0, -2) => {
                        tail.1 -= 1;
                    }
                    (0, 2) => {
                        tail.1 += 1;
                    }
                    (-2, -1) | (-1, -2) | (-2, -2) => {
                        tail.0 -= 1;
                        tail.1 -= 1;
                    }
                    (-2, 1) | (-1, 2) | (-2, 2) => {
                        tail.0 -= 1;
                        tail.1 += 1;
                    }
                    (2, -1) | (1, -2) | (2, -2) => {
                        tail.0 += 1;
                        tail.1 -= 1;
                    }
                    (2, 1) | (1, 2) | (2, 2) => {
                        tail.0 += 1;
                        tail.1 += 1;
                    }
                    (-1, -1)
                    | (-1, 0)
                    | (-1, 1)
                    | (0, -1)
                    | (0, 0)
                    | (0, 1)
                    | (1, -1)
                    | (1, 0)
                    | (1, 1) => (),
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
