use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "\u{2191}", // Up arrow
                Direction::South => "\u{2193}", // Down arrow
                Direction::West => "\u{2190}",  // Left arrow
                Direction::East => "\u{2192}",  // Right arrow
            }
        )
    }
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
