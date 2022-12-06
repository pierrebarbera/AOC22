use std::fs::File;
use std::io::{self, BufRead};

// iterator that moves over a bufreader in windows / kmers
pub struct WindowIter<'a, T, const N: usize>
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

pub fn make_window_iter<const N: usize>(
    reader: &mut io::BufReader<File>,
) -> Result<WindowIter<io::BufReader<File>, N>, io::Error> {
    let iter: WindowIter<io::BufReader<File>, N> = WindowIter {
        reader: reader,
        window: [0; N],
        first: true,
    };
    Ok(iter)
}
