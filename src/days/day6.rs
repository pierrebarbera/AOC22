use std::collections::HashSet;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

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
    // let reader = io::get_reader(filename);

    let file = File::open(filename)?;
    let mut reader = io::BufReader::new(file);
    let iter = make_window_iter::<N>(&mut reader)?;

    for (pos, bytes) in iter.enumerate() {
        let s: String = bytes.map(|b| b as char).iter().collect();

        if has_only_unique(bytes) {
            return Ok((pos, s));
        }
    }
    Err(NoStart.into())
}

fn has_only_unique<T, I>(a: T) -> bool
where
    T: IntoIterator<Item = I>,
    I: Eq + std::hash::Hash,
{
    let mut items: HashSet<I> = HashSet::new();
    for n in a {
        if items.contains(&n) {
            return false;
        } else {
            items.insert(n);
        }
    }
    true
}

// iterator that moves over a bufreader in windows / kmers

struct WindowIter<'a, T, const N: usize>
where
    T: BufRead,
{
    reader: &'a mut T,
    window: [u8; N],
    first: bool,
}

impl<'a, T, const N: usize> Iterator for WindowIter<'a, T, N>
where
    T: BufRead,
{
    type Item = [u8; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            match self.reader.read_exact(&mut self.window) {
                Ok(_) => Some(self.window.clone()),
                Err(_) => None,
            }
        } else {
            let mut buf = [0; 1];
            match self.reader.read_exact(&mut buf) {
                Ok(_) => {
                    // change the array in place
                    let window = &mut self.window;
                    for i in 0..N - 1 {
                        window[i] = window[i + 1];
                    }
                    window[N - 1] = buf[0];
                    // return a clone
                    Some(window.clone())
                }
                _ => None,
            }
        }
    }
}

fn make_window_iter<const N: usize>(
    reader: &mut io::BufReader<File>,
) -> Result<WindowIter<io::BufReader<File>, N>, io::Error> {
    let iter: WindowIter<io::BufReader<File>, N> = WindowIter {
        reader: reader,
        window: [0; N],
        first: true,
    };
    Ok(iter)
}
