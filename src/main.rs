mod file;
mod day1;
mod day2;
mod day3;

fn main() {
    let today = 3;

    match today {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        _ => println!("Not found")
    }
}
