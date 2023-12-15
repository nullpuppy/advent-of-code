use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Eq)]
pub enum Rock {
    RoundRock,
    CubeRock,
    EmptyGround,
    Fill,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Display for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rock::CubeRock => '#', // TODO unicode
                Rock::RoundRock => 'O',
                Rock::EmptyGround => '.',
                Rock::Fill => ' ',
            }
        )
    }
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            'O' => Rock::RoundRock,
            '#' => Rock::CubeRock,
            '.' => Rock::EmptyGround,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Platform {
    platform: Vec<Vec<Rock>>,
    width: usize,
    height: usize,
}

impl Debug for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        write!(f, "    ")?;
        for n in 0..self.width {
            write!(f, "{}", n % 10)?;
        }
        writeln!(f)?;

        for r in 0..self.height {
            write!(f, "{r:3} ")?;
            for c in 0..self.width {
                write!(f, "{}", self.platform[r][c])?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Platform {
    pub fn new(platform: Vec<Vec<Rock>>) -> Self {
        let width = platform[0].len();
        let height = platform.len();
        Self {
            platform,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn process_column(&mut self, c: usize, dir: &Direction) {
        let mut count = 0;
        if dir == &Direction::North {
            for r in (0..self.height).rev() {
                if self.platform[r][c] == Rock::RoundRock {
                    self.platform[r][c] = Rock::EmptyGround;
                    count += 1;
                }
                if r == 0 || self.platform[r][c] == Rock::CubeRock {
                    if count != 0 {
                        self.place_rocks_in_column(r, c, count, dir);
                    }
                    count = 0;
                }
            }
        } else {
            for r in 0..self.height {
                if self.platform[r][c] == Rock::RoundRock {
                    self.platform[r][c] = Rock::EmptyGround;
                    count += 1;
                }
                if r == self.height-1 || self.platform[r][c] == Rock::CubeRock {
                    if count != 0 {
                        self.place_rocks_in_column(r, c, count, dir);
                    }
                    count = 0;
                }
            }
        }
    }

    fn place_rocks_in_column(&mut self, r: usize, c: usize, count: usize, dir: &Direction) {
        if dir == &Direction::North {
            let offset = if self.platform[r][c] == Rock::CubeRock {
                1
            } else {
                0
            };
            for i in r + offset..r + offset + count {
                self.platform[i][c] = Rock::RoundRock;
            }
        } else {
            let offset = if self.platform[r][c] == Rock::CubeRock {
                0
            } else {
                1
            };
            for i in r+offset-count..r+offset {
                self.platform[i][c] = Rock::RoundRock;
            }
        }
    }

    fn place_rocks_in_row(&mut self, r: usize, c: usize, count: usize, dir: &Direction) {
        // Tilting West, so processing right to left
        if dir == &Direction::West {
            let offset = if self.platform[r][c] == Rock::CubeRock {
                1
            } else {
                0
            };
            for i in c + offset..c + offset + count {
                if self.platform[r][i] == Rock::CubeRock {
                    panic!("Trying to overwrite a cube! Going West {r},{i}");
                }
                self.platform[r][i] = Rock::RoundRock;
            }
        } else {
            let offset = if self.platform[r][c] == Rock::CubeRock {
                0
            } else {
                1
            };
            for i in c+offset-count..c + offset {
                if self.platform[r][i] == Rock::CubeRock {
                    panic!("Trying to overwrite a cube! Going East {r},{i}");
                }
                self.platform[r][i] = Rock::RoundRock;
            }
        }
    }

    ///
    fn process_row(&mut self, r: usize, dir: &Direction) {
        let mut count = 0;
        if dir == &Direction::West {
            for c in (0..self.width).rev() {
                if self.platform[r][c] == Rock::RoundRock {
                    self.platform[r][c] = Rock::EmptyGround;
                    count += 1;
                }
                if c == 0 || self.platform[r][c] == Rock::CubeRock {
                    if count != 0 {
                        self.place_rocks_in_row(r, c, count, dir);
                    }
                    count = 0;
                }
            }
        } else {
            for c in 0..self.width {
                if self.platform[r][c] == Rock::RoundRock {
                    self.platform[r][c] = Rock::EmptyGround;
                    count += 1;
                }
                if c == self.width-1 || self.platform[r][c] == Rock::CubeRock {
                    if count != 0 {
                        self.place_rocks_in_row(r, c, count, dir);
                    }
                    count = 0;
                }
            }
        }
    }

    pub fn new_tilt(&mut self, direction: &Direction) {
        match direction {
            Direction::North | Direction::South => {
                for c in 0..self.width {
                    self.process_column(c, direction);
                }
            }
            Direction::West => {
                for r in (0..self.height).rev() {
                    self.process_row(r, direction);
                }
            }
            Direction::East => {
                for r in 0..self.height() {
                    self.process_row(r, direction);
                }
            }
        }
    }

    pub fn tilt_old(&mut self) {
        // we might want to start at the end.
        // and iterate backwards. count RoundRocks until we get to 0 or a Cube rock,
        // update relevant rows, and continue.
        for c in 0..self.width {
            let mut count = 0;
            for r in (0..self.height).rev() {
                if self.platform[r][c] == Rock::RoundRock {
                    self.platform[r][c] = Rock::EmptyGround;
                    count += 1;
                }
                if r == 0 || self.platform[r][c] == Rock::CubeRock {
                    if count != 0 {
                        let offset = if self.platform[r][c] == Rock::CubeRock {
                            1
                        } else {
                            0
                        };
                        for i in r + offset..r + offset + count {
                            self.platform[i][c] = Rock::RoundRock;
                        }
                        // Populate r to r + count with RoundRock,
                        // replace all RoundRock in start to end with ground
                    }
                    count = 0;
                }
            }
        }
    }

    pub fn cycle(&mut self, cycles: usize) {
        for n in 0..cycles {
            for dir in [Direction::North, Direction::West, Direction::South, Direction::East] {
                self.new_tilt(&dir);
            }
            if n % 100_000 == 0 {
                println!("Processed {} cycles", n+1);
            }
        }
    }

    pub fn total_load(&self) -> usize {
        let mut sum = 0;
        for (r, line) in self.platform.iter().enumerate() {
            let line_sum: usize = line
                .iter()
                .filter(|&v| v == &Rock::RoundRock)
                .map(|_v| 1)
                .sum();
            sum += line_sum * (self.height - r);
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tilt() {
        let input = [
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ];

        let expected = [
            "OOOO.#.O..",
            "OO..#....#",
            "OO..O##..O",
            "O..#.OO...",
            "........#.",
            "..#....#.#",
            "..O..#.O.O",
            "..O.......",
            "#....###..",
            "#....#....",
        ];
        let mut platform = Platform::new(
            input
                .iter()
                .map(|line| line.chars().map(Rock::from).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
        let expected_platform = Platform::new(
            expected
                .iter()
                .map(|line| line.chars().map(Rock::from).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
        platform.new_tilt(&Direction::North);
        assert_eq!(expected_platform, platform);
    }

    #[test]
    fn test_load() {
        let input = [
            "OOOO.#.O..",
            "OO..#....#",
            "OO..O##..O",
            "O..#.OO...",
            "........#.",
            "..#....#.#",
            "..O..#.O.O",
            "..O.......",
            "#....###..",
            "#....#....",
        ];

        let platform = Platform::new(
            input
                .iter()
                .map(|line| line.chars().map(Rock::from).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
        assert_eq!(136, platform.total_load());
    }

    #[test]
    fn test_cycle() {
        let input = [
            "OOOO.#.O..",
            "OO..#....#",
            "OO..O##..O",
            "O..#.OO...",
            "........#.",
            "..#....#.#",
            "..O..#.O.O",
            "..O.......",
            "#....###..",
            "#....#....",
        ];

        let expected = [
            ".....#....",
            "....#...O#",
            "...OO##...",
            ".OO#......",
            ".....OOO#.",
            ".O#...O#.#",
            "....O#....",
            "......OOOO",
            "#...O###..",
            "#..OO#....",
        ];

        let mut platform = Platform::new(
            input
                .iter()
                .map(|line| line.chars().map(Rock::from).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        let expected_platform = Platform::new(
            expected
                .iter()
                .map(|line| line.chars().map(Rock::from).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        platform.cycle(1);
        assert_eq!(expected_platform, platform);
    }

    #[test]
    fn test_tilt_west() {
        let input = [
            "OOOO.#.O..",
            "OO..#....#",
            "OO..O##..O",
            "O..#.OO...",
            "........#.",
            "..#....#.#",
            "..O..#.O.O",
            "..O.......",
            "#....###..",
            "#....#....",
        ];

        let expected = [
            "OOOO.#O...",
            "OO..#....#",
            "OOO..##O..",
            "O..#OO....",
            "........#.",
            "..#....#.#",
            "O....#OO..",
            "O.........",
            "#....###..",
            "#....#....",
        ];

        let mut platform = Platform::new(
            input
                .iter()
                .map(|line| line.chars().map(Rock::from).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        let expected_platform = Platform::new(
            expected
                .iter()
                .map(|line| line.chars().map(Rock::from).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        platform.new_tilt(&Direction::West);
        assert_eq!(expected_platform, platform);
    }

    #[test]
    fn test_tilt_south() {
        let input = [
            "OOOO.#.O..",
            "OO..#....#",
            "OO..O##..O",
            "O..#.OO...",
            "........#.",
            "..#....#.#",
            "..O..#.O.O",
            "..O.......",
            "#....###..",
            "#....#....",
        ];

        let expected = [
            ".....#....",
            "....#....#",
            "...O.##...",
            "...#......",
            "O.O....O#O",
            "O.#..O.#.#",
            "O....#....",
            "OO....OO..",
            "#OO..###..",
            "#OO.O#...O",
        ];

        let mut platform = Platform::new(
            input
                .iter()
                .map(|line| line.chars().map(Rock::from).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        let expected_platform = Platform::new(
            expected
                .iter()
                .map(|line| line.chars().map(Rock::from).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        platform.new_tilt(&Direction::South);
        assert_eq!(expected_platform, platform);
    }

}
