use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn foreach_line<P, F>(filename: P, mut f: F)
where
    P: AsRef<Path>,
    F: FnMut(&str),
{
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                f(line.as_str());
            }
        }
    }
}

pub fn foreach_linegroup<P, F>(filename: P, group_size: usize, mut f: F)
where
    P: AsRef<Path>,
    F: FnMut(&Vec<String>),
{
    let mut group: Vec<String> = Vec::new();
    foreach_line(filename, |line| {
        group.push(line.to_string());
        if group.len() == group_size {
            f(&group);
            group.clear();
        }
    });
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_reader<P>(filename: P) -> io::BufReader<File>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
}
