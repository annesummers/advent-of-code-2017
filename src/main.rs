mod file;
mod day1;
mod day2;

fn main() {
    let today = 2;

    match today {
        1 => day1::run(),
        2 => day2::run(),
        _ => println!("Not found")
    }
}
