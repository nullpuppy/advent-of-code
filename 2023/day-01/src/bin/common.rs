use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap_or_default())
    )
}
