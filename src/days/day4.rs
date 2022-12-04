use io;
use std::error::Error;
use std::fmt;

pub fn day4(args: &[String]) {
    if args.len() != 1 {
        panic!("Expecting exactly one arg to day4, which is a valid file path.");
    }

    println!(
        "Number of full range overlaps: {}",
        number_full_overlaps(&args[0])
    );

    println!(
        "Number of any range overlaps: {}",
        number_any_overlaps(&args[0])
    );
}

fn number_full_overlaps(filename: &str) -> u32 {
    let mut count: u32 = 0;
    foreach_rangepair(filename, |lhs, rhs| {
        if lhs.contains(&rhs) || rhs.contains(&lhs) {
            count += 1;
        }
    });
    count
}

fn number_any_overlaps(filename: &str) -> u32 {
    let mut count = 0;
    foreach_rangepair(filename, |lhs, rhs| {
        if lhs.overlaps(&rhs) {
            count += 1
        }
    });
    count
}

fn foreach_rangepair<F>(filename: &str, mut f: F)
where
    F: FnMut(&Range, &Range),
{
    io::foreach_line(filename, |line| {
        let (lhs, rhs) = line
            .split_once(',')
            .unwrap_or_else(|| panic!("Cannot split line on ',': {}", line));

        let lhs_range = Range::from_str(lhs).unwrap();
        let rhs_range = Range::from_str(rhs).unwrap();

        f(&lhs_range, &rhs_range);
    });
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn from_str(input: &str) -> Result<Range, RangeInvalidErr> {
        match input.split_once('-') {
            Some((lhs, rhs)) => Range::from_strs(lhs, rhs),
            None => Err(RangeInvalidErr {
                msg: format!("Cannot split '{}' on '-'", input),
            }),
        }
    }
    fn from_strs(lhs: &str, rhs: &str) -> Result<Range, RangeInvalidErr> {
        match (lhs.parse::<u32>(), rhs.parse::<u32>()) {
            (Ok(start), Ok(end)) => Ok(Range { start, end }),
            (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => Err(RangeInvalidErr {
                msg: format!("Failed to parse number '{}-{}': {}", lhs, rhs, e),
            }),
        }
    }
    fn contains(&self, other: &Range) -> bool {
        (other.start >= self.start) && (other.end <= self.end)
    }
    fn overlaps(&self, other: &Range) -> bool {
        other.contains(self) || self.has(other.start) || self.has(other.end)
    }
    fn has(&self, n: u32) -> bool {
        (n >= self.start) && (n <= self.end)
    }
}

#[derive(Debug, Clone)]
struct RangeInvalidErr {
    msg: String,
}
impl Error for RangeInvalidErr {}
impl fmt::Display for RangeInvalidErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not convert to Range : {}", self.msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_contains() {
        let r1 = Range { start: 0, end: 5 };
        let r2 = Range { start: 1, end: 4 };
        assert_eq!(true, r1.contains(&r2));
        assert_eq!(false, r2.contains(&r1));
        let r3 = Range { start: 0, end: 5 };
        assert_eq!(true, r1.contains(&r3));
        assert_eq!(true, r3.contains(&r1));

        let r4 = Range { start: 6, end: 6 };
        let r5 = Range { start: 4, end: 6 };
        assert_eq!(true, r5.contains(&r4));
    }
    #[test]
    fn test_range_overlaps() {
        let r1 = Range { start: 0, end: 5 };
        let r2 = Range { start: 1, end: 4 };
        assert_eq!(true, r1.overlaps(&r2));
        assert_eq!(true, r2.overlaps(&r1));
        let r3 = Range { start: 5, end: 10 };
        assert_eq!(true, r1.overlaps(&r3));
        assert_eq!(true, r3.overlaps(&r1));

        let r4 = Range { start: 0, end: 5 };
        let r5 = Range { start: 6, end: 6 };
        assert_eq!(false, r5.overlaps(&r4));
        assert_eq!(false, r4.overlaps(&r5));
    }
    #[test]
    fn test_from_str() {
        let r1 = Range::from_str("1-4").unwrap();
        let r2 = Range::from_str("365-786").unwrap();
        assert_eq!(1, r1.start);
        assert_eq!(4, r1.end);
        assert_eq!(365, r2.start);
        assert_eq!(786, r2.end);
    }

    #[test]
    fn test_from_strs() {
        let r1 = Range::from_strs("1", "4").unwrap();
        let r2 = Range::from_strs("365", "786").unwrap();
        assert_eq!(1, r1.start);
        assert_eq!(4, r1.end);
        assert_eq!(365, r2.start);
        assert_eq!(786, r2.end);
    }
}
