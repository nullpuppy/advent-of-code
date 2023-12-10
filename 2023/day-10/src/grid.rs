use crate::{Direction, Tile};
use checked_clamp::CheckedClamp;
use colored::*;
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fmt::Formatter;

#[derive(Default)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
    start: Coord2d,
    loop_coords: HashSet<Coord2d>,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord2d {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Dim2d {
    pub width: usize,
    pub height: usize,
}

impl Grid {
    /// Build a grid and find the loop within. Assumes the `Start` node con only connect
    /// to two of it's adjacent nodes.
    pub fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut start = Coord2d::default();
        let mut grid = Self {
            tiles: input
                .enumerate()
                .map(|(r, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(c, ch)| {
                            if ch == 'S' {
                                start.x = c;
                                start.y = r;
                            }
                            Tile::from(ch)
                        })
                        .collect()
                })
                .collect(),
            start,
            ..Self::default()
        };
        grid.find_loop();
        grid
    }

    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    /// Get adjacent tiles that can be part of a loop
    pub fn get_exits(&self, p: &Coord2d) -> Vec<(Coord2d, Direction)> {
        let mut exits = vec![];
        let bounds = Dim2d {
            height: self.tiles.len(),
            width: self.tiles[0].len(),
        };
        let dirs = match self.tiles[p.y][p.x] {
            Tile::Start => vec![
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ],
            Tile::NEBend => vec![Direction::North, Direction::East],
            Tile::NWBend => vec![Direction::North, Direction::West],
            Tile::SWBend => vec![Direction::South, Direction::West],
            Tile::SEBend => vec![Direction::South, Direction::East],
            Tile::Vertical => vec![Direction::North, Direction::South],
            Tile::Horizontal => vec![Direction::West, Direction::East],
            _ => {
                // Anything else, just ignore.
                vec![]
            }
        };
        for dir in dirs {
            if let Some(c) = p.adjust(&dir, &bounds) {
                let t = self.tile_at(&c);
                if Tile::Start.can_connect(t, dir) {
                    exits.push((c, dir));
                }
            }
        }

        exits
    }

    pub fn tile_at(&self, p: &Coord2d) -> &Tile {
        &self.tiles[p.y][p.x]
    }

    pub fn max_loop_distance(&self) -> usize {
        self.loop_coords.len() / 2
    }

    /// Find set of coordinates that make up the loop contained within the grid.
    pub fn find_loop(&mut self) {
        let mut loop_coords = HashSet::from([self.start]);
        let mut to_visit = VecDeque::from(self.get_exits(&self.start));

        loop {
            if let Some((c, dir)) = to_visit.pop_front() {
                if c == self.start {
                    break;
                }

                // Remove exits that go back to where we came from.
                let exits: Vec<_> = self
                    .get_exits(&c)
                    .iter()
                    .filter(|(_, exit_dir)| match dir {
                        Direction::North => *exit_dir != Direction::South,
                        Direction::South => *exit_dir != Direction::North,
                        Direction::West => *exit_dir != Direction::East,
                        Direction::East => *exit_dir != Direction::West,
                    })
                    .copied()
                    .collect();
                to_visit.extend(exits);
                loop_coords.insert(c);
            } else {
                println!("Ran out of nodes to visit!");
                break;
            }
        }

        self.loop_coords = loop_coords;
    }

    /// Isolate loop cells in the grid by replacing everything that isn't the loop with the
    /// specified tile. This fills both areas outside the loop, and areas contained within.
    pub fn isolate_loop(&mut self, tile: Tile) {
        let width = self.tiles[0].len();
        let height = self.tiles.len();
        for r in 0..height {
            for c in 0..width {
                if !self.loop_coords.contains(&Coord2d::new(c, r)) {
                    self.tiles[r][c] = tile;
                }
            }
        }
    }

    /// Fill the grid outside the loop with the specified tile.
    pub fn flood_fill(&mut self, fill: Tile) {
        let bounds = Dim2d {
            width: self.tiles[0].len(),
            height: self.tiles.len(),
        };

        for r in 0..bounds.height {
            let mut prev_bend: Option<Tile> = None;
            let mut pipe_cnt = 0;
            for c in 0..bounds.width {
                if self.loop_coords.contains(&Coord2d::new(c, r)) {
                    let tile = self.tile_at(&Coord2d::new(c, r));
                    pipe_cnt += match tile {
                        Tile::Vertical | Tile::Start => 1,
                        Tile::NWBend => {
                            if prev_bend == Some(Tile::NEBend) {
                                -1 // treat u bends as if they don't exist
                            } else if prev_bend == Some(Tile::SEBend) {
                                0 // Treat curve as a single boundary
                            } else {
                                1
                            }
                        }
                        Tile::NEBend => 1,
                        Tile::SWBend => {
                            if prev_bend == Some(Tile::SEBend) {
                                -1 // Treat u bends as if they don't exist
                            } else if prev_bend == Some(Tile::NEBend) {
                                0 // Treat curve as a single boundary
                            } else {
                                1
                            }
                        }
                        Tile::SEBend => 1,
                        _ => 0,
                    };
                    if tile.is_bend() {
                        prev_bend = Some(tile.to_owned());
                    } else if tile == &Tile::Vertical {
                        prev_bend = None;
                    }
                } else if pipe_cnt % 2 == 0
                    && self.tile_at(&Coord2d::new(c, r)) == &Tile::Ground
                {
                    self.tiles[r][c] = fill;
                }
            }
        }
    }

    /// Print out the current state, optionally highlighting a specific coordinate
    /// and direction of traversal
    pub fn print_debug(&self, current: Option<&Coord2d>, dir: Option<&Direction>) {
        let loop_coords = &self.loop_coords;
        print!("   ");
        for i in 0..self.tiles[0].len() {
            print!("{}", i % 10);
        }
        print!("\n");
        for (y, row) in self.tiles.iter().enumerate() {
            print!("{:3} ", y);
            for (x, t) in row.iter().enumerate() {
                let c = Coord2d::new(x, y);
                if current == Some(&c) {
                    print!(
                        "{}",
                        format!(
                            "{}",
                            match dir {
                                Some(d) => format!("{}", d),
                                None => format!("{}", t),
                            }
                        )
                        .green()
                    );
                } else if loop_coords.contains(&c) {
                    print!("{}", format!("{}", t).blue());
                } else {
                    print!("{}", format!("{}", t).yellow());
                }
            }
            println!();
        }
        println!();
    }
}

impl Coord2d {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn adjust(&self, dir: &Direction, bounds: &Dim2d) -> Option<Self> {
        let Some(nx) = (match dir {
            Direction::West => self.x.checked_sub(1),
            Direction::East => self.x.checked_add(1).and_then(|x| {
                x.checked_clamp(0usize, bounds.width - 1)
                    .map_or(None, |x| Some(x))
            }),
            _ => Some(self.x),
        }) else {
            // Overflowed
            return None;
        };

        let Some(ny) = (match dir {
            Direction::North => self.y.checked_sub(1),
            Direction::South => self.y.checked_add(1).and_then(|y| {
                y.checked_clamp(0usize, bounds.height - 1)
                    .map_or(None, |y| Some(y))
            }),
            _ => Some(self.y),
        }) else {
            // Overflowed
            return None;
        };

        Some(Self { x: nx, y: ny })
    }
}

impl fmt::Display for Coord2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Coord2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
