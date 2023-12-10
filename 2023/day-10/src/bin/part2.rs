#![allow(unused_imports)]
#![allow(dead_code)]

use day_10::{Coord2d, Dim2d, Direction, Grid, Tile};
use std::collections::HashSet;
use utils::read_lines;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let input = read_lines("input.txt").expect("Unable to open input");
    let ans = process(input);
    println!("Part 2: {}", ans);
}

fn process(input: impl Iterator<Item = String>) -> usize {
    let mut grid = Grid::from_input(input);
    grid.isolate_loop(Tile::Ground);
    grid.flood_fill(Tile::Void);
    let mut contained = 0;
    for row in grid.tiles() {
        for tile in row {
            if *tile == Tile::Ground {
                contained += 1;
            }
        }
    }

    grid.print_debug(Some(&Coord2d::new(5, 5)), Some(&Direction::East));
    contained
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_sample_four_test() {
        let input = read_lines("sample_four.txt").expect("Unable to open sample text");
        assert_eq!(4, process(input));
    }

    #[test]
    fn part_two_sample_five_test() {
        let input = read_lines("sample_five.txt").expect("Unable to open sample text");
        assert_eq!(8, process(input));
    }

    #[test]
    fn part_two_sample_six_test() {
        let input = read_lines("sample_six.txt").expect("Unable to open sample text");
        assert_eq!(10, process(input));
    }
}
