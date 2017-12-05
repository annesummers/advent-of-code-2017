use file;

pub fn run() {
    let inputs = file::read_inputs("Day5.txt");

    println!("Steps {}", solve(&"0\n3\n0\n1\n-3".to_string(), part1));
    println!("Steps {}", solve(&"0\n3\n0\n1\n-3".to_string(), part2));
    println!("Steps {}", solve(&inputs, part1));
    println!("Steps {}", solve(&inputs, part2));
}

fn solve(inputs: &String, modify: fn(&mut i32)) -> u32 {
    let mut lines: Vec<i32> = inputs
        .lines()
        .filter(|s| !s.is_empty())
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    let mut pos: i32 = 0;
    let mut count = 0;

    while pos < lines.len() as i32 && pos >= 0 {
        let jump = lines[pos as usize];

        modify(&mut(lines[pos as usize]));

        pos += jump;
        count += 1;
    }

    return count;
}

fn part1(item: &mut i32) {
    *item += 1;
}

fn part2(item: &mut i32) {
    if *item >= 3 {
        *item -= 1;
    } else {
        *item += 1;
    }
}