mod file;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let today = 4;

    match today {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        _ => println!("Not found")
    }
}
