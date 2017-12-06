use file;
use std::collections::hash_map::HashMap;
use time;

pub fn run() {
    let inputs = file::read_inputs("Day6.txt");

    println!("Answer {}", solve(&"0\t2\t7\t0".to_string(), true));
    println!("Answer {}", solve(&"0\t2\t7\t0".to_string(), false));

    let mark = time::precise_time_ns();
    println!("Answer {} - {}ms", solve(&inputs, true), (time::precise_time_ns() - mark)/1000000);

    let mark = time::precise_time_ns();
    println!("Answer {} - {}ms", solve(&inputs, false), (time::precise_time_ns() - mark)/1000000);
}

fn solve(inputs: &String, part1: bool) -> u32 {
    let mut hash_map: HashMap<Vec<i32>, u32> = HashMap::new();
    let mut banks: Vec<i32> = inputs
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let mut count: u32 = 0;
    let mut hash;

    while !hash_map.contains_key(&banks) {
        hash = Vec::new();
        hash.clone_from(&banks);

        hash_map.insert(hash, count);

        let mut index = 0;
        let mut largest: i32 = 0;
        for (i, val) in banks.iter().enumerate() {
            if *val > largest {
                largest = *val;
                index = i;
            }
        }

        let mut blocks = banks[index];
        banks[index] = 0;

        while blocks > 0 {
            index = (index + 1) % banks.len();
            banks[index] += 1;
            blocks -= 1;
        }

        count += 1;
    }

    if part1 {
        return count;
    }

    return count - hash_map.get(&banks).unwrap();

}