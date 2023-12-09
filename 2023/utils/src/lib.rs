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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_read_file() {
        let input = read_lines("test_input.txt").unwrap();
        assert_eq!(input.count(), 18);
    }

    #[test]
    fn test_bad_path_returns_error() {
        let input = read_lines("does-not-exist");
        assert!(input.is_err_and(|err| err.kind() == io::ErrorKind::NotFound));
    }
}