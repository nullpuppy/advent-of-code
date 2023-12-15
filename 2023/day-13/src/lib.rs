use checked_clamp::CheckedClamp;
use std::fmt::{Debug, Display, Formatter};

pub struct AshAndRockPattern {
    pattern: Vec<Vec<Terrain>>,
    width: usize,
    height: usize,
}

#[derive(PartialEq, Eq)]
pub enum Terrain {
    Ash,
    Rock,
}

pub fn parse_input_to_patterns(input: impl Iterator<Item = String>) -> Vec<AshAndRockPattern> {
    let mut buffer = vec![];
    let mut patterns = vec![];
    for line in input {
        if line.is_empty() {
            patterns.push(AshAndRockPattern::new(buffer));
            buffer = vec![];
        } else {
            buffer.push(line.chars().map(Terrain::from).collect::<Vec<_>>());
        }
    }
    patterns.push(AshAndRockPattern::new(buffer));

    patterns
}

impl From<char> for Terrain {
    fn from(value: char) -> Self {
        match value {
            '.' => Terrain::Ash,
            '#' => Terrain::Rock,
            _ => unreachable!(),
        }
    }
}

impl Debug for Terrain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Terrain::Ash => '.',
                Terrain::Rock => '#',
            }
        )
    }
}

impl Display for AshAndRockPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                write!(f, "{:?}", self.pattern[r][c])?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl AshAndRockPattern {
    pub fn new(pattern: Vec<Vec<Terrain>>) -> Self {
        let width = pattern[0].len();
        let height = pattern.len();
        Self {
            pattern,
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

    /// Find the line where a reflection occurs, and return the lesser index where it occurs.
    /// For example, this pattern has a reflection that occurs between rows 4 ond 5, and thus will return 4.
    /// 1 #...##..# 1
    /// 2 #....#..# 2
    /// 3 ..##..### 3
    /// 4v#####.##.v4
    /// 5^#####.##.^5
    /// 6 ..##..### 6
    /// 7 #....#..# 7
    pub fn reflection_line(&self, with_smudge: bool) -> (Option<usize>, Option<usize>) {
        (self.row_reflection_line(with_smudge), self.column_reflection_line(with_smudge))
    }

    fn row_reflection_line(&self, with_smudge: bool) -> Option<usize> {
        let Some(mut start) = self.next_row_reflection(0, with_smudge) else {
            // We immediately didn't find any valid pairs of reflected lines
            return None;
        };

        // now compare lines until we've exhausted
        let mut offset = 1;
        loop {
            if start.checked_sub(offset).is_none()
                || (start + offset + 1).checked_clamp(0usize, self.height - 1)
                    != Ok(start + offset + 1)
            {
                break;
            }

            if !self.compare_rows(start-offset, start+offset+1, with_smudge) {
                // if self.pattern[start - offset] != self.pattern[start + 1 + offset] {
                // try again with self.next_row_reflection(start+1)
                // unless start = self.width-1
                if start >= self.height - 1 {
                    return None;
                }
                let Some(next) = self.next_row_reflection(start + 1, with_smudge) else {
                    return None;
                };
                start = next;
                offset = 0;
            }

            offset += 1;
        }

        // Line numbers start at 1, not 0.
        Some(start + 1)
    }

    fn next_row_reflection(&self, start: usize, with_smudge: bool) -> Option<usize> {
        // find pair of lines that are identical.
        // This will probably need to move to it's own method, we might need to do this multiple times.
        (start..self.height - 1)
            .find(|&r| r < self.height - 1 && self.compare_rows(r, r+1, with_smudge))
    }

    fn compare_rows(&self, a: usize, b: usize, with_smudge: bool) -> bool {
        if self.pattern[a] != self.pattern[b] {
            if with_smudge {
                let mut errors = 0;
                for i in 0..self.width {
                    if self.pattern[a][i] != self.pattern[b][i] {
                        if errors < 1 {
                            errors += 1;
                        } else {
                            return false;
                        }
                    }
                }
            } else {
                return false;
            }
        }
        true
    }

    fn column_reflection_line(&self, with_smudge: bool) -> Option<usize> {
        let Some(mut start) = self.next_col_reflection(0, with_smudge) else {
            // We immediately didn't find any valid pairs of reflected lines
            return None;
        };

        // now compare lines until we've exhausted
        let mut offset = 1;
        loop {
            // We've exceeded the width, one or the other direction, and haven't found an error.
            if start.checked_sub(offset).is_none()
                || (start + offset + 1).checked_clamp(0usize, self.width - 1)
                    != Ok(start + offset + 1)
            {
                break;
            }

            // Check columns
            if !self.compare_columns(start-offset, start+offset+1, with_smudge) {
                // We've found differences! return
                // try again with self.next_col_reflection(start+1)
                // unless start = self.width-1
                if start >= self.width - 1 {
                    return None;
                }
                let Some(next) = self.next_col_reflection(start + 1, with_smudge) else {
                    return None;
                };
                start = next;
                offset = 0;
            }

            offset += 1;
        }

        // Line numbers start at 1, not 0.
        Some(start + 1)
    }

    fn next_col_reflection(&self, start: usize, with_smudge: bool) -> Option<usize> {
        // find pair of lines that are identical.
        for c in start..self.width - 1 {
            if c < self.width - 1 {
                if !self.compare_columns(c, c+1, with_smudge) {
                    continue;
                }
                // If we get here, we have two columns that are identical.
                return Some(c);
            }
        }

        None
    }

    fn compare_columns(&self, a: usize, b: usize, with_smudge: bool) -> bool {
        // with smudge, allow 1 correction.
        let mut errors = 0;
        for r in 0..self.height {
            if self.pattern[r][a] != self.pattern[r][b] {
                // No match found, continue outer loop
                if with_smudge && errors < 1 {
                    errors += 1;
                } else {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_reflection_test() {
        let input = [
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
        ];

        let mut pattern = vec![];
        for line in input.iter() {
            pattern.push(line.chars().map(Terrain::from).collect());
        }
        let pattern = AshAndRockPattern::new(pattern);
        assert_eq!(Some(4), pattern.row_reflection_line(false));
    }

    #[test]
    fn col_reflection_no_match_test() {
        let input = [
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
        ];

        let mut pattern = vec![];
        for line in input.iter() {
            pattern.push(line.chars().map(Terrain::from).collect());
        }
        let pattern = AshAndRockPattern::new(pattern);
        assert_eq!(None, pattern.column_reflection_line(false));
    }

    #[test]
    fn col_reflection_match_test() {
        let input = [
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
        ];

        let mut pattern = vec![];
        for line in input.iter() {
            pattern.push(line.chars().map(Terrain::from).collect());
        }
        let pattern = AshAndRockPattern::new(pattern);
        assert_eq!(Some(5), pattern.column_reflection_line(false));
    }

    #[test]
    fn another_row_reflection_test() {
        let input = [
            ".##.#...#",
            "##.#.##..",
            "...#.....",
            "########.",
            "####..#.#",
            "###...#..",
            ".#..####.",
            "#..#..##.",
            "#####..#.",
            "#####..#.",
            "#..#..#..",
            "#..#..#..",
            "#####..#.",
            "#####..#.",
            "#..#..##.",
        ];

        let mut pattern = vec![];
        for line in input.iter() {
            pattern.push(line.chars().map(Terrain::from).collect());
        }
        let pattern = AshAndRockPattern::new(pattern);
        assert_eq!(None, pattern.column_reflection_line(false));
        assert_eq!(Some(11), pattern.row_reflection_line(false));
    }
}
