use io;
use std::path::Path;

pub fn day1(args: &[String]) {
    if args.len() != 1 {
        panic!("Expecting exactly one arg to day1, which is a valid file path.");
    }

    let mut calories_of_elf = get_calories(&args[0]);

    calories_of_elf.sort_by(|a, b| b.cmp(a));

    println!("Hungriest Elf carries {} calories.", calories_of_elf[0]);

    let sum_top_3: i32 = calories_of_elf[..3].iter().sum();

    println!("The top 3 Elfs carry {sum_top_3} kcal in total.");
}

fn get_calories<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let mut calories_of_elf: Vec<i32> = Vec::new();
    let mut cur_calories = 0;

    if let Ok(lines) = io::read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    calories_of_elf.push(cur_calories);
                    cur_calories = 0;
                } else {
                    let calories: i32 = line.parse().expect("Could not parse number");
                    cur_calories += calories;
                }
            }
        }
    }
    return calories_of_elf;
}
