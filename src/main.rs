use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut calories_of_elf = get_calories(file_path);

    calories_of_elf.sort_by(|a, b| b.cmp(a));

    println!("Hungriest Elf carries {} calories.", calories_of_elf[0]);

    let sum_top_3: i32 = calories_of_elf[..3].iter().sum();

    println!("The top 3 Elfs carry {sum_top_3} kcal in total.");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_calories<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let mut calories_of_elf: Vec<i32> = Vec::new();
    let mut cur_calories = 0;

    if let Ok(lines) = read_lines(filename) {
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
