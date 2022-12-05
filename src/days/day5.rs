use io;
use std::io::prelude::*;

pub fn day5(args: &[String]) {
    if args.len() != 1 {
        panic!("Expecting exactly one arg to day5, which is a valid file path.");
    }

    solve(&args[0]);
}

fn parse_into_stacks(input: &mut Vec<String>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let size = input
        .pop()
        .unwrap()
        .rsplit_once(' ')
        .unwrap()
        .1
        .parse::<usize>()
        .unwrap();
    stacks.resize(size, Vec::new());

    while !input.is_empty() {
        let line = &input.pop().unwrap();
        for (k, c) in line.chars().enumerate() {
            if k != 0 && ((k - 1) % 4 == 0) && c != ' ' {
                let x = k / 4;
                stacks[x].push(c);
            }
        }
    }

    stacks
}

struct MoveOp {
    n: usize,
    src: usize,
    dest: usize,
}

fn solve(filename: &str) {
    let mut reader = io::get_reader(filename);
    let mut line = String::new();

    // first loop: accumulate the stackstate into a string-stack until we reach the first empty line
    let mut string_stack: Vec<String> = Vec::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                if line == "\n" {
                    break;
                }
                string_stack.push(line.trim_end().to_string());
                line.clear();
            }
            Err(e) => panic!("{}", e),
        }
    }

    // dbg!(&string_stack);

    let mut stacks = parse_into_stacks(&mut string_stack);

    let mut stacks_copy = stacks.clone();

    // dbg!(&stacks);

    let mut move_ops: Vec<MoveOp> = Vec::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(6, parts.len());
        let n = parts[1].parse::<usize>().unwrap();
        let src = parts[3].parse::<usize>().unwrap() - 1;
        let dest = parts[5].parse::<usize>().unwrap() - 1;
        move_ops.push(MoveOp { n, src, dest });
        line.clear();
    }

    move_crates_onebyone(&mut stacks, &move_ops);
    println!(
        "Top of the stacks after incorrect move: '{}'",
        top_of_stack(&stacks)
    );

    move_crates_together(&mut stacks_copy, &move_ops);
    println!(
        "Top of the stacks after correct move: '{}'",
        top_of_stack(&stacks_copy)
    );
}

fn top_of_stack(stacks: &Vec<Vec<char>>) -> String {
    let mut top_of_stack = String::new();
    for stack in stacks {
        if let Some(top) = stack.last() {
            top_of_stack.push(top.clone());
        }
    }
    top_of_stack
}

fn move_crates_onebyone(stacks: &mut Vec<Vec<char>>, ops: &Vec<MoveOp>) {
    for op in ops {
        for _ in 0..op.n {
            if let Some(c) = stacks[op.src].pop() {
                stacks[op.dest].push(c);
            }
        }
    }
}

fn move_crates_together(stacks: &mut Vec<Vec<char>>, ops: &Vec<MoveOp>) {
    for op in ops {
        let n = stacks[op.src].len() - op.n;
        // stacks[op.dest].extend(stacks[op.src].drain(n..));
        let mut tmp: Vec<char> = stacks[op.src].drain(n..).collect();
        stacks[op.dest].append(&mut tmp);
    }
}
