use std::fs;

#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Strategy {
    X,
    Y,
    Z,
}

struct Round {
    them: Hand,
    me: Hand,
}

fn read_rounds() -> Vec<(Hand, Strategy)> {
    fs::read_to_string("input/day2.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (them, me) = line.split_once(' ').unwrap();
            let them = match them {
                "A" => Hand::Rock,
                "B" => Hand::Paper,
                "C" => Hand::Scissors,
                _ => unreachable!(),
            };
            let me = match me {
                "X" => Strategy::X,
                "Y" => Strategy::Y,
                "Z" => Strategy::Z,
                _ => unreachable!(),
            };
            (them, me)
        })
        .collect()
}

fn get_score(round: Round) -> u32 {
    let shape = match round.me {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    };
    let outcome = match round {
        // lose
        Round {
            them: Hand::Paper,
            me: Hand::Rock,
        }
        | Round {
            them: Hand::Scissors,
            me: Hand::Paper,
        }
        | Round {
            them: Hand::Rock,
            me: Hand::Scissors,
        } => 0,
        // draw
        Round {
            them: Hand::Rock,
            me: Hand::Rock,
        }
        | Round {
            them: Hand::Paper,
            me: Hand::Paper,
        }
        | Round {
            them: Hand::Scissors,
            me: Hand::Scissors,
        } => 3,
        // win
        Round {
            them: Hand::Rock,
            me: Hand::Paper,
        }
        | Round {
            them: Hand::Paper,
            me: Hand::Scissors,
        }
        | Round {
            them: Hand::Scissors,
            me: Hand::Rock,
        } => 6,
    };
    shape + outcome
}

fn do_part(strategy_to_hand: fn(&(Hand, Strategy)) -> Hand) -> u32 {
    let rounds = read_rounds();
    rounds
        .iter()
        .map(|round| Round {
            them: round.0,
            me: strategy_to_hand(round),
        })
        .map(get_score)
        .sum()
}

pub fn part1() -> u32 {
    do_part(|round| match round.1 {
        Strategy::X => Hand::Rock,
        Strategy::Y => Hand::Paper,
        Strategy::Z => Hand::Scissors,
    })
}

pub fn part2() -> u32 {
    do_part(|round| match round {
        // lose
        (Hand::Rock, Strategy::X) => Hand::Scissors,
        (Hand::Paper, Strategy::X) => Hand::Rock,
        (Hand::Scissors, Strategy::X) => Hand::Paper,
        // draw
        (Hand::Rock, Strategy::Y) => Hand::Rock,
        (Hand::Paper, Strategy::Y) => Hand::Paper,
        (Hand::Scissors, Strategy::Y) => Hand::Scissors,
        // win
        (Hand::Rock, Strategy::Z) => Hand::Paper,
        (Hand::Paper, Strategy::Z) => Hand::Scissors,
        (Hand::Scissors, Strategy::Z) => Hand::Rock,
    })
}
