use file;

pub fn run() {
    let inputs = file::read_inputs("Day1.txt");

    calculate_matches(&inputs);
    calculate_matches(&"1122");
    calculate_matches(&"1111");
    calculate_matches(&"1234");
    calculate_matches(&"91212129");
}

fn calculate_matches(str_inputs: &str) {
    let inputs: Vec<u32> = str_inputs.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let length = inputs.len();

    let mut half_way: usize = (length/2) as usize;
    let mut next_match_sum = 0;
    let mut opposite_match_sum = 0;

    for (i, digit) in inputs.iter().enumerate() {
        if i == 0 && *digit == inputs[length - 1] ||
            i < length - 1 && *digit == inputs[i + 1] {
            next_match_sum += digit;
        }

        if *digit == inputs[half_way] {
            opposite_match_sum += digit;
        }

        half_way += 1;
        if half_way >= length {
            half_way = 0;
        }
    }

    println!("Match count 1 is {}", next_match_sum);
    println!("Match count 2 is {}", opposite_match_sum);
}