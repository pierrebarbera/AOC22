use algorithm as algo;
use io;
use iter::window_iter::*;
use std::error;
use std::fmt;

pub fn day6(args: &[String]) {
    if args.len() != 1 {
        panic!("Expecting exactly one arg to day6, which is a valid file path.");
    }

    let (pos, marker) = find_first_unique::<4>(&args[0]).unwrap();

    println!(
        "Start-of-packet marker '{marker}' at position {pos}, meaning packet starts at character number {}",
        pos + marker.len()
    );

    let (pos, marker) = find_first_unique::<14>(&args[0]).unwrap();

    println!(
        "Start-of-packet message '{marker}' at position {pos}, meaning message starts at character number {}",
        pos + marker.len()
    );
}

#[derive(Debug, Clone)]
struct NoStart;
impl fmt::Display for NoStart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find start marker!")
    }
}
impl error::Error for NoStart {}

/*
   Opens a stream of elven communication and seek to the start-of-packet marker
*/
fn find_first_unique<const N: usize>(
    filename: &str,
) -> Result<(usize, String), Box<dyn std::error::Error>> {
    let mut reader = io::get_reader(filename)?;

    let iter = make_window_iter::<N>(&mut reader)?;

    for (pos, bytes) in iter.enumerate() {
        let s: String = bytes.map(|b| b as char).iter().collect();

        if algo::has_only_unique(bytes) {
            return Ok((pos, s));
        }
    }
    Err(NoStart.into())
}
