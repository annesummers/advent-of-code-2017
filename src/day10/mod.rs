use file;
use std::u8;

pub fn run() {
    let inputs = file::read_inputs("Day10.txt");

    println!("{}", part1(&"3,4,1,5", 5));
    println!("{}", part1(&inputs, 256));
    println!("Hash {}", find_hash(&mut[65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22]));
    println!("{}", part2(&"", 256));
    println!("{}", part2(&"AoC 2017", 256));
    println!("{}", part2(&"1,2,3", 256));
    println!("{}", part2(&"1,2,4", 256));
    println!("{}", part2(&inputs, 256));
}

fn part1(inputs: &str, list_length: usize) -> usize {
    let slice_lengths: Vec<u8> = inputs
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    let mut list: Vec<u8> = Vec::with_capacity(list_length);

    for i in 0..list_length {
        list.insert(i, i as u8);
    }

    process(&slice_lengths, &mut list, 0, 0);

    list[0] as usize * list[1] as usize
}

fn part2(inputs: &str, list_length: usize) -> String {
    let mut slice_lengths: Vec<u8> = inputs
        .chars()
        .map(|c| c as u8)
        .collect();

    slice_lengths.append(&mut vec![17, 31, 73, 47, 23]);

    let mut list: Vec<u8> = Vec::with_capacity(list_length);

    for i in 0..list_length {
        list.insert(i, i as u8);
    }

    let mut pointer = 0;
    let mut skip = 0;

    for _ in 0..64 {
        let result = process(&slice_lengths, &mut list, pointer, skip);

        pointer = result.0;
        skip = result.1;
    }

    const HASH_LEN: usize = 16;
    let mut string_hash = String::new();
    let mut counter = 0;

    while counter + HASH_LEN <= list.len() {
        string_hash.push_str(&format!("{:02x}", find_hash(&mut list[counter..counter + HASH_LEN])));
        counter += HASH_LEN;
    }

    string_hash
}

fn find_hash(slice: &mut [u8]) -> u8 {
    let mut hash: u8 = slice[0] as u8;
    for i in 1..slice.len()  {
        hash = hash ^ slice[i] as u8;
    }

    hash
}

fn process(slice_lengths: &Vec<u8>, list: &mut Vec<u8>, pointer: usize, skip: usize) -> (usize, usize) {
    let mut mut_pointer: usize = pointer;
    let mut mut_skip: usize = skip;

    for slice_length in slice_lengths {
        let abs_split: usize = (mut_pointer + *slice_length as usize) % (list.len());

        if abs_split < *slice_length as usize {
            let mut slice = Vec::with_capacity(*slice_length as usize);
            for i in 0..*slice_length {
                let index = (i as usize + mut_pointer) % list.len();
                slice.insert(i as usize, list[index]);
            }

            slice.reverse();

            let mut counter = 0;

            for i in mut_pointer..list.len() {
                list[i] = slice[counter];
                counter = counter + 1;
            }

            for i in 0..slice.len() - counter {
                list[i] = slice[counter];
                counter = counter + 1;
            }
        } else {
            let slice = &mut list[mut_pointer..(mut_pointer + *slice_length as usize )];
            slice.reverse();
        }

        mut_pointer = (mut_pointer + mut_skip + *slice_length as usize) % list.len();
        mut_skip = mut_skip + 1;
    }

    (mut_pointer, mut_skip)
}