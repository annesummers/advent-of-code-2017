use file;
use std::collections::hash_map::HashMap;

pub fn run() {
    let inputs = file::read_inputs("Day8.txt");
    let test = "b inc 5 if a > 1
    a inc 1 if b < 5
    c dec -10 if a >= 1
    c inc -20 if c == 10";

    println!("{:?}", solve(test));
    println!("{:?}", solve(&inputs));
}

#[derive(Debug)]
enum Condition {
    Greater,
    GreaterOrEquals,
    Equals,
    SmallerOrEquals,
    Smaller,
    NotEquals,
}

#[derive(Debug)]
struct Predicate<'a> {
    register : &'a str,
    value : i32,
    condition : Condition,
}

#[derive(Debug)]
struct Instruction<'a> {
    predicate : Predicate<'a>,
    register : &'a str,
    rule : i32,
}

fn solve(inputs: &str) -> (i32, i32) {
    let lines: Vec<Vec<&str>> = inputs
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split_whitespace().collect())
            .collect();

    let mut registers = HashMap::new();
    let mut instructions = Vec::new();

    for line in &lines {
        registers.insert(line[0], 0);

        let mut rule = 0;
        rule = line[2].parse::<i32>().unwrap();
        if line[1] == "dec" {
            rule = -rule;
        }

        instructions.push(Instruction {
            predicate: Predicate {
                register: line[4],
                value: line[6].parse::<i32>().unwrap(),
                condition: match line[5] {
                    ">" => Condition::Greater,
                    ">=" => Condition::GreaterOrEquals,
                    "==" => Condition::Equals,
                    "<=" => Condition::SmallerOrEquals,
                    "<" => Condition::Smaller,
                    "!=" => Condition::NotEquals,
                    _ => Condition::Equals,
                },
            },
            register: line[0],
            rule,
        });
    }

    let mut highest = 0;

    for instruction in instructions {
        let pred_reg_value = registers[instruction.predicate.register];
        let fulfills = match instruction.predicate.condition {
            Condition::Greater => pred_reg_value > instruction.predicate.value,
            Condition::GreaterOrEquals => pred_reg_value >= instruction.predicate.value,
            Condition::Equals => pred_reg_value == instruction.predicate.value,
            Condition::SmallerOrEquals => pred_reg_value <= instruction.predicate.value,
            Condition::Smaller => pred_reg_value < instruction.predicate.value,
            Condition::NotEquals => pred_reg_value != instruction.predicate.value,
        };

        if fulfills {
            let new_val = registers[instruction.register] + instruction.rule;
            registers.insert(instruction.register, new_val);

            if new_val > highest {
                highest = new_val;
            }
        }
    }

    (*registers.values().max().unwrap(), highest)
}