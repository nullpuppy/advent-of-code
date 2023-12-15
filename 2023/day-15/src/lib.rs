use std::fmt::{Debug, Formatter};

pub struct Lens {
    label: String,
    focal_length: usize,
}

impl Debug for Lens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length)
    }
}

impl Lens {
    pub fn new(label: &str, focal_length: usize) -> Self {
        Self {
            label: label.to_owned(),
            focal_length,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn focal_length(&self) -> usize {
        self.focal_length
    }

    pub fn set_focal_length(&mut self, focal_length: usize) {
        self.focal_length = focal_length;
    }
}

pub fn hash(s: &str) -> usize {
    // for each c in input
    // current_value += ascii_value(c)
    // current_value *= 17
    // current_value %= 256

    s.chars()
        .map(char_to_ascii)
        .fold(0, |acc, v| ((acc + v) * 17) % 256)
}

pub fn char_to_ascii(ch: char) -> usize {
    ch as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_HASH() {
        let input = "HASH";
        assert_eq!(52, hash(input));
    }

    #[test]
    fn test_char_to_ascii() {
        assert_eq!(72, char_to_ascii('H'));
        assert_eq!(65, char_to_ascii('A'));
        assert_eq!(83, char_to_ascii('S'));
        assert_eq!(61, char_to_ascii('='));
    }

    #[test]
    fn test_hash() {
        assert_eq!(0, hash("rn"));
        assert_eq!(0, hash("cm"));
        assert_eq!(1, hash("qp"));
        assert_eq!(3, hash("ot"));
        assert_eq!(3, hash("ab"));
        assert_eq!(3, hash("pc"));
    }
}
