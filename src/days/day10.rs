use io;

pub fn day10(args: &[String]) {
    if args.len() != 1 {
        panic!("Expecting exactly one arg to day10, which is a valid file path.");
    }

    let mut sum_signal_strengths: i32 = 0;

    let mut check_cycles: Vec<usize> = vec![usize::MAX, 219, 179, 139, 99, 59, 19];
    let mut check_cycle = check_cycles.pop().unwrap();

    foreach_cycle(&args[0], |cycle, x| {
        if cycle == check_cycle {
            // we are at a check cycle!
            let signal_strength = x * (check_cycle + 1) as i32;
            sum_signal_strengths += signal_strength;
            // get the next check cycle
            check_cycle = check_cycles.pop().unwrap();
        }
    });

    println!("Sum of signal strengths: {}", sum_signal_strengths);

    // render the image!
    let width: usize = 40;
    let height: usize = 6;
    let mut image = vec![vec!['🎄'; width]; height];
    foreach_cycle(&args[0], |cycle, x| {
        let col = cycle % width;
        if (x - 1..=x + 1).contains(&(col as i32)) {
            let row = cycle / width;
            image[row][col] = '🎁';
        }
    });

    for row in image {
        println!("{}", row.into_iter().collect::<String>());
    }
}

enum Instruction {
    Noop,
    Addx(i32),
}

fn foreach_cycle<F>(filename: &str, mut f: F)
where
    F: FnMut(usize, i32),
{
    let mut cycle: usize = 0;
    let mut prev_cycle: usize = cycle;
    let mut x: i32 = 1;
    let mut prev_x: i32 = x;
    io::foreach_nonempty_line(filename, |line| {
        prev_cycle = cycle;
        prev_x = x;

        match parse_instruction(line) {
            Instruction::Noop => cycle += 1,
            Instruction::Addx(val) => {
                x += val;
                cycle += 2;
            }
        }

        for c in prev_cycle..cycle {
            f(c, prev_x);
        }
    });
}

fn parse_instruction(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split_whitespace().collect();
    assert!([1, 2].contains(&parts.len()));
    match parts[0] {
        "noop" => Instruction::Noop,
        "addx" => Instruction::Addx(parts[1].parse::<i32>().unwrap()),
        _ => panic!("Unrecognized command: {}", line),
    }
}
