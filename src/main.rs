use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn main() {
    day1();
}

fn day1() {
    let inputs = read_inputs("Day1.txt");

    calculate_matches_with_next(&inputs);
    calculate_matches_with_next(&"1122".to_string());
    calculate_matches_with_next(&"1111".to_string());
    calculate_matches_with_next(&"1234".to_string());
    calculate_matches_with_next(&"91212129".to_string());
}

fn calculate_matches_with_next(inputs: &String) {
    let mut first_digit = -1;
    let mut prev_digit = -1;
    let mut match_count = 0;

    for digit_char in inputs.chars() {
        let digit = char_to_int(digit_char);

        println!("Digit is {}", digit);

        if first_digit == -1 {
            first_digit = digit;
            continue;
        }

        if digit == prev_digit {
            match_count += prev_digit;
        }

        prev_digit = digit;
    }

    if first_digit == prev_digit {
        match_count += first_digit;
    }

    println!("Match count is {}", match_count);
}

fn char_to_int(int_as_char: char) -> i32 {
    let digit = int_as_char.to_string().parse::<i32>().unwrap();
    digit
}

fn read_inputs<P>(path : P) -> String where P: AsRef<Path> {
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open file: {}",
                           why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read file: {}",
                           why.description()),
        Ok(s) => s,
    };

    s
}
