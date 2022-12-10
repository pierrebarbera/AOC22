use std::env;
mod algorithm;
mod days;
mod io;
mod iter;
mod tree;

extern crate itertools;

fn main() {
    let args: Vec<String> = env::args().collect();

    if !(args.len() > 2) {
        eprintln!("Call must at least specify a day.");
        return;
    }

    let day: &str = &args[1];
    let args = &args[2..];

    match day {
        "day1" => days::day1::day1(args),
        "day2" => days::day2::day2(args),
        "day3" => days::day3::day3(args),
        "day4" => days::day4::day4(args),
        "day5" => days::day5::day5(args),
        "day6" => days::day6::day6(args),
        "day7" => days::day7::day7(args),
        "day8" => days::day8::day8(args),
        "day9" => days::day9::day9(args),
        _ => eprintln!("Not a valid day: {day}"),
    }
}
