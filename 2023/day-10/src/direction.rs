use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn is_n_s(&self) -> bool {
        match self {
            Direction::North | Direction::South => true,
            _ => false,
        }
    }

    pub fn is_w_e(&self) -> bool {
        match self {
            Direction::West | Direction::East => true,
            _ => false,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Direction::North => "N",
            Direction::South => "S",
            Direction::West => "W",
            Direction::East => "E",
        })
    }
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}