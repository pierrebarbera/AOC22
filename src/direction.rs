use self::Direction::*;
use std::slice::Iter;

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Down
    }
}

impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Up, Left, Down, Right];
        DIRECTIONS.iter()
    }
    #[allow(dead_code)]
    /// Returns the opposite direction
    pub fn opposite(&self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Right => Left,
            Left => Right,
        }
    }
}
