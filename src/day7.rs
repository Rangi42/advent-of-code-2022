use std::collections::HashMap;
use std::fs;

fn get_dir_sizes() -> HashMap<String, u32> {
    let mut sizes = HashMap::<String, u32>::new();
    let mut pwds = Vec::<String>::from(["/".to_string()]);
    for line in fs::read_to_string("input/day7.txt").unwrap().lines() {
        if line == "$ ls" || line.starts_with("dir ") {
            // do nothing
        } else if line == "$ cd .." {
            pwds.pop();
        } else if line.starts_with("$ cd ") {
            let name = line.strip_prefix("$ cd ").unwrap();
            pwds.push(pwds.last().unwrap().to_string() + name + "/");
        } else {
            let size = line.split_once(' ').unwrap().0.parse::<u32>().unwrap();
            for dir in &pwds {
                sizes.insert(dir.to_string(), sizes.get(&dir[..]).unwrap_or(&0) + size);
            }
        }
    }
    sizes
}

pub fn part1() -> u32 {
    get_dir_sizes()
        .values()
        .filter(|size| **size <= 100000)
        .sum::<u32>()
}

pub fn part2() -> u32 {
    let sizes = get_dir_sizes();
    let needed_space = sizes.get("/").unwrap() + 30000000 - 70000000;
    *sizes
        .values()
        .filter(|size| **size >= needed_space)
        .min()
        .unwrap()
}
