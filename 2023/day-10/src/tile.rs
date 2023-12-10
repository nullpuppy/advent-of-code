use crate::Direction;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Tile {
    Start,      // Can Exit N, S, W, E
    Vertical,   // Can Exit N, S
    Horizontal, // Can Exit W, E
    NEBend,     // Can Exit N, E
    NWBend,     // Can Exit N, W
    SWBend,     // Can Exit S, W
    SEBend,     // Can Exit S, E
    Ground,     // Can Exit None.
    Void,       // Can Exit None
}

impl Tile {
    pub fn is_bend(&self) -> bool {
        match self {
            Tile::NWBend | Tile::NEBend | Tile::SWBend | Tile::SEBend => true,
            _ => false,
        }
    }

    pub fn can_connect(&self, t: &Tile, direction: Direction) -> bool {
        if *self == Tile::Ground || *t == Tile::Ground {
            return false;
        }

        match direction {
            // ?    ?
            // S    |
            Direction::North => {
                if [Tile::SWBend, Tile::SEBend, Tile::Horizontal].contains(self) {
                    return false;
                }
                if [Tile::SWBend, Tile::SEBend, Tile::Vertical, Tile::Start].contains(t) {
                    return true;
                }
            }
            // S
            // ?
            Direction::South => {
                if [Tile::NWBend, Tile::NEBend, Tile::Horizontal].contains(self) {
                    return false;
                }
                if [Tile::NWBend, Tile::NEBend, Tile::Vertical, Tile::Start].contains(t) {
                    return true;
                }
            }
            // ?S
            Direction::West => {
                if [Tile::NEBend, Tile::SEBend, Tile::Vertical].contains(self) {
                    return false;
                }
                if [Tile::NEBend, Tile::SEBend, Tile::Horizontal, Tile::Start].contains(t) {
                    return true;
                }
            }
            // S?
            Direction::East => {
                if [Tile::NWBend, Tile::SWBend, Tile::Vertical].contains(self) {
                    return false;
                }
                if [Tile::NWBend, Tile::SWBend, Tile::Horizontal, Tile::Start].contains(t) {
                    return true;
                }
            }
        }
        false
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Start => '\u{25cf}', //'S',
                Tile::Vertical => '\u{2502}',
                Tile::Horizontal => '\u{2500}',
                Tile::NEBend => '\u{2514}', // "L"
                Tile::NWBend => '\u{2518}', // "J"
                Tile::SWBend => '\u{2510}', // "7",
                Tile::SEBend => '\u{250c}', // "F"
                Tile::Ground => '\u{25e6}', // '.',
                Tile::Void => ' ',
            }
        )
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Ground,
            'S' => Tile::Start,
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NEBend,
            'J' => Tile::NWBend,
            '7' => Tile::SWBend,
            'F' => Tile::SEBend,
            _ => unreachable!(),
        }
    }
}
