use io;
use itertools::Either;
use std::cmp::max;
use std::convert::TryInto;
use std::fmt;
use std::iter::Rev;
use std::ops::Range;

#[derive(Clone, Debug)]
struct Directional {
    up: i32,
    down: i32,
    left: i32,
    right: i32,
}

impl fmt::Display for Directional {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({},{},{},{})",
            self.left, self.down, self.up, self.right
        )
    }
}

pub fn day8(args: &[String]) {
    if args.len() != 1 {
        panic!("Expecting exactly one arg to day8, which is a valid file path.");
    }

    let map = parse_map(&args[0]);

    let mut viz_mat: Vec<Vec<Directional>> = vec![
        vec![
            Directional {
                up: -1,
                down: -1,
                left: -1,
                right: -1,
            };
            map[0].len()
        ];
        map.len()
    ];

    foreach_dp_step(
        &map,
        DpDirection::DownRight,
        |row, col, prev_row, prev_col| {
            viz_mat[row][col].up = max(viz_mat[prev_row][col].up, map[prev_row][col]);
            viz_mat[row][col].left = max(viz_mat[row][prev_col].left, map[row][prev_col]);
        },
    );

    foreach_dp_step(&map, DpDirection::UpLeft, |row, col, prev_row, prev_col| {
        viz_mat[row][col].down = max(viz_mat[prev_row][col].down, map[prev_row][col]);
        viz_mat[row][col].right = max(viz_mat[row][prev_col].right, map[row][prev_col]);
    });

    // print_matrix(&map);
    // print_matrix(&viz_mat);

    // finally, go through each cell in the map and check the visibility maps
    // to see if that tree is visible
    let mut sum_visible_trees: u32 = 0;
    for (row, map_row) in map.iter().enumerate() {
        for (col, map_val) in map_row.iter().enumerate() {
            if is_visible(*map_val, &viz_mat[row][col]) {
                sum_visible_trees += 1;
            }
        }
    }

    println!("Number of visible trees on the map: {}", sum_visible_trees);
}

fn is_visible(height: i32, visibility: &Directional) -> bool {
    height > visibility.up
        || height > visibility.down
        || height > visibility.left
        || height > visibility.right
}

#[allow(dead_code)]
fn print_matrix<T: std::fmt::Display>(mat: &Vec<Vec<T>>) {
    for row in mat {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }
}

fn parse_map(filename: &str) -> Vec<Vec<i32>> {
    let mut map: Vec<Vec<i32>> = Vec::new();

    io::foreach_line(filename, |line| {
        if !line.is_empty() {
            let mut row: Vec<i32> = Vec::new();
            for c in line.chars() {
                row.push(
                    c.to_digit(10)
                        .unwrap_or_else(|| panic!("Unrecognized char: {}", c))
                        .try_into()
                        .unwrap(),
                );
            }
            map.push(row);
        }
    });

    map
}

enum DpDirection {
    DownRight,
    UpLeft,
}

fn get_prev_row_col(row: usize, col: usize, d: &DpDirection) -> (usize, usize) {
    match d {
        DpDirection::DownRight => (row - 1, col - 1),
        DpDirection::UpLeft => (row + 1, col + 1),
    }
}

fn get_iter_ranges(
    m: usize,
    n: usize,
    d: &DpDirection,
) -> (
    Either<Range<usize>, Rev<Range<usize>>>,
    Either<Range<usize>, Rev<Range<usize>>>,
) {
    match d {
        DpDirection::DownRight => (Either::Left(1..m), Either::Left(1..n)),
        DpDirection::UpLeft => (
            Either::Right((0..m - 1).rev()),
            Either::Right((0..n - 1).rev()),
        ),
    }
}

fn foreach_dp_step<T, F>(mat: &Vec<Vec<T>>, d: DpDirection, mut f: F)
where
    F: FnMut(usize, usize, usize, usize),
{
    let m = mat.len();
    let n = mat[0].len();

    let (row_range, col_range) = get_iter_ranges(m, n, &d);

    for row in row_range {
        // assert matrix
        assert_eq!(n, mat[row].len());
        for col in col_range.clone() {
            let (prev_row, prev_col) = get_prev_row_col(row, col, &d);
            f(row, col, prev_row, prev_col);
        }
    }
}
