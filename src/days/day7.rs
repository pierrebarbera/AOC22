use io;
use std::fmt;
use tree;

type FsNodeRef = tree::NodeRef<FsNode>;

pub fn day7(args: &[String]) {
    if args.len() != 1 {
        panic!("Expecting exactly one arg to day7, which is a valid file path.");
    }

    let root: FsNodeRef = tree::new_node_ref("root", FsNode::Dir(0), None);
    tree::add_node_below(root.clone(), "/", FsNode::Dir(0));

    build_tree_from_log(&args[0], root.clone());

    let mut sum: usize = 0;
    tree::traverse_tree_apply_if(root.clone(), is_dir, |node| {
        if let FsNode::Dir(size) = node.borrow().value {
            if size <= 100000 {
                sum += size
            }
        }
    });

    tree::print_tree(root);

    println!(
        "Sum of directory sizes with a total lower than 100000: {}",
        sum
    );
}

#[derive(Clone, PartialEq)]
enum FsNode {
    File(usize),
    Dir(usize),
}
impl fmt::Display for FsNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FsNode::File(size) => write!(f, "file, size={}", size),
            FsNode::Dir(size) => write!(f, "dir, size={}", size),
        }
    }
}

enum Command {
    Ls,
    Cd(String),
}

fn is_dir(node: FsNodeRef) -> bool {
    match node.borrow().value {
        FsNode::Dir(_) => true,
        _ => false,
    }
}

fn build_tree_from_log(filename: &str, root_node: FsNodeRef) {
    let mut cur_node = root_node.clone();
    io::foreach_line(filename, |line| {
        cur_node = parse_line(line, cur_node.clone()).unwrap_or(cur_node.clone());
    });

    // finally, we need to traverse the tree and add up the dir sizes internally
    tree::traverse_tree_apply_if(root_node, is_dir, |node| {
        let mut dir_sum: usize = 0;
        for child in node.clone().borrow().children.iter() {
            match tree::get_value(child) {
                FsNode::File(size) => dir_sum += size,
                FsNode::Dir(size) => dir_sum += size,
            }
        }
        tree::set_value(node.clone(), FsNode::Dir(dir_sum));
    });
}

fn parse_line(line: &str, node: FsNodeRef) -> Option<FsNodeRef> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if !line.is_empty() {
        match parts.first().copied() {
            Some(start) => match start {
                "$" => match parse_command(&parts[1..]) {
                    Command::Cd(dir) => return cd(node, &dir),
                    Command::Ls => {
                        // println!("$ ls");
                        ()
                    }
                },
                "dir" => {
                    if parts.len() == 2 {
                        // add a dir
                        tree::add_node_below(node.clone(), parts[1], FsNode::Dir(0));
                        // println!("dir {}", parts[1]);
                    } else {
                        panic!("Detected dir, but no name: {}", line)
                    }
                }
                _ => {
                    if let Ok(size) = start.parse::<usize>() {
                        if parts.len() == 2 {
                            // add a file
                            tree::add_node_below(node.clone(), parts[1], FsNode::File(size));
                            // println!("{} {}", size, parts[1]);
                        } else {
                            panic!("Detected file with size {}, but no name: {}", size, line);
                        }
                    } else {
                        panic!("Unrecognized start of string: {}", start);
                    }
                }
            },
            None => panic!("Invalid line: {}", line),
        }
    } else {
        panic!("Unexpected empty line.");
    }
    None
}

fn parse_command(parts: &[&str]) -> Command {
    match parts.len() {
        1 if parts[0] == "ls" => Command::Ls,
        2 if parts[0] == "cd" => Command::Cd(parts[1].to_string()),
        _ => panic!("Unrecognized command: {:?}", parts),
    }
}

fn cd(node: FsNodeRef, dir: &str) -> Option<FsNodeRef> {
    // println!("$ cd {}", dir);
    match dir {
        ".." => {
            if let Some(n) = tree::go_up(node) {
                Some(n)
            } else {
                panic!("failed to go up")
            }
        }
        _ => Some(tree::go_down(node, dir).unwrap()),
    }
}
