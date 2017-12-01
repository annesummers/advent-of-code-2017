use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn read_inputs<P>(path : P) -> String where P: AsRef<Path> {
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