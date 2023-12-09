#![allow(dead_code)]
use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap_or_default())
    )
}

#[derive(Default)]
pub struct Game {
    id: usize,
    results: Vec<GameResult>,
    is_valid: bool,
}

pub struct GameRule {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Default, Debug, Clone)]
pub struct GameResult {
    red: usize,
    green: usize,
    blue: usize,
}

impl Game {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

impl GameRule {
    pub fn new(red: usize, green: usize, blue: usize) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }

    pub fn power(&self) -> usize {
        self.red * self.blue * self.green
    }

    pub fn process_game(&self, game_str: String) -> Game {
        let mut results = vec![];
        let mut is_valid = true;

        let mut iter = game_str.split_whitespace();
        iter.next(); // consume game
        // Parse game id.
        let id = iter.next().unwrap().replace(":", "").parse::<usize>().unwrap();

        let mut result = GameResult::default();
        let mut game_end = false;
        while let Some(next) = iter.next() {
            if let Ok(num) = next.parse::<usize>() {
                let mut color = iter.next().unwrap().replace(",", "");
                if color.ends_with(";") {
                    game_end = true;
                    color = color.to_string().replace(";", "");
                }
                match color.as_str() {
                    "red" => {
                        result.red = num;

                        if num > self.red {
                            is_valid = false;
                        };
                    },
                    "green" => {
                        result.green = num;
                        if num > self.green {
                            is_valid = false;
                        }
                    },
                    "blue" => {
                        result.blue = num;
                        if num > self.blue {
                            is_valid = false;
                        }
                    },
                    _ => {},
                };
            }
            if game_end {
                results.push(result.clone());
                result = GameResult::default();
                game_end = false;
            }
        }

        Game {
            id,
            results,
            is_valid,
            ..Game::default()
        }
    }
}
