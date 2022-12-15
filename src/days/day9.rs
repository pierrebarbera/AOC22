use direction::Direction;
use io;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::From;
use std::fmt;

pub fn day9(args: &[String]) {
    if args.len() != 1 {
        panic!("Expecting exactly one arg to day9, which is a valid file path.");
    }

    let visited_first_rope = visited_coords(&args[0], 2);

    println!(
        "Number of coordinates visited by the tail of a rope with {} knots: {}",
        2,
        visited_first_rope.len()
    );
    // for c in visited_first_rope {
    //     println!("{}", c);
    // }

    let visited_second_rope = visited_coords(&args[0], 10);

    println!(
        "Number of coordinates visited by the tail of a rope with {} knots: {}",
        10,
        visited_second_rope.len()
    );
}

fn visited_coords(moveset_file: &str, knots: usize) -> HashSet<Coord> {
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut rope = vec![Coord { x: 0, y: 0 }; knots];
    visited.insert(rope.last().unwrap().clone());

    io::foreach_line(moveset_file, |line| {
        if !line.is_empty() {
            let (direction, steps) = line.trim().split_once(' ').unwrap();
            let steps: usize = steps.parse().unwrap();
            let direction: Direction = direction.into();

            for _ in 0..steps {
                rope.first_mut().unwrap().step(&direction);
                let mut prev_knot = rope.first().unwrap().clone();
                for knot in rope[1..].iter_mut() {
                    if !knot.touches(&prev_knot) {
                        knot.step_toward(&prev_knot);
                    }
                    prev_knot = knot.clone();
                }
                // track where the tail visits
                visited.insert(rope.last().unwrap().clone());
            }
        }
    });
    visited
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Coord {
    fn step(&mut self, d: &Direction) {
        match d {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn is_adjacent(&self, other: &Self) -> bool {
        let x_adjacent = self.x == other.x - 1 || self.x == other.x + 1;
        let y_adjacent = self.y == other.y - 1 || self.y == other.y + 1;

        x_adjacent && y_adjacent
            || x_adjacent && self.y == other.y
            || y_adjacent && self.x == other.x
    }

    fn touches(&self, other: &Self) -> bool {
        self == other || self.is_adjacent(other)
    }

    fn step_toward(&mut self, other: &Self) {
        self.x += step(self.x, other.x);
        self.y += step(self.y, other.y);
    }
}

fn step(d1: i32, d2: i32) -> i32 {
    match d1.cmp(&d2) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unrecognized direction: {}", s),
        }
    }
}
