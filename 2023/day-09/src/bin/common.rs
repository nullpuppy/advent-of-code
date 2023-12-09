use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap_or_default()))
}
pub struct OasisValue {
    seq: Vec<Vec<isize>>,
}

impl OasisValue {
    pub fn from_string(s: &String, reverse: bool) -> Self {
        let mut seq: Vec<isize> = s
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        if reverse {
            seq = seq.iter().copied().rev().collect();
        }
        Self { seq: vec![seq] }
    }

    pub fn seq(&self) -> &Vec<Vec<isize>> {
        &self.seq
    }

    pub fn build_sequences(&mut self) {
        loop {
            self.seq.push(self.step_differences());
            // Once we have all zeros, time to move on to the next thing.
            if self
                .seq
                .last()
                .unwrap()
                .iter()
                .map(|v| v.abs())
                .sum::<isize>()
                == 0
            {
                break;
            }
        }
    }

    pub fn extrapolate(&mut self) {
        let mut next = 0;
        for seq in self.seq.iter_mut().rev() {
            next += seq.last().unwrap().to_owned();
            seq.push(next);
        }
    }

    pub fn get_next(&self) -> isize {
        self.seq.first().unwrap().last().unwrap().to_owned()
    }

    pub fn step_differences(&self) -> Vec<isize> {
        self.seq
            .last()
            .unwrap()
            .as_slice()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect()
    }
}
