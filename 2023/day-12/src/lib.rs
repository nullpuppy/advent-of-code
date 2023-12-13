use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum Spring {
    Unknown, // ?
    Broken,  // #
    Working, // .
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '?' => Self::Unknown,
            '#' => Self::Broken,
            '.' => Self::Working,
            _ => unimplemented!("This should never occur"),
        }
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Spring::Unknown => '?',
                Spring::Broken => '#',
                Spring::Working => '.',
            }
        )
    }
}

pub struct SpringRecord {
    springs: Vec<Spring>,
    /// Groups of consecutive broken springs
    groups: Vec<usize>,
}

impl SpringRecord {
    pub fn new(input: String) -> Self {
        let split = input.split_once(' ').unwrap();

        Self {
            springs: split.0.chars().map(Spring::from).collect::<Vec<_>>(),
            groups: split
                .1
                .split_terminator(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        }
    }

    pub fn springs(&self) -> &Vec<Spring> {
        &self.springs
    }

    pub fn groups(&self) -> &Vec<usize> {
        &self.groups
    }

    pub fn possible_arrangement_count(&self) -> usize {
        let mut possibilites = vec![vec![]];

        // for every ?, add a . and a # as possibles.
        for spring in self.springs.iter() {
            match spring {
                Spring::Unknown => {
                    // duplicate the current possibilities.
                    // We have to do something like this because we can't hold multiple
                    // mutable refs around.
                    let len = possibilites.len();
                    let mut dupes = vec![];
                    for p in possibilites.iter() {
                        dupes.push(p.iter().copied().to_owned().collect());
                    }

                    possibilites.extend(dupes);
                    for (i, p) in possibilites.iter_mut().enumerate() {
                        if i < len {
                            p.push(Spring::Broken);
                        } else {
                            p.push(Spring::Working);
                        }
                    }
                },
                _ => {
                    for p in possibilites.iter_mut() {
                        p.push(*spring);
                    }
                }
            }
        }

        let mut sum = 0;
        for p in possibilites {
            let mut broken = 0;
            let mut groups = vec![];
            for s in &p {
                match s {
                    Spring::Broken => broken += 1,
                    _ => {
                        if broken > 0 {
                            groups.push(broken);
                            broken = 0;
                        }
                    },
                }
            }
            if broken > 0 {
                groups.push(broken);
            }
            if groups == self.groups {
                sum += 1;
            }
        }

        sum
    }

    // Part2
    /// "unfold" the springs and groups by a factor of factor, joining the groups with an unknown
    /// spring
    pub fn unfold(&mut self, factor: usize) {
        self.springs.push(Spring::Unknown);
        self.springs = self.springs.repeat(factor);
        self.springs.pop();

        self.groups = self.groups.repeat(factor);
    }
}

impl Display for SpringRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for spring in self.springs.iter() {
            write!(f, "{spring}")?;
        }
        write!(f, " ")?;

        write!(f, "{}", self.groups.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
    }
}
