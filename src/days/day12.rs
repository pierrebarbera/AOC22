use direction::Direction;
use io;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

pub fn day12(args: &[String]) {
    if args.len() != 1 {
        panic!("Expecting exactly one arg to day12, which is a valid file path.");
    }

    let mut map = parse_map(&args[0]);

    assert!(!map.is_empty());
    assert_eq!(map.y_end, map.map.len());
    assert_eq!(map.x_end, map.map[0].len());
    // dbg!(map.y_end);
    // dbg!(map.x_end);

    // print!("{}", map);

    let length = dijkstra(&mut map);

    println!("Shortest path length from S to E: {}", length.unwrap());
}

fn parse_map(filename: &str) -> HeightMap {
    let mut map: HeightMap = HeightMap::new();
    let mut y: usize = 0;
    let mut x_len: Option<usize> = None;
    io::foreach_nonempty_line(filename, |line| {
        if x_len.is_none() {
            x_len = Some(line.len())
        } else if let Some(l) = x_len {
            assert_eq!(l, line.len());
        }
        let mut row = vec![Grid::new(); line.len()];
        for (x, c) in line.chars().enumerate() {
            row[x].height = to_height(c);

            if c == 'S' {
                map.my_pos = Coord::new(x, y);
                map.my_pos.distance = 0;
                row[x].distance = 0;
            }
            if c == 'E' {
                map.target_pos = Coord::new(x, y);
            }
        }
        map.map.push(row);
        y += 1;
    });
    map.x_end = x_len.unwrap();
    map.y_end = y;
    map
}

fn to_height(c: char) -> u8 {
    match c {
        'S' => to_height('a'),
        'E' => to_height('z'),
        _ => c.to_digit(36).unwrap() as u8,
    }
}

#[derive(Default)]
struct HeightMap {
    pub map: Vec<Vec<Grid>>,
    pub x_end: usize,
    pub y_end: usize,
    pub my_pos: Coord,
    pub target_pos: Coord,
}

impl HeightMap {
    fn new() -> Self {
        Default::default()
    }
    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    fn at(&self, coord: &Coord) -> &Grid {
        assert!(self.is_inside(coord.x as i32, coord.y as i32));
        &self.map[coord.y][coord.x]
    }
    fn at_mut(&mut self, coord: &Coord) -> &mut Grid {
        assert!(self.is_inside(coord.x as i32, coord.y as i32));
        &mut self.map[coord.y][coord.x]
    }
    fn is_inside(&self, x: i32, y: i32) -> bool {
        (0..self.y_end as i32).contains(&y) && (0..self.x_end as i32).contains(&x)
    }
    /// Get the neighbour, if any, in the desired direction
    fn neighbour(&self, coord: &Coord, dir: &Direction) -> Option<Coord> {
        // NOTE: in this case, up and down are reversed, so up would be y - 1
        match dir {
            Direction::Up => {
                if (coord.y as i32 - 1) >= 0 {
                    Some(Coord::new(coord.x, coord.y - 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if (coord.y as i32 + 1) < self.y_end as i32 {
                    Some(Coord::new(coord.x, coord.y + 1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if (coord.x as i32 - 1) >= 0 {
                    Some(Coord::new(coord.x - 1, coord.y))
                } else {
                    None
                }
            }
            Direction::Right => {
                if (coord.x as i32 + 1) < self.x_end as i32 {
                    Some(Coord::new(coord.x + 1, coord.y))
                } else {
                    None
                }
            }
        }
    }
    /// Get a vector of coordinates that are possible next moves in the map
    fn get_neighbours_if<F>(&self, cur: &Coord, mut predicate: F) -> Vec<Coord>
    where
        F: FnMut(&Coord) -> bool,
    {
        let mut neighbours: Vec<Coord> = Vec::new();
        for dir in Direction::iter() {
            if let Some(next) = self.neighbour(&cur, dir) {
                if predicate(&next) {
                    neighbours.push(next);
                }
            }
        }
        neighbours
    }
    #[allow(dead_code)]
    fn foreach_coord<F>(&mut self, coordinates: &Vec<Coord>, mut f: F)
    where
        F: FnMut(&mut Grid),
    {
        for c in coordinates {
            f(&mut self.at_mut(&c));
        }
    }
}

impl fmt::Display for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            for (x, grid) in row.iter().enumerate() {
                let cur_pos = Coord::new(x, y);
                if cur_pos == self.my_pos {
                    write!(f, "[🧝]")?;
                } else if cur_pos == self.target_pos {
                    write!(f, "[🚩]")?;
                } else {
                    write!(f, "[{:#02}]", grid.height)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Default, Clone)]
struct Grid {
    height: u8,
    visited: bool,
    distance: usize,
}
impl Grid {
    fn new() -> Self {
        let mut res: Self = Default::default();
        res.distance = usize::MAX;
        res
    }
}

#[derive(Default, Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
    distance: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord {
            x,
            y,
            distance: usize::MAX,
        }
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coord {}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(map: &mut HeightMap) -> Option<usize> {
    // make sure to reset visited values
    for row in map.map.iter_mut() {
        for grid in row.iter_mut() {
            grid.visited = false;
        }
    }
    let target = map.target_pos.clone();

    let mut unvisited = BinaryHeap::new();
    unvisited.push(map.my_pos.clone());

    while let Some(cur) = unvisited.pop() {
        if cur == target {
            return Some(cur.distance);
        }

        let cur_distance = map.at(&cur).distance;
        // get the neighbours of the current
        let mut neighbours = map.get_neighbours_if(&cur, |neighbour| {
            // a valid neighbour has have a distance greater than one farther than the current
            // (otherwise that grid already has a shorther path to it)
            map.at(&neighbour).distance > cur_distance + 1
            // and has to not have been visited yet
                && !map.at(&neighbour).visited
            // and it has to be no higher than one step
                && ((map.at(neighbour).height == map.at(&cur).height + 1)
                // or any lower distance
                    || map.at(neighbour).height <= map.at(&cur).height)
        });

        // set the new, lower distances of the neighboring grids
        for c in neighbours.iter_mut() {
            c.distance = cur_distance + 1;
            map.at_mut(&c).distance = c.distance;
        }

        // mark current as visited
        map.at_mut(&cur).visited = true;

        // add the neighbours to the unvisited queue
        unvisited.extend(neighbours);
    }

    None
}
