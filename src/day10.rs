use std::fs;

fn run_cpu(mut do_cycle: impl FnMut(i32, i32)) {
    let mut x = 1;
    let mut cycle = 1;
    do_cycle(x, cycle);
    for line in fs::read_to_string("input/day10.txt").unwrap().lines() {
        if line == "noop" {
            cycle += 1;
            do_cycle(x, cycle);
        } else if line.starts_with("addx ") {
            cycle += 1;
            do_cycle(x, cycle);
            x += line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
            cycle += 1;
            do_cycle(x, cycle);
        } else {
            unreachable!();
        }
    }
}

pub fn part1() -> i32 {
    let mut answer = 0;
    run_cpu(|x, cycle| {
        if cycle % 40 == 20 {
            answer += x * cycle;
        }
    });
    answer
}

pub fn part2() -> i32 {
    run_cpu(|x, cycle| {
        let px = (cycle - 1) % 40;
        print!("{}", if (px - x).abs() <= 1 { "##" } else { ".." });
        if px == 39 {
            println!();
        }
    });
    0
}
