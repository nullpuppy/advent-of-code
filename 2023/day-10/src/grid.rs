use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use crate::{Direction, Tile};

pub struct Grid {
    pub tiles: Vec<Vec<Tile>>,
    pub start: Coord2d,
    // pub bounds: Dim2d,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord2d {
    pub x: usize,
    pub y: usize,
}

pub struct Dim2d {
    pub width: usize,
    pub height: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let coords = self.loop_coords();
        write!(f, "  ")?;
        for i in 0..self.tiles[0].len() {
            write!(f, "{}", i%10)?;
        }
        write!(f, "\n")?;
        for (y, row) in self.tiles.iter().enumerate() {
            write!(f, "{} ", y)?;
            for (x, t) in row.iter().enumerate() {
                if coords.contains(&Coord2d {x,y}) {
                    write!(f, "{}", t)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let coords = self.loop_coords();
        let inside = self.contained_coords();
        write!(f, "  ")?;
        for i in 0..self.tiles[0].len() {
            write!(f, "{}", i%10)?;
        }
        write!(f, "\n")?;
        for (y, row) in self.tiles.iter().enumerate() {
            write!(f, "{} ", y)?;
            for (x, t) in row.iter().enumerate() {
                if coords.contains(&Coord2d {x,y}) {
                    write!(f, "{}", t)?;
                } else if inside.contains(&Coord2d {x, y}){
                    write!(f, ".")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")
    }
}

impl Grid {
    pub fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut start = Coord2d::default();
        Grid {
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
        }
    }

    pub fn get_exits(&self, p: &Coord2d) -> Vec<(Coord2d, Direction)> {
        let mut exits = vec![];
        let bounds = Dim2d {
            height: self.tiles.len(),
            width: self.tiles[0].len(),
        };
        let dirs = match self.tiles[p.y][p.x] {
            Tile::Start => vec![Direction::North, Direction::South, Direction::West, Direction::East],
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
        self.loop_coords().len()/2
    }

    pub fn loop_coords(&self) -> HashSet<Coord2d> {
        let mut visited = HashSet::from([self.start]);
        let mut to_visit = VecDeque::from(self.get_exits(&self.start));

        loop {
            if let Some((c, dir)) = to_visit.pop_front() {
                if c == self.start {
                    break;
                }

                // Remove exits that go back to where we came from.
                let exits: Vec<_> = self.get_exits(&c).iter().filter(|(_, exit_dir)| match dir {
                    Direction::North => *exit_dir != Direction::South,
                    Direction::South => *exit_dir != Direction::North,
                    Direction::West => *exit_dir != Direction::East,
                    Direction::East => *exit_dir != Direction::West,
                })
                    .copied()
                    .collect();
                to_visit.extend(exits);
                visited.insert(c);
            } else {
                println!("Ran out of nodes to visit!");
                break;
            }
        }

        visited
    }

    pub fn contained_coords(&self) -> HashSet<Coord2d> {
        let loop_coords = self.loop_coords();
        let mut inside: HashSet<Coord2d> = HashSet::new();

        let exits = self.get_exits(&self.start);
        println!("First exit from {} is {:?}", self.start, exits[0]);
        println!("So we want to follow path starting at {}, and going {} but look at tiles going {}", exits[0].0, exits[0].1, exits[1].1);
        println!("If we end up at a bend, we update traversal_dir and search dir");

        let mut search_dir = exits[1].1;
        let mut traverse_dir = exits[0].1;
        let mut traversal_coord = exits[0].0;
        let bounds = Dim2d {
            height: self.tiles.len(),
            width: self.tiles[0].len(),
        };

        println!("Traverse {}, search {}", traverse_dir, search_dir);
        while traversal_coord != self.start {
            let mut search_coord = traversal_coord;
            let mut cnt = 0;
            loop {
                if let Some(next) = search_coord.adjust(&search_dir, &bounds) {
                    search_coord = next;
                } else {
                    println!("Next coord is None, breaking out.");
                    break;
                }

                // println!("Checking {} going {} {}", search_coord, search_dir, loop_coords.contains(&search_coord));
                cnt += 1;
                if loop_coords.contains(&search_coord) {
                    // println!("Breaking out of loop, found loop coord");
                    break;
                }
                inside.insert(search_coord);
            }

            traversal_coord = traversal_coord.adjust(&traverse_dir, &bounds).unwrap();
            if self.tile_at(&traversal_coord).is_bend() {
                match traverse_dir {
                    Direction::North => {
                        match self.tile_at(&traversal_coord) {
                            Tile::SWBend => {
                                traverse_dir = Direction::West;
                                search_dir = Direction::South;
                            },
                            Tile::SEBend => {
                                traverse_dir = Direction::East;
                                search_dir = Direction::South;
                            },
                            _ => unreachable!()
                        }
                    }
                    Direction::South => {
                        match self.tile_at(&traversal_coord) {
                            Tile::NWBend => {
                                traverse_dir = Direction::West;
                                search_dir = Direction::North;
                            },
                            Tile::NEBend => {
                                traverse_dir = Direction::East;
                                search_dir = Direction::North;
                            },
                            _ => unreachable!()
                        }
                    }
                    Direction::West => {
                        match self.tile_at(&traversal_coord) {
                            Tile::NEBend => {
                                traverse_dir = Direction::North;
                                search_dir = Direction::East;
                            },
                            Tile::SEBend => {
                                traverse_dir = Direction::South;
                                search_dir = Direction::East;
                            },
                            _ => unreachable!()
                        }
                    }
                    Direction::East => {
                        match self.tile_at(&traversal_coord) {
                            Tile::NWBend => {
                                traverse_dir = Direction::North;
                                search_dir = Direction::West;
                            },
                            Tile::SWBend => {
                                traverse_dir = Direction::South;
                                search_dir = Direction::West;
                            },
                            _ => unreachable!()
                        }
                    }
                }
                println!("Traverse {}, search {}", traverse_dir, search_dir);
            }
        }

        inside
    }
}

impl Coord2d {
    pub fn adjust(&self, dir: &Direction, bounds: &Dim2d) -> Option<Self> {
        let nx = match dir {
            Direction::West => self.x.saturating_sub(1),
            Direction::East => self.x.saturating_add(1),
            _ => self.x
        };
        let ny = match dir {
            Direction::North => self.y.saturating_sub(1),
            Direction::South => self.y.saturating_add(1),
            _ => self.y
        };

        if dir.is_n_s() && (ny == self.y || ny >= bounds.height) {
            // We're expecting y to change, but we get out of bounds
            return None;
        } else if dir.is_w_e() && (nx == self.x || nx >= bounds.width) {
            // We're expecting x to change, but we get out of bounds
            return None;
        }
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
