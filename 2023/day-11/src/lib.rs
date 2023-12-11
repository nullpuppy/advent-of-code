/// Just a struct so I have a type I can impl some methods on.
/// Something like type Galaxy = (usize, usize) could probably work too.
#[derive(Debug)]
pub struct Galaxy {
    point: (usize, usize),
}

/// Parse input into a Vector of galaxies, and two arrays which are intended to
/// easily indicate if a column or row has any galaxies. This data can then be
/// used to age the galaxies by the amount desired
pub fn parse_input(input: impl Iterator<Item = String>) -> (Vec<Galaxy>, Vec<usize>, Vec<usize>) {
    let mut galaxies = vec![];
    let mut density_horiz = vec![];
    let mut density_vert = vec![];
    for (y, line) in input.enumerate() {
        if y == 0 {
            density_horiz.resize(line.len(), 0);
        }
        let mut has_galaxy = false;
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                has_galaxy = true;
                density_horiz[x] += 1;
                galaxies.push(Galaxy::new(x, y));
            }
        }
        if !has_galaxy {
            density_vert.push(0);
        } else {
            density_vert.push(1);
        }
    }

    (galaxies, density_horiz, density_vert)
}

/// Age every known galaxy based on it's position, and presence of empty space to the left or above
/// by the magnitude requested
pub fn age_galaxies(galaxies: &mut Vec<Galaxy>, density_horiz: &[usize], density_vert: &[usize], magnitude: usize) {
    for galaxy in galaxies {
        // need count of 0s to the left of
        // println!("Checking for 0s left of {}", galaxy.0);
        let horiz_exp: usize = density_horiz
            .iter()
            .copied()
            .enumerate()
            .filter(|(j, v)| *j < galaxy.x() && *v == 0)
            .map(|_| 1)
            .sum();
        let vert_exp: usize = density_vert
            .iter()
            .copied()
            .enumerate()
            .filter(|(j, v)| *j < galaxy.y() && *v == 0)
            .map(|_| 1)
            .sum();
        if horiz_exp > 0 || vert_exp > 0 {
            galaxy.age(horiz_exp, vert_exp, magnitude);
        }
    }
}

/// Sum the taxicab distance between every pair of known galaxies
pub fn sum_of_pairs(galaxies: &Vec<Galaxy>) -> usize {
    // Make pairs
    let mut pairs = vec![];
    for i in 0..galaxies.len()-1 {
        for j in i+1..galaxies.len() {
            pairs.push((&galaxies[i], &galaxies[j]));
        }
    }
    // Then sum distances between every pair
    pairs
        .iter()
        .map(|(p1, p2)| p1.taxicab_distance(p2))
        .sum()
}

impl Galaxy {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            point: (x, y),
        }
    }

    pub fn x(&self) -> usize {
        self.point.0
    }

    pub fn y(&self) -> usize {
        self.point.1
    }

    /// Compute taxicab distance between this Galaxy, and another.
    pub fn taxicab_distance(&self, other: &Galaxy) -> usize {
        self.point.0.abs_diff(other.point.0) + self.point.1.abs_diff(other.point.1)
    }

    /// Age the galaxy by position and magnitude.
    /// x/y can be scaled independently with multiple calls.
    pub fn age(&mut self, x: usize, y: usize, magnitude: usize) {
        self.point.0 += x * (magnitude -1);
        self.point.1 += y * (magnitude -1);
    }
}
