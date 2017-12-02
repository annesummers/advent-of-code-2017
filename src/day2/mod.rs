use file;

pub fn run() {
    let inputs = file::read_inputs("Day2.txt");
    let lines = read_values(&inputs);

    part1(&lines);
    part2(&lines);
    part2_recursive(&lines);

    part1(&read_values(&"5 1 9 5\n7 5 3\n2 4 6 8"));
    part2(&read_values(&"5 9 2 8\n9 4 7 3\n3 8 6 5"));
}

fn read_values(inputs: &str) -> Vec<Vec<u32>> {
    let mut values = Vec::new();

    for line in inputs.split('\n') {
        if line.is_empty() {
            break;
        }

        values.push(Vec::new());

        let length = values.len();

        let mut line: Vec<u32> = line
            .split(|c| c == ' ' || c == '\t')
            .map(|v| v.parse::<u32>().unwrap())
            .collect();

        line.sort_by(|a, b| b.cmp(a));

        values[length - 1] = line.to_owned();
    }

    values
}

fn part1(lines: &Vec<Vec<u32>>) {
    let mut sum: u32 = 0;

    for line in lines {
        sum += line[0] - line[lines.len() - 1];
    }

    println!("Checksum {}", sum);
}

fn part2(lines: &Vec<Vec<u32>>) {
    let mut sum: u32 = 0;

    for line in lines {
        let mut division = 0;

        for i in 0..line.len() {
            for j in i + 1..line.len() {
                if line[i] % line[j] == 0 {
                    division = line[i] / line[j];
                    break;
                }
            }
        }

        sum += division;
    }

    println!("New checksum {}", sum);
}

fn part2_recursive(lines: &Vec<Vec<u32>>) {
    let sum: u32 = lines
        .iter()
        .map(|line| find_divisor(line))
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    println!("New checksum recursive {}", sum);
}

fn find_divisor(line: &Vec<u32>) -> u32 {
    match line.split_first() {
        None => return 0,
        Some((comp, line)) => {
            for value in line {
                if *comp % *value == 0 {
                    return *comp / *value;
                }
            }

            return find_divisor(&line.to_vec());
        },
    };
}