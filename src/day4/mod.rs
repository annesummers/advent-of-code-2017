use file;
use std::str;
use time::*;

pub fn run() {
    let inputs = file::read_inputs("Day4.txt");

    let lines: Vec<&str> = inputs.lines().collect();
    let test: Vec<&str> = vec![&"aa bb cc dd ee\n", &"aa bb cc dd aa\n", &"aa bb cc dd aaa\n"];

    println!("Valid {}", solve(&lines, part1));

    let mark = precise_time_ns();
    println!("Valid {} - {}", solve(&lines, part1), precise_time_ns() - mark);

    let mark = precise_time_ns();
    println!("Valid {} - {}", solve(&lines, sorted), precise_time_ns() - mark);

    println!("Valid {}", solve(&lines, part2));

    println!("Valid {}", solve(&test, part2));
}

fn solve(lines: &Vec<&str>, f: fn(&Vec<String>) -> u32) -> u32 {
    return lines
        .iter()
        .map(|line| f(&line.split_whitespace().map(|s| s.to_string()).collect()))
        .sum();
}

fn sorted(passphrase: &Vec<String>) -> u32 {
    let mut mut_passphrase = passphrase.to_owned();
    mut_passphrase.sort();

    for i in 0..mut_passphrase.len() - 1 {
        if mut_passphrase[i].eq(&mut_passphrase[i + 1]) {
            return 0;
        }
    }

    1
}

fn part1(passphrase: &Vec<String>) -> u32 {
    valid_passphrase(&passphrase)
}

fn part2(passphrase: &Vec<String>) -> u32 {
    valid_passphrase(&passphrase
        .iter()
        .map(|word| {
            let mut sorted: Vec<char> = word.chars().collect();
            sorted.sort();
            return sorted.iter().collect()
        })
        .collect())
}

fn valid_passphrase(passphrase: &Vec<String>) -> u32 {
    for (i, word) in passphrase.iter().enumerate() {
        if passphrase.split_at(i + 1).1.contains(word) {
            return 0;
        }
    }

    1
}